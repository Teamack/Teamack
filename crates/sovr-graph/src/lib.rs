use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type KeyId = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Relationship {
    Follow,
    Mute,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Follow {
    pub follower: KeyId,
    pub followee: KeyId,
    pub created_at_ms: i64,
    pub relationship: Relationship,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub owner: KeyId,
    pub name: String,
    pub members: Vec<KeyId>,
}

pub fn crate_version() -> &'static str { env!("CARGO_PKG_VERSION") }

static FOLLOWS: Lazy<RwLock<HashSet<(KeyId, KeyId)>>> = Lazy::new(|| RwLock::new(HashSet::new()));
static MUTES:   Lazy<RwLock<HashSet<KeyId>>>          = Lazy::new(|| RwLock::new(HashSet::new()));
static BLOCKS:  Lazy<RwLock<HashSet<KeyId>>>          = Lazy::new(|| RwLock::new(HashSet::new()));
static CIRCLES: Lazy<RwLock<HashMap<String, Circle>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn follow(follower: KeyId, followee: KeyId) {
    FOLLOWS.write().insert((follower, followee));
}
pub fn unfollow(follower: KeyId, followee: KeyId) {
    FOLLOWS.write().remove(&(follower, followee));
}
pub fn mute(target: KeyId)   { MUTES.write().insert(target); }
pub fn block(target: KeyId)  { BLOCKS.write().insert(target); }
pub fn is_muted(target: &KeyId) -> bool { MUTES.read().contains(target) }
pub fn is_blocked(target: &KeyId) -> bool { BLOCKS.read().contains(target) }

pub fn circle_upsert(circle: Circle) {
    let key = format!("{}:{}", hex(&circle.owner), circle.name);
    CIRCLES.write().insert(key, circle);
}

pub fn hex(k: &KeyId) -> String {
    k.iter().map(|b| format!("{:02x}", b)).collect()
}
