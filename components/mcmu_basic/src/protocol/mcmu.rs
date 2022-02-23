use std::{io::Error as IoError, ops::Deref};

use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[macro_export]
macro_rules! invalid_data {
    () => {{
        println!("[{} ({}:{})]: InvalidData", line!(), line!(), column!());
        Protocol::InvalidData
    }};
}

const MAX_PROTOCOL_SIZE: usize = 4 * 1024;
const VERSION: (u32, u32, u32) = (0, 0, 1);

#[derive(Serialize, Deserialize)]
pub struct Token(pub [u8; 16]);

//用于访问数据库（Deserialize因为需要检查字符串所以手动实现）
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Id([u8; 32]);

#[derive(Serialize, Deserialize)]
pub enum Protocol {
    Login(Login),
    Register(Register),
    FriendOperate(FriendOperate),
    RoomInfo(RoomInfo),
    InvalidData,
    InvalidId,
    Exit,
}

#[derive(Serialize, Deserialize)]
pub enum Login {
    LoginStart,
    LoginStartResult {
        salt: [u8; 8],
    },
    Login {
        id: Id,
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

#[derive(Serialize, Deserialize)]
pub enum Register {
    CheckForIdAvailable(Id),
    Register {
        id: Id,
        name: String,
        pwd_hash: [u8; 32],
    },
    CheckForIdAvailableResult(bool),
    RegisterSucceed,
    IdConflicted,
    RegisterForbidden,
    Failed(String),
}

#[derive(Serialize, Deserialize)]
pub enum RoomInfo {
    UpdateTo(Option<Vec<u8>>), //None为当前账户没有开放房间
    UpdateSucceed,
    DataTooLong,
    UpdateTooFrequent,
    Get(Id),
    Failed(String),
}

#[derive(Serialize, Deserialize)]
pub enum FriendOperate {
    Add(Id),
    Remove(Id),
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

impl Protocol {
    pub async fn write_to<W: AsyncWriteExt + Unpin>(
        &self,
        mut writer: W,
    ) -> Result<(), ProtocolError> {
        let data = bincode::serialize(self).unwrap();
        writer.write_all(&(data.len() as u32).to_be_bytes()).await?;
        writer.write_all(&data).await?;
        Ok(())
    }

    pub async fn read_from<R: AsyncReadExt + Unpin>(mut reader: R) -> Result<Self, ProtocolError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).await?;
        let len = u32::from_be_bytes(buf) as usize;
        let mut buf = [0u8; MAX_PROTOCOL_SIZE];

        if len > MAX_PROTOCOL_SIZE {
            return Err(ProtocolError::ProtocolTooLarge);
        }

        reader.read_exact(&mut buf[..len]).await?;
        bincode::deserialize(&buf[..len]).map_err(|_| ProtocolError::MalformedBody)
    }
}

impl Id {
    #[inline]
    pub fn new(id: &str) -> Self {
        Id(*blake3::hash(id.as_bytes()).as_bytes())
    }
}

// 下面是from实现

impl From<Register> for Protocol {
    #[inline]
    fn from(reg: Register) -> Self {
        Protocol::Register(reg)
    }
}

impl From<Login> for Protocol {
    #[inline]
    fn from(log: Login) -> Self {
        Protocol::Login(log)
    }
}

impl From<RoomInfo> for Protocol {
    #[inline]
    fn from(upd: RoomInfo) -> Self {
        Protocol::RoomInfo(upd)
    }
}

impl Deref for Id {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
