//! Command Response Broadcaster
//!
//! Handles I/O for command responses.
//! - Plugins send command responses via CommandsAPI
//! - This system reads those requests and delivers them (network I/O or console log)

use bevy_ecs::prelude::*;
use ferrumc_chat_api::ChatAPI;
use ferrumc_commands_api::SendCommandResponseEvent;
use tracing::info;

/// Broadcast command responses
///
/// This is PURE I/O - no game logic!
/// - Reads SendCommandResponseEvent from commands
/// - Delivers to players via ChatAPI or logs to console
pub fn broadcast_command_responses(
    mut events: EventReader<SendCommandResponseEvent>,
    mut chat: ChatAPI,
) {
    for event in events.read() {
        match event.receiver {
            Some(player) => {
                if event.actionbar {
                    chat.send_actionbar(player, event.message.clone());
                } else {
                    chat.send(player, event.message.clone());
                }
            }
            None => {
                // Console output
                info!("{}", event.message);
            }
        }
    }
}
