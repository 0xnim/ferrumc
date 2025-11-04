use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_text::TextComponent;

use crate::events::SendChatMessageRequest;

/// Plugin API for sending chat messages
///
/// This is a system parameter that plugins use to send chat messages
/// without knowing about the underlying network implementation.
///
/// # Example
///
/// ```rust,no_run
/// use bevy_ecs::prelude::*;
/// use ferrumc_chat_api::ChatAPI;
/// use ferrumc_text::TextComponent;
///
/// fn my_system(mut chat: ChatAPI) {
///     // Broadcast message to all players
///     chat.broadcast(TextComponent::from("Hello, world!"));
/// }
/// ```
#[derive(SystemParam)]
pub struct ChatAPI<'w> {
    events: EventWriter<'w, SendChatMessageRequest>,
}

impl<'w> ChatAPI<'w> {
    /// Send a message to a specific player
    ///
    /// # Arguments
    ///
    /// * `receiver` - The player entity to send to
    /// * `message` - The message to send
    pub fn send(&mut self, receiver: Entity, message: TextComponent) {
        self.events.write(SendChatMessageRequest {
            message,
            receiver: Some(receiver),
            overlay: false,
        });
    }

    /// Send an action bar message to a specific player
    ///
    /// # Arguments
    ///
    /// * `receiver` - The player entity to send to
    /// * `message` - The message to send
    pub fn send_actionbar(&mut self, receiver: Entity, message: TextComponent) {
        self.events.write(SendChatMessageRequest {
            message,
            receiver: Some(receiver),
            overlay: true,
        });
    }

    /// Broadcast a message to all players
    ///
    /// # Arguments
    ///
    /// * `message` - The message to broadcast
    pub fn broadcast(&mut self, message: TextComponent) {
        self.events.write(SendChatMessageRequest {
            message,
            receiver: None,
            overlay: false,
        });
    }

    /// Broadcast an action bar message to all players
    ///
    /// # Arguments
    ///
    /// * `message` - The message to broadcast
    pub fn broadcast_actionbar(&mut self, message: TextComponent) {
        self.events.write(SendChatMessageRequest {
            message,
            receiver: None,
            overlay: true,
        });
    }
}
