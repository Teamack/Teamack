use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub type KeyId = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockList { pub name: String, pub keys: Vec<KeyId> }

static MUTES:  Lazy<RwLock<HashSet<KeyId>>> = Lazy::new(|| RwLock::new(HashSet::new()));
static BLOCKS: Lazy<RwLock<HashSet<KeyId>>> = Lazy::new(|| RwLock::new(HashSet::new()));

pub fn crate_version() -> &'static str { env!("CARGO_PKG_VERSION") }

pub fn mute(k: KeyId)  { MUTES.write().insert(k); }
pub fn block(k: KeyId) { BLOCKS.write().insert(k); }

pub fn import_blocklist(list: BlockList) {
    let mut b = BLOCKS.write();
    for k in list.keys { b.insert(k); }
}
