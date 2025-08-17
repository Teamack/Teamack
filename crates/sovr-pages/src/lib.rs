use serde::{Deserialize, Serialize};

pub type KeyId = [u8; 32];
pub type DocId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role { Admin, Editor, Mod }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleBinding { pub member: KeyId, pub role: Role }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geo { pub lat: f64, pub lon: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hours { pub open: String, pub close: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money { pub currency: String, pub amount: i64 } // cents

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
    pub page_owner: KeyId,
    pub title: String,
    pub body_md: String,
    pub price: Option<Money>,
    pub ledger_link: Option<String>,
    pub created_at_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub owner: KeyId,
    pub handle: String,
    pub display_name: String,
    pub roles: Vec<RoleBinding>,
    pub categories: Vec<String>,
    pub location: Option<Geo>,
    pub hours: Option<Hours>,
}

pub fn crate_version() -> &'static str { env!("CARGO_PKG_VERSION") }
