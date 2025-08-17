use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobRef { pub cid: String, pub bytes: usize }

pub fn crate_version() -> &'static str { env!("CARGO_PKG_VERSION") }

pub fn put_blob(_bytes: &[u8]) -> BlobRef {
    BlobRef { cid: "cid-placeholder".into(), bytes: 0 }
}

pub fn get_blob(_cid: &str) -> Option<Vec<u8>> { None }
