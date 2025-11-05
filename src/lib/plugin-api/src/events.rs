//! Common event types for plugins
//!
//! This module contains generic player lifecycle events that don't belong
//! to a specific domain API. Domain-specific events are in their respective
//! API crates (block-api, chat-api, etc.)

use bevy_ecs::prelude::*;

/// Event emitted when a player joins the server
#[derive(Event, Debug, Clone)]
pub struct PlayerJoinEvent {
    /// The player entity
    pub player: Entity,
    /// Player's username
    pub username: String,
}

/// Event emitted when a player leaves the server
#[derive(Event, Debug, Clone)]
pub struct PlayerLeaveEvent {
    /// The player entity
    pub player: Entity,
    /// Player's username
    pub username: String,
    /// Optional disconnect reason
    pub reason: Option<String>,
}




