use chrono::{DateTime, Utc};
pub use mcmu_basic::protocol::mcmu::FriendList;
use mcmu_basic::{profile::ProfileError, UserId};
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicU64;
use thiserror::Error as ThisError;

pub mod database;

use database::DatabaseError;

pub type Result<T> = std::result::Result<T, AccountError>;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub pwd_hash: [u8; 32],
    pub exclusive_title: Option<String>,
    pub is_administrator: bool,
    pub play_time: PlayTime,
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
