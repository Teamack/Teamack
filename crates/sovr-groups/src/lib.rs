use serde::{Deserialize, Serialize};
pub type KeyId = [u8; 32];
pub type GroupId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupMode { Open, Approval, Secret }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
    pub name: String,
    pub mode: GroupMode,
    pub about: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group { pub id: GroupId, pub cfg: GroupConfig }

pub fn crate_version() -> &'static str { env!("CARGO_PKG_VERSION") }
