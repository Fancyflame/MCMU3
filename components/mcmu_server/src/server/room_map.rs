use mcmu_basic::{protocol::minecraft::OwnedUnconnectedPong, UserId};
use std::collections::{HashMap, HashSet};
use tokio::sync::watch;

pub type Updater = watch::Sender<Option<OwnedUnconnectedPong>>;

pub type Listener = watch::Receiver<Option<OwnedUnconnectedPong>>;

pub struct RoomMap {
    watch_pool: HashMap<UserId, PoolItem>,
}

struct PoolItem {
    have_permission: HashSet<UserId>,
    listener: Listener,
}

impl RoomMap {
    #[inline]
    pub fn new() -> Self {
        RoomMap {
            watch_pool: HashMap::new(),
        }
    }

    pub fn online(&mut self, hid: &UserId, friends: &[UserId]) -> Updater {
        let (tx, rx) = watch::channel(None);
        let pi = PoolItem {
            have_permission: HashSet::with_capacity(friends.len()),
            listener: rx,
        };
        assert!(self.watch_pool.insert(hid.clone(), pi).is_none());
        tx
    }

    pub fn offline(&mut self, hid: &UserId) {
        assert!(self.watch_pool.remove(hid).is_some());
    }

    pub fn try_listen(&self, hid: &UserId, self_id: &UserId) -> Option<Listener> {
        if let Some(host) = self.watch_pool.get(hid) {
            if host.have_permission.contains(self_id) {
                return Some(host.listener.clone());
            }
        }
        None
    }

    pub fn update_permission(&mut self, hid: &UserId, friends: &[UserId]) {
        let item = self
            .watch_pool
            .get_mut(hid)
            .expect("Here shouldn't be empty");
        for x in friends {
            if !item.have_permission.contains(x) {
                item.have_permission.insert(x.clone());
            }
        }
    }
}
