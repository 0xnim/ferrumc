//! Plugin API Prelude
//!
//! This module provides a safe subset of types that plugins can use.
//! 
//! **IMPORTANT:** Plugins should ONLY import from this prelude, not directly
//! from bevy_ecs or internal crates.
//!
//! # Usage
//!
//! ```rust
//! use ferrumc_plugin_api::prelude::*;
//! ```
//!
//! This gives you access to:
//! - Safe types (Entity)
//! - Event system (EventReader - for reading events)
//! - Domain APIs (AnimationAPI, BlockAPI, ChatAPI, EntityAPI, etc.)
//! - Safe queries (EntityQueries, WorldQueries)
//!
//! This does NOT give you:
//! - EventWriter (use domain APIs to send requests)
//! - Raw Query, Res, ResMut (use safe APIs instead)
//! - World, App (not accessible to plugins)
//! - Network types (StreamWriter, etc.)
//! - Database types
//! - EntityExt, WorldExt (direct I/O access - use domain APIs)

// Re-export safe ECS types only
pub use bevy_ecs::entity::Entity;
pub use bevy_ecs::event::EventReader; // Note: EventWriter NOT exported - use domain APIs!

// Re-export plugin API
pub use crate::build_context::{EventRegistry, PluginBuildContext, SystemRegistry};
pub use crate::capabilities::{PluginCapabilities, PluginCapabilitiesBuilder, ResourceCapability};
pub use crate::{Plugin, PluginConfig};

// Re-export common events from domain APIs for convenience
// (These are the authoritative versions - no duplicates!)
pub use ferrumc_join_leave_api::{PlayerJoinEvent, PlayerLeaveEvent};

// Re-export safe query APIs
pub use crate::queries::{EntityQueries, InventoryQueries, InventoryQueriesMut, WorldQueries};

// NOTE: Domain APIs are imported separately:
// use ferrumc_animation_api::AnimationAPI;
// use ferrumc_block_api::BlockRequests;
// use ferrumc_chat_api::ChatAPI;
// use ferrumc_entity_api::EntityAPI;
// etc.

// DO NOT re-export:
// - EventWriter (bypasses domain APIs)
// - Query, QueryState (use EntityQueries/WorldQueries instead)
// - Res, ResMut (use domain APIs instead)
// - World, App (not for plugins)
// - Commands (not for plugins)
// - SystemParam (plugins don't define custom params)
// - EntityExt, WorldExt (direct I/O access)
