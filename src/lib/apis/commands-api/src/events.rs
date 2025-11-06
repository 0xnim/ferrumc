//! Command response events

use bevy_ecs::prelude::*;
use ferrumc_text::TextComponent;

/// Event requesting a command response to be sent
#[derive(Event, Clone)]
pub struct SendCommandResponseEvent {
    /// The player to send to (None = console)
    pub receiver: Option<Entity>,
    /// The message to send
    pub message: TextComponent,
    /// Whether to send as actionbar
    pub actionbar: bool,
}
