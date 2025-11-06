//! Entity API Events

use bevy_ecs::prelude::*;
use ferrumc_text::TextComponent;

use crate::Target;

/// Request to send a system message to a specific player
///
/// Emitted by plugins, handled by core networking systems.
#[derive(Event)]
pub struct SendSystemMessageRequest {
    /// The player who should receive this message
    pub player: Entity,
    /// The message to send
    pub message: TextComponent,
}

/// Request to broadcast a system message to players
///
/// Emitted by plugins, handled by core networking systems.
#[derive(Event)]
pub struct BroadcastSystemMessageRequest {
    /// The message to broadcast
    pub message: TextComponent,
    /// Target recipients
    pub target: Target,
}
