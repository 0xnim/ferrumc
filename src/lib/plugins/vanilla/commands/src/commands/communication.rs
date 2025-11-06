//! Communication commands
//!
//! Implements:
//! - `/help` - Provides help for commands
//! - `/me <message>` - Displays a message about the sender

use bevy_ecs::prelude::*;
use ferrumc_commands::{arg::primitive::string::GreedyString, Sender};
use ferrumc_commands_api::CommandsAPI;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::command;
use ferrumc_text::{ComponentBuilder, NamedColor};

#[command("help")]
fn help_command(
    #[sender] sender: Sender,
    mut commands: CommandsAPI,
) {
    let help_text = ComponentBuilder::text("Available commands:")
        .color(NamedColor::Gold)
        .build();
    
    match sender {
        Sender::Player(entity) => {
            commands.send_to_player(entity, help_text);
            commands.send_to_player(entity, ComponentBuilder::text("  /help - Shows this help message").build());
            commands.send_to_player(entity, ComponentBuilder::text("  /me <message> - Displays a message about yourself").build());
            commands.send_to_player(entity, ComponentBuilder::text("  /echo <message> - Echoes a message").build());
            commands.send_to_player(entity, ComponentBuilder::text("  /nested - Test nested command").build());
        }
        Sender::Server => {
            commands.send_to_console(help_text);
            commands.send_to_console(ComponentBuilder::text("  /help - Shows this help message").build());
            commands.send_to_console(ComponentBuilder::text("  /me <message> - Displays a message about yourself").build());
        }
    }
}

#[command("me")]
fn me_command(
    #[arg] message: GreedyString,
    #[sender] sender: Sender,
    query: Query<&PlayerIdentity>,
    mut commands: CommandsAPI,
    mut chat: ferrumc_chat_api::ChatAPI,
) {
    let username = match sender {
        Sender::Server => "Server".to_string(),
        Sender::Player(entity) => query
            .get(entity)
            .map(|identity| identity.username.clone())
            .unwrap_or_else(|_| "Unknown".to_string()),
    };

    let me_message = ComponentBuilder::text(format!("* {} {}", username, message.clone()))
        .color(NamedColor::Yellow)
        .italic()
        .build();

    // Broadcast to all players
    chat.broadcast(me_message);
}
