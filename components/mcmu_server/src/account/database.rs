use super::Account;
use mcmu_basic::UserId;
use rocksdb::{DBWithThreadMode, SingleThreaded};
use serde::Serialize;
use std::path::Path;
use thiserror::Error as ThisError;
use tokio::sync::Mutex;

pub type Result<T> = std::result::Result<T, DatabaseError>;

pub struct Database(DBWithThreadMode<SingleThreaded>, Mutex<()>);

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

#[derive(Serialize, Clone)]
pub enum QueryType<'a> {
    Information(&'a UserId),
    Friends(&'a UserId),
    Messages(&'a UserId),
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
            Ok(db) => Ok(Database(db, Mutex::new(()))),
            Err(err) => Err(DatabaseError::OpenFailed(err)),
        }
    }

    pub fn get(&self, qt: QueryType) -> Result<Option<Account>> {
        Ok(
            match self
                .0
                .get_pinned(bincode::serialize(&qt).unwrap())
                .map_err(|err| DatabaseError::CoreError(err.into()))?
            {
                Some(d) => {
                    Some(bincode::deserialize(&*d).map_err(|_| DatabaseError::AccountDestroyed)?)
                }
                None => None,
            },
        )
    }

    pub fn set(&self, qt: QueryType, account: &Account) -> Result<()> {
        if !self.exists(qt.clone()) {
            return Err(DatabaseError::AccountNotExist);
        }
        self.set_unchecked(qt, account)
    }

    #[inline]
    fn set_unchecked(&self, qt: QueryType, account: &Account) -> Result<()> {
        self.0.put(
            bincode::serialize(&qt).unwrap(),
            bincode::serialize(account).unwrap(),
        )?;
        Ok(())
    }

    pub async fn create(&self, uid: &UserId, a: &Account) -> Result<()> {
        let _mutex = self.1.lock().await;
        if self.exists(QueryType::Information(uid)) {
            return Err(DatabaseError::AccountAlreadyExist);
        }
        self.set_unchecked(QueryType::Information(uid), &a)
        //self.set_unchecked(QueryType::Friends(), todo!())?;
        //self.set_unchecked(QueryType::Messages(), todo!())?;
    }

    pub fn remove(&self, uid: &UserId) -> Result<()> {
        if !self.exists(QueryType::Information(uid)) {
            return Err(DatabaseError::AccountNotExist);
        }

        self.0
            .delete(bincode::serialize(&QueryType::Information(uid)).unwrap())?;
        //self.0.delete(QueryType::Friends(uid))?;
        //self.0.delete(QueryType::Messages(uid))?;
        Ok(())
    }

    #[inline]
    pub fn exists(&self, qt: QueryType) -> bool {
        self.0.key_may_exist(bincode::serialize(&qt).unwrap())
    }
}
