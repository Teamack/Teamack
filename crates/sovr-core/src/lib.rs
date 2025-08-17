//! Core façade for SOVR: re-export social layer and provide stable APIs.
use anyhow::Result;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;

pub use sovr_graph::{follow as graph_follow, unfollow as graph_unfollow, Circle, Follow, KeyId, Relationship};
pub use sovr_profile::{get_profile as profile_get, upsert_profile as profile_upsert, Link, Profile, ProfileKind};
pub use sovr_feed::{
    post_create as feed_post_create, post_fetch_timeline as feed_timeline, post_get as feed_post_get,
    DocId, FeedFilter, Post, PostView, Reaction, ReactionKind, Scope, Visibility,
};
pub use sovr_pages::{Money, Offer, Page, Role, RoleBinding};
pub use sovr_groups::{Group, GroupConfig, GroupId, GroupMode};
pub use sovr_moderation::{BlockList};

pub fn version() -> &'static str { env!("CARGO_PKG_VERSION") }

// --- In-memory doc store for quick MVP ---
static PAGES:  Lazy<RwLock<HashMap<String, Page>>> = Lazy::new(|| RwLock::new(HashMap::new()));
static GROUPS: Lazy<RwLock<HashMap<String, Group>>> = Lazy::new(|| RwLock::new(HashMap::new()));

// -------- Profiles / Graph ----------
pub fn create_profile(p: Profile) -> Result<String> {
    profile_upsert(p.clone());
    Ok("profile-ok".into())
}
pub fn get_profile(owner: KeyId) -> Option<Profile> { profile_get(&owner) }
pub fn follow(target: KeyId, me: KeyId) -> Result<()> { graph_follow(me, target); Ok(()) }
pub fn unfollow(target: KeyId, me: KeyId) -> Result<()> { graph_unfollow(me, target); Ok(()) }

// -------- Posts / Feed ----------
pub fn post_create(post: Post) -> Result<DocId> {
    Ok(feed_post_create(post))
}
pub fn post_fetch_timeline(cursor: Option<String>, filter: FeedFilter) -> Result<Vec<PostView>> {
    Ok(feed_timeline(cursor, filter))
}
pub fn react(_r: Reaction) -> Result<()> {
    // TODO: store reactions; OK for MVP
    Ok(())
}

// -------- Groups ----------
pub fn group_create(cfg: GroupConfig) -> Result<GroupId> {
    let id = format!("grp-{}", cfg.name.replace(' ', "_").to_lowercase());
    GROUPS.write().insert(id.clone(), Group { id: id.clone(), cfg });
    Ok(id)
}
pub fn group_join(_id: &GroupId) -> Result<()> { Ok(()) }

// -------- Pages ----------
pub fn page_create(pg: Page) -> Result<String> {
    let id = format!("page-{}", pg.handle);
    PAGES.write().insert(id.clone(), pg);
    Ok(id)
}
pub fn page_offer(_owner: KeyId, _offer: Offer) -> Result<String> {
    Ok("offer-ok".into())
}

// -------- Moderation ----------
pub fn moderation_mute(k: KeyId) -> Result<()> { sovr_moderation::mute(k); Ok(()) }
pub fn moderation_block(k: KeyId) -> Result<()> { sovr_moderation::block(k); Ok(()) }
pub fn import_blocklist(list: BlockList) -> Result<()> { sovr_moderation::import_blocklist(list); Ok(()) }
