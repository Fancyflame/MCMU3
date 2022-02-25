use chrono::{DateTime, Utc};
use mcmu_basic::{
    profile::{self, ProfileError},
    UserId,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, path::PathBuf, sync::atomic::AtomicU64};
use thiserror::Error as ThisError;

mod database;

use database::{Database, DatabaseError};

pub type Result<T> = std::result::Result<T, AccountError>;

lazy_static! {
    pub static ref GLOBAL_DB: Result<Database> = {
        (|| {
            let mut dbpath: PathBuf = profile::get_and_parse("server.databasePath")?.unwrap_or({
                let mut p = profile::STORAGE_PATH.clone();
                p.push("db");
                p
            });
            dbpath.push("mcmu_db");

            Ok(Database::new(dbpath).expect("Unable to start the database"))
        })()
    };
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub pwd_hash: [u8; 32],
    pub exclusive_title: Option<String>,
    pub is_administrator: bool,
    pub play_time: PlayTime,
    pub friends: HashSet<UserId>,
    //pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayTime {
    pub seconds_remaining: AtomicU64,
    pub rule_effective_until: Option<DateTime<Utc>>,
    pub rule: PlayTimeRule,
}

#[derive(Serialize, Deserialize)]
pub enum PlayTimeRule {
    None,
    Banned,
    Unlimited,
    FixedTimePerDay {
        fixed_seconds: u64,
        seconds_remaining_today: u64,
    },
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    Article { title: String, article: String },
    FriendInvitation(UserId),
    GroupInvitation(UserId),
}

#[derive(Debug, ThisError)]
pub enum AccountError {
    #[error("Account matches this id is exist")]
    AccountAlreadyExist,

    #[error("{0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("{0}")]
    ProfileError(#[from] ProfileError),
}

impl Account {
    #[inline]
    pub fn new(name: String, pwd_hash: [u8; 32]) -> Self {
        Account {
            name,
            pwd_hash,
            exclusive_title: None,
            is_administrator: false,
            friends: HashSet::new(),
            //messages: Vec::new(),
            play_time: PlayTime {
                seconds_remaining: AtomicU64::new(0),
                rule_effective_until: None,
                rule: PlayTimeRule::None,
            },
        }
    }

    pub fn set_admin(&mut self) {
        self.is_administrator = true;
        self.play_time.rule_effective_until = None;
        self.play_time.rule = PlayTimeRule::Unlimited;
    }
}
