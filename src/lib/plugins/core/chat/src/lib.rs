//! Chat Plugin for FerrumC
//!
//! This plugin implements the gameplay logic for chat messages.
//! It listens to high-level events (from the chat API) and decides
//! how to format and broadcast messages using the ChatAPI trait.
//!
//! # Architecture
//!
//! - Core converts packets → events
//! - This plugin reads events and applies game logic
//! - Plugin uses ChatAPI to broadcast messages
//! - Core converts chat requests → packets
//!
//! # Example Flow
//!
//! 1. Player types a message in game
//! 2. Network layer receives ChatMessagePacket
//! 3. Core emits ChatMessageEvent
//! 4. **This plugin** reads the event
//! 5. **This plugin** formats the message (adds username, etc.)
//! 6. **This plugin** calls chat_api.broadcast()
//! 7. Core receives SendChatMessageRequest
//! 8. Core broadcasts SystemMessagePacket to all players

use bevy_ecs::prelude::*;
use ferrumc_chat_api::{ChatAPI, ChatMessageEvent, SendChatMessageRequest};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_plugin_api::{register_events, Plugin, PluginContext};
use ferrumc_text::TextComponent;
use tracing::info;

#[derive(Default)]
pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn name(&self) -> &'static str {
        "chat"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Handles chat message formatting and broadcasting"
    }

    fn build(&self, ctx: &mut PluginContext<'_>) {
        info!("Loading chat plugin");

        // Register events from chat API
        register_events!(ctx, ChatMessageEvent, SendChatMessageRequest);

        // Register our gameplay logic systems
        ctx.add_tick_system(handle_chat_messages);

        info!("Chat plugin loaded successfully");
    }
}

/// Plugin system: Handle chat messages
///
/// This is pure game logic - when a player sends a chat message,
/// we format it with their username and broadcast it to all players.
///
/// # Game Logic
///
/// - Format: "<username> message"
/// - Broadcast to all players
/// - Future: Add chat filtering, permissions, channels, etc.
fn handle_chat_messages(
    mut events: EventReader<ChatMessageEvent>,
    mut chat: ChatAPI,
    query: Query<&PlayerIdentity>,
) {
    for event in events.read() {
        let Ok(identity) = query.get(event.player) else {
            continue;
        };

        // Format the message with the player's username
        let formatted = format!("<{}> {}", identity.username, event.message);

        // Broadcast to all players using the Chat API
        // Clean, high-level API - no knowledge of packets or network!
        chat.broadcast(TextComponent::from(formatted));
    }
}
