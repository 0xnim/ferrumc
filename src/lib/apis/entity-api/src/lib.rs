//! Entity API for FerrumC
//!
//! This API provides safe messaging and broadcasting operations for entities.
//! 
//! # Architecture
//!
//! **Requests (Plugin → Core):**
//! - `SendSystemMessageRequest` - Send system message to specific player
//! - `BroadcastSystemMessageRequest` - Broadcast system message to all/some players
//!
//! # Example
//!
//! ```rust,no_run
//! fn my_system(mut entity_api: EntityAPI) {
//!     // Send to one player
//!     entity_api.send_system_message(player, "Hello!");
//!     
//!     // Broadcast to all
//!     entity_api.broadcast_system_message("Server message!", Target::All);
//!     
//!     // Broadcast except one player
//!     entity_api.broadcast_system_message("Message", Target::Except(player));
//! }
//! ```

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_core::transform::position::Position;
use ferrumc_text::TextComponent;

pub mod events;

pub use events::*;

/// Target for broadcasts
#[derive(Debug, Clone, Copy)]
pub enum Target {
    /// Send to all players
    All,
    /// Send to one specific player
    One(Entity),
    /// Send to all except this player
    Except(Entity),
    /// Send to all players within range of a position
    InRange { center: Position, range: f64 },
}

/// Entity API - SystemParam for plugins
///
/// Provides safe messaging operations without exposing network layer.
///
/// # Example
///
/// ```rust,no_run
/// fn notification_system(mut entity_api: EntityAPI) {
///     entity_api.broadcast_system_message("Event starting!", Target::All);
/// }
/// ```
#[derive(SystemParam)]
pub struct EntityAPI<'w> {
    message_requests: EventWriter<'w, SendSystemMessageRequest>,
    broadcast_requests: EventWriter<'w, BroadcastSystemMessageRequest>,
}

impl<'w> EntityAPI<'w> {
    /// Send a system message to a specific player
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// entity_api.send_system_message(player, "Welcome to the server!");
    /// ```
    pub fn send_system_message(&mut self, player: Entity, message: impl Into<TextComponent>) {
        self.message_requests.write(SendSystemMessageRequest {
            player,
            message: message.into(),
        });
    }
    
    /// Broadcast a system message to players based on target
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// // Send to all
    /// entity_api.broadcast_system_message("Server restarting", Target::All);
    /// 
    /// // Send to all except one player
    /// entity_api.broadcast_system_message("Player joined", Target::Except(joining_player));
    /// 
    /// // Send to players in range
    /// entity_api.broadcast_system_message(
    ///     "Explosion!",
    ///     Target::InRange { center: explosion_pos, range: 100.0 }
    /// );
    /// ```
    pub fn broadcast_system_message(&mut self, message: impl Into<TextComponent>, target: Target) {
        self.broadcast_requests.write(BroadcastSystemMessageRequest {
            message: message.into(),
            target,
        });
    }
}

/// Macro to register all Entity API events
///
/// # Example
///
/// ```rust,no_run
/// use ferrumc_entity_api::register_entity_api_events;
/// 
/// impl Plugin for MyPlugin {
///     fn build(&self, mut ctx: PluginBuildContext) {
///         register_entity_api_events!(ctx);
///     }
/// }
/// ```
#[macro_export]
macro_rules! register_entity_api_events {
    ($ctx:expr) => {
        $ctx.events()
            .register::<$crate::SendSystemMessageRequest>()
            .register::<$crate::BroadcastSystemMessageRequest>();
    };
}
