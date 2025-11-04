//! Broadcaster: SendChatMessageRequest → SystemMessagePacket
//!
//! This is pure I/O layer - no game logic!

use bevy_ecs::prelude::*;
use ferrumc_chat_api::SendChatMessageRequest;
use ferrumc_net::{connection::StreamWriter, packets::outgoing::system_message::SystemMessagePacket};
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// Broadcast chat messages from plugins to the network
///
/// # Architecture
///
/// This system sits at the I/O boundary:
/// - Reads SendChatMessageRequest events from plugins
/// - Converts them into network packets
/// - Sends packets to the appropriate players
///
/// No game logic here - that belongs in the chat plugin!
pub fn broadcast_chat_messages(
    mut events: EventReader<SendChatMessageRequest>,
    query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        match event.receiver {
            Some(receiver) => {
                // Send to specific player
                let Ok((entity, writer)) = query.get(receiver) else {
                    continue;
                };

                if !state.0.players.is_connected(entity) {
                    continue;
                }

                if let Err(err) = writer.send_packet(SystemMessagePacket {
                    message: event.message.clone(),
                    overlay: event.overlay,
                }) {
                    error!("Failed to send chat message to player: {}", err);
                }
            }
            None => {
                // Broadcast to all players
                for (entity, writer) in query.iter() {
                    if !state.0.players.is_connected(entity) {
                        continue;
                    }

                    if let Err(err) = writer.send_packet(SystemMessagePacket {
                        message: event.message.clone(),
                        overlay: event.overlay,
                    }) {
                        error!("Failed to broadcast chat message: {}", err);
                    }
                }
            }
        }
    }
}
