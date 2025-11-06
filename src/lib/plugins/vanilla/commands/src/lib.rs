//! Vanilla Commands Plugin for FerrumC
//!
//! Implements vanilla Minecraft commands for Java Edition 1.21.8
//!
//! # Implemented Commands
//!
//! ## Communication (OP Level 0)
//! - `/list` - Lists players on the server
//! - `/help` - Provides help for commands
//! - `/me <message>` - Displays a message about the sender
//! - `/msg <player> <message>` (aliases: `/tell`, `/w`) - Private message
//! - `/teammsg <message>` (alias: `/tm`) - Team message
//!
//! ## Multiplayer (OP Level 2-4)
//! - `/kick <player> [reason]` - Kicks a player
//! - `/ban <player> [reason]` - Bans a player
//! - `/pardon <player>` - Unbans a player
//! - `/whitelist <add|remove|list|on|off>` - Manages whitelist
//! - `/op <player>` - Grants operator status
//! - `/deop <player>` - Revokes operator status
//!
//! ## Gameplay (OP Level 2)
//! - `/gamemode <mode> [player]` - Sets game mode
//! - `/teleport <destination>` (alias: `/tp`) - Teleports entities
//! - `/give <player> <item> [count]` - Gives items
//! - `/kill [target]` - Kills entities
//! - `/time <set|add|query> <value>` - Manages world time
//! - `/weather <clear|rain|thunder> [duration]` - Sets weather
//!
//! # Architecture
//!
//! This plugin implements vanilla commands using the command system.
//! All commands use ChatAPI for message output.
//!

use ferrumc_plugin_api::{Plugin, PluginBuildContext, PluginCapabilities};
use tracing::info;

mod commands;

#[derive(Default)]
pub struct VanillaCommandsPlugin;

impl Plugin for VanillaCommandsPlugin {
    fn name(&self) -> &'static str {
        "vanilla-commands"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Vanilla Minecraft commands for Java Edition 1.21.8"
    }

    fn priority(&self) -> i32 {
        10
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_chat_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        info!("Loading vanilla commands plugin");

        // Register command handlers
        commands::register(&mut ctx);

        info!("Vanilla commands plugin loaded successfully");
    }
}
