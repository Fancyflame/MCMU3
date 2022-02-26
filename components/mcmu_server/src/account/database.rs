use super::Account;
use mcmu_basic::UserId;
use rocksdb::{DBWithThreadMode, SingleThreaded};
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

macro_rules! impl_database {
    {$($StructName:ident),*} => {
        mod hidden_trait{
            use serde::{Deserialize,Serialize};
            use crate::account::{$($StructName),*};
            use mcmu_basic::UserId;

            #[derive(Serialize,Clone,Copy)]
            enum QueryCode{
                $($StructName),*
            }

            #[derive(Serialize, Clone)]
            struct QueryMsg<'a>(
                QueryCode, //编号
                &'a UserId,
            );

            pub trait Queryable:for<'de> Deserialize<'de>+Serialize{
                fn query_key(uid:&UserId)->Vec<u8>;
            }

            $(
                impl Queryable for $StructName{
                    #[inline]
                    fn query_key(uid:&UserId)->Vec<u8>{
                        bincode::serialize(&QueryMsg(QueryCode::$StructName,uid)).unwrap()
                    }
                }
            )*
        }

        use hidden_trait::Queryable;
    }
}

impl_database! {
    Account
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

    #[inline]
    fn set_unchecked<Q: Queryable>(&self, uid: &UserId, item: &Q) -> Result<()> {
        self.0
            .put(Q::query_key(uid), bincode::serialize(item).unwrap())?;
        Ok(())
    }

    pub async fn create(&self, uid: &UserId, a: &Account) -> Result<()> {
        let _mutex = self.1.lock().await;
        if self.exists(uid) {
            return Err(DatabaseError::AccountAlreadyExist);
        }
        self.set_unchecked(uid, a)
        //self.set_unchecked(QueryType::Friends(), todo!())?;
        //self.set_unchecked(QueryType::Messages(), todo!())?;
    }

    pub fn remove(&self, uid: &UserId) -> Result<()> {
        if !self.exists(uid) {
            return Err(DatabaseError::AccountNotExist);
        }

        self.0.delete(Account::query_key(uid))?;
        //self.0.delete(QueryType::Friends(uid))?;
        //self.0.delete(QueryType::Messages(uid))?;
        Ok(())
    }

    #[inline]
    pub fn exists(&self, uid: &UserId) -> bool {
        self.0.key_may_exist(Account::query_key(uid))
    }
}
