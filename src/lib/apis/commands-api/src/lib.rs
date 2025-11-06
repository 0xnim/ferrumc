//! Commands API for FerrumC
//!
//! Provides a clean interface for commands to send responses to players or console.
//! This API wraps the underlying chat system but provides command-specific ergonomics.
//!
//! # Example
//!
//! ```ignore
//! use ferrumc_commands_api::CommandsAPI;
//! use ferrumc_commands::Sender;
//!
//! fn my_command(mut commands: CommandsAPI, #[sender] sender: Sender) {
//!     commands.reply(sender, "Command executed!");
//! }
//! ```

use bevy_ecs::prelude::*;
use ferrumc_text::TextComponent;

pub mod events;

pub use events::*;

/// System parameter for sending command responses
#[derive(bevy_ecs::system::SystemParam)]
pub struct CommandsAPI<'w> {
    responses: EventWriter<'w, SendCommandResponseEvent>,
}

impl<'w> CommandsAPI<'w> {
    /// Send a response to a specific player
    pub fn send_to_player(&mut self, player: Entity, message: impl Into<TextComponent>) {
        self.responses.write(SendCommandResponseEvent {
            receiver: Some(player),
            message: message.into(),
            actionbar: false,
        });
    }

    /// Send a response to console (logged)
    pub fn send_to_console(&mut self, message: impl Into<TextComponent>) {
        self.responses.write(SendCommandResponseEvent {
            receiver: None,
            message: message.into(),
            actionbar: false,
        });
    }

    /// Send an actionbar message to a specific player
    pub fn send_actionbar(&mut self, player: Entity, message: impl Into<TextComponent>) {
        self.responses.write(SendCommandResponseEvent {
            receiver: Some(player),
            message: message.into(),
            actionbar: true,
        });
    }
}
