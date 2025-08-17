// flutter_rust_bridge bridge definitions for social layer
#![allow(unused)]

use sovr_core::*;

// ---------- Mirrors for FRB ----------
// (If your FRB config uses auto-mirroring, these type uses are enough.)

// ---------- Version ----------
pub fn api_version() -> String { version().to_string() }

// ---------- Profiles / Graph ----------
pub fn api_create_profile(p: Profile) -> anyhow::Result<String> { create_profile(p) }
pub fn api_get_profile(owner: KeyId) -> Option<Profile> { get_profile(owner) }
pub fn api_follow(target: KeyId, me: KeyId) -> anyhow::Result<()> { follow(target, me) }
pub fn api_unfollow(target: KeyId, me: KeyId) -> anyhow::Result<()> { unfollow(target, me) }

// ---------- Feed ----------
pub fn api_post_create(p: Post) -> anyhow::Result<DocId> { post_create(p) }
pub fn api_post_fetch_timeline(cursor: Option<String>, filter: FeedFilter) -> anyhow::Result<Vec<PostView>> {
    post_fetch_timeline(cursor, filter)
}
pub fn api_react(r: Reaction) -> anyhow::Result<()> { react(r) }

// ---------- Groups ----------
pub fn api_group_create(cfg: GroupConfig) -> anyhow::Result<GroupId> { group_create(cfg) }
pub fn api_group_join(id: GroupId) -> anyhow::Result<()> { group_join(&id) }

// ---------- Pages ----------
pub fn api_page_create(pg: Page) -> anyhow::Result<String> { page_create(pg) }
pub fn api_page_offer(owner: KeyId, offer: Offer) -> anyhow::Result<String> { page_offer(owner, offer) }

// ---------- Moderation ----------
pub fn api_moderation_mute(k: KeyId) -> anyhow::Result<()> { moderation_mute(k) }
pub fn api_moderation_block(k: KeyId) -> anyhow::Result<()> { moderation_block(k) }
