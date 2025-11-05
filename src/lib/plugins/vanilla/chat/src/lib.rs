//! Vanilla Chat Plugin
//!
//! Implements vanilla Minecraft chat formatting and broadcasting.

use ferrumc_plugin_api::prelude::*;
use ferrumc_chat_api::{ChatAPI, ChatMessageEvent, SendChatMessageRequest};
use ferrumc_text::TextComponent;
use tracing::trace;

#[derive(Default)]
pub struct VanillaChatPlugin;

impl Plugin for VanillaChatPlugin {
    fn name(&self) -> &'static str {
        "vanilla-chat"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Vanilla Minecraft chat - formatting and broadcasting"
    }

    fn priority(&self) -> i32 {
        30
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_chat_api()
            .with_entity_queries()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        trace!("Loading vanilla-chat plugin");

        ctx.events()
            .register::<ChatMessageEvent>()
            .register::<SendChatMessageRequest>();

        ctx.systems()
            .add_tick(handle_chat_messages);

        trace!("Vanilla-chat plugin loaded successfully");
    }
}

fn handle_chat_messages(
    mut events: EventReader<ChatMessageEvent>,
    mut api: ChatAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        // Vanilla: Format as "<username> message"
        let formatted = TextComponent::from(format!(
            "<{}> {}",
            event.username, event.message
        ));
        
        // Broadcast to all players
        for (player, _, _) in entities.iter_players() {
            api.send_message(player, formatted.clone());
        }
        
        trace!("Broadcasted chat message from {}", event.username);
    }
}
