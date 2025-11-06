//! Broadcaster: Entity API requests → Network Packets
//!
//! This is pure I/O layer - no game logic!

use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_entity_api::{BroadcastSystemMessageRequest, SendSystemMessageRequest, Target};
use ferrumc_net::{connection::StreamWriter, packets::outgoing::system_message::SystemMessagePacket};
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// Send system messages to specific players
///
/// # Architecture
///
/// This system sits at the I/O boundary:
/// - Reads SendSystemMessageRequest events from plugins
/// - Converts them into network packets
/// - Sends packets to the target player
///
/// No game logic here - that belongs in plugins!
pub fn send_system_messages(
    mut events: EventReader<SendSystemMessageRequest>,
    query: Query<&StreamWriter>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        let Ok(writer) = query.get(event.player) else {
            continue;
        };

        if !state.0.players.is_connected(event.player) {
            continue;
        }

        if let Err(err) = writer.send_packet(SystemMessagePacket {
            message: event.message.clone(),
            overlay: false,
        }) {
            error!("Failed to send system message to player: {}", err);
        }
    }
}

/// Broadcast system messages to players based on target
///
/// # Architecture
///
/// This system sits at the I/O boundary:
/// - Reads BroadcastSystemMessageRequest events from plugins
/// - Converts them into network packets
/// - Sends packets to the appropriate players based on Target
///
/// No game logic here - that belongs in plugins!
pub fn broadcast_system_messages(
    mut events: EventReader<BroadcastSystemMessageRequest>,
    writers: Query<(Entity, &StreamWriter)>,
    positions: Query<&Position>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        match event.target {
            Target::All => {
                // Send to all players
                for (entity, writer) in writers.iter() {
                    if !state.0.players.is_connected(entity) {
                        continue;
                    }

                    if let Err(err) = writer.send_packet(SystemMessagePacket {
                        message: event.message.clone(),
                        overlay: false,
                    }) {
                        error!("Failed to broadcast system message: {}", err);
                    }
                }
            }
            Target::One(target) => {
                // Send to specific player
                let Ok((entity, writer)) = writers.get(target) else {
                    continue;
                };

                if !state.0.players.is_connected(entity) {
                    continue;
                }

                if let Err(err) = writer.send_packet(SystemMessagePacket {
                    message: event.message.clone(),
                    overlay: false,
                }) {
                    error!("Failed to send system message to player: {}", err);
                }
            }
            Target::Except(excluded) => {
                // Send to all except one
                for (entity, writer) in writers.iter() {
                    if entity == excluded {
                        continue;
                    }

                    if !state.0.players.is_connected(entity) {
                        continue;
                    }

                    if let Err(err) = writer.send_packet(SystemMessagePacket {
                        message: event.message.clone(),
                        overlay: false,
                    }) {
                        error!("Failed to broadcast system message: {}", err);
                    }
                }
            }
            Target::InRange { center, range } => {
                // Send to players in range
                let range_sq = range * range;

                for (entity, writer) in writers.iter() {
                    if !state.0.players.is_connected(entity) {
                        continue;
                    }

                    let Ok(pos) = positions.get(entity) else {
                        continue;
                    };

                    let dx = pos.x - center.x;
                    let dy = pos.y - center.y;
                    let dz = pos.z - center.z;
                    let dist_sq = dx * dx + dy * dy + dz * dz;

                    if dist_sq <= range_sq {
                        if let Err(err) = writer.send_packet(SystemMessagePacket {
                            message: event.message.clone(),
                            overlay: false,
                        }) {
                            error!("Failed to send system message to player: {}", err);
                        }
                    }
                }
            }
        }
    }
}
