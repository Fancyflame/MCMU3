use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod data_structure;
pub use data_structure::*;

#[macro_export]
macro_rules! invalid_data {
    () => {{
        println!("[{} ({}:{})]: InvalidData", file!(), line!(), column!());
        Protocol::InvalidData
    }};
}

macro_rules! protocol_from {
    [$($id:ident),*] => {
        $(
            impl From<$id> for Protocol{
                #[inline]
                fn from(obj:$id)->Self{
                    Protocol::$id(obj)
                }
            }
        )*
    };
}

protocol_from![Login, Register, FriendManage, RoomInfo];

pub fn hash_pwd(bytes: &[u8], salt: Option<&[u8]>) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(bytes);
    if let Some(salt) = salt {
        hasher.update(salt);
    }
    *hasher.finalize().as_bytes()
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

impl UserId {
    #[inline]
    pub fn new(id: &str) -> Self {
        let h = blake3::hash(id.as_bytes());
        let mut b = [0u8; 16];
        b.copy_from_slice(&h.as_bytes()[..16]);
        UserId(b)
    }
}

impl AsRef<[u8]> for UserId {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
