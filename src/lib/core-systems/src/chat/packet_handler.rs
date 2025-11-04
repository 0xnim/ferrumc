//! Packet handler: ChatMessagePacket → ChatMessageEvent
//!
//! This is pure I/O layer - no game logic!

use bevy_ecs::prelude::*;
use ferrumc_chat_api::ChatMessageEvent;
use ferrumc_net::ChatMessagePacketReceiver;

/// Convert incoming chat packets into high-level ChatMessageEvent
///
/// # Architecture
///
/// This system sits at the I/O boundary:
/// - Reads raw packets from the network layer
/// - Converts them into domain events
/// - Emits events for plugins to process
///
/// No game logic here - that belongs in the chat plugin!
pub fn handle_chat_packets(
    receiver: Res<ChatMessagePacketReceiver>,
    mut events: EventWriter<ChatMessageEvent>,
) {
    for (packet, sender) in receiver.0.try_iter() {
        events.write(ChatMessageEvent {
            player: sender,
            message: packet.message,
            timestamp: packet.timestamp,
        });
    }
}
