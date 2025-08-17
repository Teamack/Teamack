use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type KeyId = [u8; 32];
pub type DocId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Scope { Public, Circles, Direct, Secret }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visibility {
    pub scope: Scope,
    pub allowed: Vec<KeyId>, // for non-Public
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: DocId,
    pub author: KeyId,
    pub created_at_ms: i64,
    pub edited_at_ms: Option<i64>,
    pub visibility: Visibility,
    pub text_md: String,
    pub media: Vec<String>,   // media CIDs
    pub tags: Vec<String>,    // hashtags
    pub reply_to: Option<DocId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReactionKind { Like, Boost, Laugh, Wow, Sad, Angry }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub post_id: DocId,
    pub reactor: KeyId,
    pub kind: ReactionKind,
    pub created_at_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostView {
    pub post: Post,
    pub reactions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedFilter { Home, Latest, Friends, Groups, Pages }

pub fn crate_version() -> &'static str { env!("CARGO_PKG_VERSION") }

static POSTS: Lazy<RwLock<HashMap<DocId, Post>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn post_create(p: Post) -> DocId {
    let id = if p.id.is_empty() { new_id() } else { p.id.clone() };
    POSTS.write().insert(id.clone(), Post { id: id.clone(), ..p });
    id
}

pub fn post_get(id: &str) -> Option<Post> {
    POSTS.read().get(id).cloned()
}

pub fn post_fetch_timeline(_cursor: Option<String>, _filter: FeedFilter) -> Vec<PostView> {
    // MVP: chronological
    let mut v: Vec<Post> = POSTS.read().values().cloned().collect();
    v.sort_by_key(|p| p.created_at_ms);
    v.into_iter().rev().map(|p| PostView { post: p, reactions: 0 }).collect()
}

fn new_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let n = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    format!("post-{}", n)
}
