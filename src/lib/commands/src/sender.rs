//! Command senders.

use bevy_ecs::prelude::*;
use ferrumc_commands_api::CommandsAPI;
use ferrumc_text::TextComponent;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A possible command sender.
pub enum Sender {
    /// A player has sent a command.
    Player(Entity),

    /// The server console has sent a command.
    Server,
}

impl Sender {
    /// Sends the given `message` to this sender, and to the action bar
    /// if `actionbar` is true.
    pub fn send_message(&self, mut commands: CommandsAPI, message: TextComponent, actionbar: bool) {
        match self {
            Sender::Player(entity) => {
                if actionbar {
                    commands.send_actionbar(*entity, message);
                } else {
                    commands.send_to_player(*entity, message);
                }
            }
            Sender::Server => {
                commands.send_to_console(message);
            }
        }
    }
}
