use anyhow::Result;
use chrono::Utc;
use rand::Rng;
use std::{collections::HashMap, net::SocketAddrV4, path::PathBuf, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{mpsc, Mutex, RwLock},
    task::JoinHandle,
};

use account::{database::Database, Account, FriendList, PlayTimeRule};
use mcmu_basic::{invalid_data, profile, protocol::mcmu::*, UserId};
use room_map::{RoomMap, Updater};

pub mod account;
mod room_map;

pub async fn run() -> Result<()> {
    //服务器使用的数据库
    let db = {
        let dbpath: PathBuf = profile::get_and_parse("server.databasePath")?.unwrap_or({
            let mut p = profile::STORAGE_PATH.clone();
            p.push("db");
            p
        });

        RwLock::new(Database::new(dbpath)?)
    };

    //服务器绑定地址
    let bind_addr = profile::get_and_parse("server.bindAddr")?
        .unwrap_or(SocketAddrV4::new([0, 0, 0, 0].into(), 27979));
    let room_map = RwLock::new(RoomMap::new());

    //打包
    let bundle = Arc::new((db, room_map));

    //启动服务器
    let srv = TcpListener::bind(&bind_addr).await?;
    println!("Server is running at {}", bind_addr);

    loop {
        let (mut stream, _) = srv.accept().await?;
        let bundle = bundle.clone();
        let _: JoinHandle<Result<()>> = tokio::spawn(async move {
            //发送数据包
            macro_rules! send {
                ($data:expr) => {{
                    let a: Protocol = $data.into();
                    a
                }
                .write_to(&mut stream)
                .await?};
            }

            //发送“无效数据”并中断会话
            macro_rules! send_invalid_data {
                () => {{
                    send!(invalid_data!());
                    return Ok(());
                }};
            }

            //获取一个数据包
            macro_rules! get_proto {
                () => {
                    match Protocol::read_from(&mut stream).await {
                        Ok(n) => n,
                        Err(ProtocolError::IoError(err))
                            if err.kind() == std::io::ErrorKind::UnexpectedEof =>
                        {
                            return Ok(())
                        }
                        Err(err) => return Err(err.into()),
                    }
                };
            }

            struct LoginInfo {
                id: UserId,
                account: Account,
                friend_list_snapshot: FriendList,
            }

            //---控制台---
            let (rwlock_db, room_map) = &*bundle;
            let mut login_info: Option<LoginInfo> = None;
            let mut room_info: Option<Updater> = None;

            //开始循环
            loop {
                let proto = get_proto!();
                let db = rwlock_db.read().await;
                match proto {
                    //无论登录与否都可以进行的操作

                    //检查该id是否可用
                    Protocol::Register(Register::CheckForIdAvailable(id)) => {
                        send!(Register::CheckForIdAvailableResult(db.exists(&id)))
                    }

                    //退出
                    Protocol::Exit => return Ok(()),

                    proto => match &mut login_info {
                        //已登录
                        Some(LoginInfo {
                            id,
                            account,
                            friend_list_snapshot,
                        }) => match proto {
                            //房间更新
                            Protocol::RoomInfo(ri) => match ri {
                                //开房间
                                RoomInfo::Open => match room_info {
                                    Some(_) => send!(RoomInfo::AlreadyOpened),
                                    None => {
                                        let updater =
                                            room_map.write().await.online(id, friend_list_snapshot);
                                        room_info = Some(updater);
                                        send!(RoomInfo::Succeed);
                                    }
                                },

                                //关房间
                                RoomInfo::Close => match room_info {
                                    Some(_) => {
                                        room_map.write().await.offline(id);
                                        room_info = None;
                                        send!(RoomInfo::Succeed);
                                    }
                                    None => send!(RoomInfo::NotOpened),
                                },

                                RoomInfo::UpdateTo(val) => {
                                    todo!()
                                }

                                _ => todo!(),
                            },

                            //好友管理
                            Protocol::FriendManage(fm) => match fm {
                                FriendManage::Fetch => {
                                    send!(FriendManage::Update(friend_list_snapshot.clone()));
                                }

                                FriendManage::Update(fls) => {
                                    if fls.len() > 30 {
                                        send!(FriendManage::TooMuchFriend);
                                    } else {
                                        *friend_list_snapshot = fls;
                                    }
                                }

                                _ => send_invalid_data!(),
                            },

                            _ => send_invalid_data!(),
                        },

                        //未登录
                        None => match proto {
                            //注册
                            Protocol::Register(Register::Register { id, name, pwd_hash }) => {
                                let account = Account::new(name, pwd_hash);
                                drop(db);
                                let mut db = rwlock_db.write().await;
                                db.create(&id, &account)?;
                                send!(Register::RegisterSucceed);

                                //创建好友列表快照
                                let fls: FriendList = db.get(&id)?.ok_or(anyhow!(
                                    "FriendList for user `{}` is not exist, it may be a bug",
                                    account.name
                                ))?;

                                login_info = Some(LoginInfo {
                                    id,
                                    account,
                                    friend_list_snapshot: fls,
                                });
                            }

                            //登录
                            Protocol::Login(Login::LoginStart(id)) => {
                                let mut account: Account = match db.get(&id)? {
                                    Some(n) => n,
                                    None => {
                                        send!(Login::AccountNotFound);
                                        return Ok(());
                                    }
                                };

                                let mut salt = [0u8; 8];
                                rand::thread_rng().fill(&mut salt); //产生随机盐值
                                send!(Login::LoginStartResult { salt }); //发送盐值
                                let correct_result = hash_pwd(&account.pwd_hash, Some(&salt)); //缓存正确结果

                                loop {
                                    match get_proto!() {
                                        //核验密码以及检查信息
                                        Protocol::Login(Login::Login { pwd_hash2 }) => {
                                            //如果密码不对
                                            if pwd_hash2 != correct_result {
                                                send!(Login::PasswordMismatched);
                                                continue;
                                            }

                                            //如果规则已过期则删除规则
                                            if let Some(ref date) =
                                                account.play_time.rule_effective_until
                                            {
                                                if &Utc::now() > date {
                                                    account.play_time.rule_effective_until = None;
                                                    account.play_time.rule = PlayTimeRule::None;
                                                    db.set(&id, &account)?;
                                                }
                                            }

                                            //用户被封禁
                                            if let PlayTimeRule::Banned = account.play_time.rule {
                                                send!(Login::AccountWasBanned);
                                                return Ok(());
                                            }

                                            send!(Login::LoginSucceed);
                                            login_info = Some(LoginInfo {
                                                friend_list_snapshot: db.get(&id)?.ok_or(
                                                anyhow!(
                                                    "FriendList for user `{}` is not exist, it may be a bug",
                                                    account.name
                                                ))?,
                                                id,
                                                account,
                                            });
                                            break;
                                        }

                                        _ => send_invalid_data!(),
                                    }
                                }
                            }

                            _ => send_invalid_data!(),
                        },
                    },
                }
            }
        });
    }
}
