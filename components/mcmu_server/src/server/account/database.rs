use super::{Account, FriendList};
use mcmu_basic::UserId;
use rocksdb::{DBWithThreadMode, SingleThreaded};
use smallvec::SmallVec;
use std::path::Path;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, DatabaseError>;

pub struct Database(DBWithThreadMode<SingleThreaded>);

#[derive(Debug, ThisError)]
pub enum DatabaseError {
    #[error("Cannot open database: {0}")]
    OpenFailed(rocksdb::Error),

    #[error("ID `{0}` is illegal")]
    IllegalID(String),

    #[error("An error occurred from database core: {0}")]
    CoreError(#[from] rocksdb::Error),

    #[error("Account data has been destroyed!")]
    AccountDestroyed,

    #[error("The account is already exists, cannot create an account duplicatedly")]
    AccountAlreadyExist,

    #[error("Account matches this id is not found, please create one before this operation")]
    AccountNotExist,
}

mod hidden_trait {
    use mcmu_basic::UserId;
    use serde::{Deserialize, Serialize};

    const BUFFER_SIZE: usize = 34;

    pub trait Queryable: for<'de> Deserialize<'de> + Serialize {
        fn query_code() -> u16;
        fn query_key(uid: &UserId) -> [u8; BUFFER_SIZE] {
            let mut buf = [0u8; BUFFER_SIZE];
            bincode::serialize_into(&mut buf[..], &(Self::query_code(), uid)).unwrap();
            buf
        }
    }
}

use hidden_trait::Queryable;

macro_rules! impl_database {
    {$($StructType:ty => $code:literal),*} => {
        $(
            impl Queryable for $StructType{
                #[inline(always)]
                fn query_code()->u16{
                    $code
                }
            }
        )*

        impl Database{
            pub fn remove(&mut self, uid: &UserId) -> Result<()> {
                if !self.exists(uid) {
                    return Err(DatabaseError::AccountNotExist);
                }

                $(self.0.delete(&<$StructType>::query_key(uid))?;)*
                Ok(())
            }
        }
    }
}

//增加元素时不要忘记修改Database::create
impl_database! {
    Account => 1,
    FriendList => 2
}

impl Database {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        match DBWithThreadMode::<SingleThreaded>::open(
            &{
                let mut opt = rocksdb::Options::default();
                opt.create_if_missing(true);
                opt
            },
            path.as_ref(),
        ) {
            Ok(db) => Ok(Database(db)),
            Err(err) => Err(DatabaseError::OpenFailed(err)),
        }
    }

    pub fn get<Q: Queryable>(&self, uid: &UserId) -> Result<Option<Q>> {
        Ok(
            match self
                .0
                .get_pinned(Q::query_key(uid))
                .map_err(|err| DatabaseError::CoreError(err.into()))?
            {
                Some(d) => {
                    Some(bincode::deserialize(&*d).map_err(|_| DatabaseError::AccountDestroyed)?)
                }
                None => None,
            },
        )
    }

    pub fn set<Q: Queryable>(&self, uid: &UserId, item: &Q) -> Result<()> {
        if !self.exists(uid) {
            return Err(DatabaseError::AccountNotExist);
        }
        self.set_unchecked(uid, item)
    }

    pub fn create(&mut self, uid: &UserId, a: &Account) -> Result<()> {
        if self.exists(uid) {
            return Err(DatabaseError::AccountAlreadyExist);
        }

        self.set_unchecked(uid, a)?;
        self.set_unchecked(uid, &FriendList::new())?;
        Ok(())
    }

    #[inline]
    fn set_unchecked<Q: Queryable>(&self, uid: &UserId, item: &Q) -> Result<()> {
        let mut buf = SmallVec::<[u8; 512]>::new();
        bincode::serialize_into(&mut buf, item).unwrap();
        self.0.put(Q::query_key(uid), &buf)?;
        Ok(())
    }

    #[inline]
    pub fn exists(&self, uid: &UserId) -> bool {
        self.0.key_may_exist(Account::query_key(uid))
    }
}
