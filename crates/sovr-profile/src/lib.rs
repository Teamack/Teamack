use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type KeyId = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProfileKind { Person, Page }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link { pub label: String, pub url: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub owner: KeyId,
    pub handle: String,
    pub display_name: String,
    pub bio: String,
    pub avatar_cid: Option<String>,
    pub banner_cid: Option<String>,
    pub links: Vec<Link>,
    pub kind: ProfileKind,
}

pub fn crate_version() -> &'static str { env!("CARGO_PKG_VERSION") }

static PROFILES: Lazy<RwLock<HashMap<Vec<u8>, Profile>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn upsert_profile(p: Profile) {
    PROFILES.write().insert(p.owner.to_vec(), p);
}
pub fn get_profile(owner: &KeyId) -> Option<Profile> {
    PROFILES.read().get(&owner.to_vec()).cloned()
}
