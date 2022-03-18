use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::io::Error as IoError;
use thiserror::Error as ThisError;

pub const MAX_PROTOCOL_SIZE: usize = 4 * 1024;
pub type FriendList = SmallVec<[UserId; 30]>;
pub const VERSION: (u32, u32, u32) = (0, 0, 1);

#[derive(Serialize, Deserialize)]
pub struct Token(pub [u8; 16]);

//用于访问数据库（Deserialize因为需要检查字符串所以手动实现）
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(pub(crate) [u8; 16]);

//总协议
#[derive(Serialize, Deserialize)]
pub enum Protocol {
    Login(Login),
    Register(Register),
    AccountManage(),
    FriendManage(FriendManage),
    RoomInfo(RoomInfo),
    InvalidData,
    InvalidId,
    Exit,
}

//登录协议
#[derive(Serialize, Deserialize)]
pub enum Login {
    LoginStart(UserId),
    LoginStartResult {
        salt: [u8; 8],
    },
    Login {
        pwd_hash2: [u8; 32], //加盐值后的hash
    },
    LoginSucceed,

    PasswordMismatched,
    AccountNotFound,
    AlreadyLoggedIn,
    AccountWasBanned,
    ServerIsBusy,
    Failed(String),
}

//注册协议
#[derive(Serialize, Deserialize)]
pub enum Register {
    CheckForIdAvailable(UserId),
    Register {
        id: UserId,
        name: String,
        pwd_hash: [u8; 32],
    },
    CheckForIdAvailableResult(bool),
    RegisterSucceed,

    IdConflicted,
    RegisterForbidden,
    Failed(String),
}

//房间信息更新协议
#[derive(Serialize, Deserialize)]
pub enum RoomInfo {
    Get(UserId),
    Open,
    UpdateTo(Option<Vec<u8>>), //None为当前账户没有开放房间
    Succeed,
    Close,

    AlreadyOpened,
    NotOpened,
    DataTooLong,
    UpdateTooFrequent,
    Failed(String),
}

//账户资料管理协议
#[derive(Serialize, Deserialize)]
pub enum AccountMng {
    Fetch,
    Response,
    UpdateAccountInfo { name: String },
}

//好友管理协议
#[derive(Serialize, Deserialize)]
pub enum FriendManage {
    Fetch,
    Update(FriendList),
    UpdateSucceed,

    TooMuchFriend,
}

#[derive(Debug, ThisError)]
pub enum ProtocolError {
    #[error(
        "the version is mismatched, expected {}.{}.{}, but found {0}.{1}.{2}, please update to the newest version",
        VERSION.0, VERSION.1, VERSION.2
    )]
    VersionMismatched(u32, u32, u32),

    #[error("An IO error was occurred: {0}")]
    IoError(#[from] IoError),

    #[error("The body of the protocol may be destroyed")]
    MalformedBody,

    #[error(
        "The size of the protocol body is too large, it at most contains {} bytes",
        MAX_PROTOCOL_SIZE
    )]
    ProtocolTooLarge,
}
