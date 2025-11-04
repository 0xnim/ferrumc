//! Default Commands Plugin for FerrumC
//!
//! This plugin bridges the legacy command system to the new chat API.
//!
//! # How It Works
//!
//! 1. Commands are defined using `#[command]` macro (in ferrumc-default-commands crate)
//! 2. Commands call `sender.send_message()` which queues to `ferrumc_core::mq::QUEUE`
//! 3. This plugin drains that queue every tick
//! 4. Messages are converted to `SendChatMessageRequest` events
//! 5. The chat broadcaster (in core-systems) sends packets to players
//!
//! # Supported Commands
//!
//! - `/echo <message>` - Echoes a message back to the sender
//! - `/nested` - Test nested command
//! - `/nested nested` - Test double-nested command
//!
//! # Migration Path
//!
//! Future commands can be written as plugins that directly use `ChatAPI`,
//! avoiding the mq queue entirely. This bridge allows legacy commands to
//! continue working during the migration.

use ferrumc_chat_api::ChatAPI;
use ferrumc_plugin_api::Plugin;
use ferrumc_plugin_api::PluginContext;
use tracing::info;

#[derive(Default)]
pub struct DefaultCommandsPlugin;

impl Plugin for DefaultCommandsPlugin {
    fn name(&self) -> &'static str {
        "default-commands"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Bridges legacy command responses to the chat API"
    }

    fn build(&self, ctx: &mut PluginContext<'_>) {
        info!("Loading default commands plugin");

        // Register system to drain mq queue and convert to chat events
        ctx.add_tick_system(drain_legacy_mq_queue);

        info!("Default commands plugin loaded successfully");
    }
}

/// Drain the legacy mq queue and convert to chat API events
///
/// This bridges the old command system (which uses ferrumc_core::mq)
/// to the new chat API. Once all commands are migrated to use ChatAPI
/// directly, this system can be removed.
fn drain_legacy_mq_queue(mut chat: ChatAPI) {
    while !ferrumc_core::mq::QUEUE.is_empty() {
        let Some(entry) = ferrumc_core::mq::QUEUE.pop() else {
            break;
        };

        match entry.receiver {
            Some(receiver) => {
                if entry.overlay {
                    chat.send_actionbar(receiver, entry.message);
                } else {
                    chat.send(receiver, entry.message);
                }
            }
            None => {
                if entry.overlay {
                    chat.broadcast_actionbar(entry.message);
                } else {
                    chat.broadcast(entry.message);
                }
            }
        }
    }
}
