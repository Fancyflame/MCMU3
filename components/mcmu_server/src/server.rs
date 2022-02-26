use anyhow::Result;
use blake3::Hasher;
use chrono::Utc;
use rand::Rng;
use std::{collections::HashMap, net::SocketAddrV4, path::PathBuf, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{mpsc, Mutex, RwLock},
    task::JoinHandle,
};

use crate::account::{database::Database, Account, PlayTimeRule};
use mcmu_basic::{invalid_data, profile, protocol::mcmu::*, UserId};

pub struct RoomHost {
    info: Vec<u8>,
}

pub async fn run() -> Result<()> {
    let db = {
        let dbpath: PathBuf = profile::get_and_parse("server.databasePath")?.unwrap_or({
            let mut p = profile::STORAGE_PATH.clone();
            p.push("db");
            p
        });

        Database::new(dbpath)?
    };

    let bind_addr = profile::get_and_parse("server.bindAddr")?
        .unwrap_or(SocketAddrV4::new([0, 0, 0, 0].into(), 27979));
    let room_hosts = RwLock::new(HashMap::<UserId, RoomHost>::new());

    let bundle = Arc::new((db, room_hosts));

    let srv = TcpListener::bind(&bind_addr).await?;
    println!("Server is running at {}", bind_addr);

    loop {
        let (mut stream, _) = srv.accept().await?;
        let bundle = bundle.clone();
        let _: JoinHandle<Result<()>> = tokio::spawn(async move {
            macro_rules! send {
                ($data:expr) => {{
                    let a: Protocol = $data.into();
                    a
                }
                .write_to(&mut stream)
                .await?};
            }

            macro_rules! _invalid_data {
                () => {{
                    send!(invalid_data!());
                    return Ok(());
                }};
            }

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

            let (db, room_hosts) = &*bundle;

            //登录阶段
            let (id, mut account) = {
                loop {
                    match get_proto!() {
                        //注册
                        Protocol::Register(reg) => match reg {
                            //检查该id是否可用
                            Register::CheckForIdAvailable(id) => {
                                send!(Register::CheckForIdAvailableResult(db.exists(&id)))
                            }

                            //注册
                            Register::Register { id, name, pwd_hash } => {
                                let account = Account::new(name, pwd_hash);
                                db.create(&id, &account).await?;
                                send!(Register::RegisterSucceed);
                                break (id, account);
                            }

                            _ => _invalid_data!(),
                        },

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

                            break loop {
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
                                        break (id, account);
                                    }

                                    _ => _invalid_data!(),
                                }
                            };
                        }

                        Protocol::Exit => return Ok(()),

                        _ => _invalid_data!(),
                    }
                }
            };

            /*let mut w = room_hosts.write().await;
            //用户已登录
            if w.contains_key(&id) {
                send!(Login::AlreadyLoggedIn);
                return Ok(());
            }
            let account = Arc::new(RwLock::new(account));
            w.insert(id.clone(), account.clone());
            drop(w);*/

            //操作阶段
            loop {
                let proto = get_proto!();

                match proto {
                    Protocol::RoomInfo(ri) => match ri {
                        RoomInfo::UpdateTo(val) => {
                            todo!()
                        }

                        _ => todo!(),
                    },

                    Protocol::FriendOperate(fo) => match fo {
                        FriendOperate::Add(tid) => {
                            account.friends.insert(tid);
                            db.set(&id, &account)?;
                        }

                        FriendOperate::Remove(tid) => {
                            account.friends.remove(&tid);
                            db.set(&id, &account)?;
                        }
                    },

                    Protocol::Exit => return Ok(()),

                    _ => _invalid_data!(),
                }
            }
        });
    }
}
