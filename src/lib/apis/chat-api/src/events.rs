use bevy_ecs::prelude::*;
use ferrumc_text::TextComponent;

/// High-level event: Player sent a chat message
///
/// # Event Flow
///
/// 1. Network layer receives `ChatMessagePacket`
/// 2. Core emits `ChatMessageEvent`
/// 3. Chat plugin processes message (formatting, filtering)
/// 4. Plugin emits `SendChatMessageRequest`
/// 5. Core broadcasts `SystemMessagePacket` to clients
#[derive(Event, Clone)]
pub struct ChatMessageEvent {
    pub player: Entity,
    pub message: String,
    pub timestamp: u64,
}

/// Request to send a chat message to player(s)
#[derive(Event, Clone)]
pub struct SendChatMessageRequest {
    pub message: TextComponent,
    pub receiver: Option<Entity>,
    pub overlay: bool,
}
