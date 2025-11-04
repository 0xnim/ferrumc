//! Broadcasters: Block events → Network packets
//!
//! This is pure I/O layer - no game logic!

use bevy_ecs::prelude::*;
use ferrumc_block_api::{SendBlockChangeAckRequest, SendBlockUpdateRequest};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// Broadcast block updates to players
///
/// # Architecture
///
/// This system sits at the I/O boundary:
/// - Reads SendBlockUpdateRequest events from plugins
/// - Converts them into BlockUpdate packets
/// - Sends packets to appropriate players
///
/// No game logic here - that belongs in the blocks plugin!
pub fn broadcast_block_updates(
    mut events: EventReader<SendBlockUpdateRequest>,
    query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        let packet = BlockUpdate {
            location: event.position.clone(),
            block_state_id: VarInt::from(event.block),
        };

        for (entity, writer) in query.iter() {
            // Skip excluded player
            if let Some(excluded) = event.exclude_player {
                if entity == excluded {
                    continue;
                }
            }

            // Skip disconnected players
            if !state.0.players.is_connected(entity) {
                continue;
            }

            if let Err(err) = writer.send_packet_ref(&packet) {
                error!("Failed to send block update to player: {}", err);
            }
        }
    }
}

/// Send block change acknowledgments to players
///
/// # Architecture
///
/// This system sits at the I/O boundary:
/// - Reads SendBlockChangeAckRequest events from plugins
/// - Converts them into BlockChangeAck packets
/// - Sends packets to the appropriate player
///
/// No game logic here - that belongs in the blocks plugin!
pub fn send_block_change_acks(
    mut events: EventReader<SendBlockChangeAckRequest>,
    query: Query<&StreamWriter>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        // Skip disconnected players
        if !state.0.players.is_connected(event.player) {
            continue;
        }

        let Ok(writer) = query.get(event.player) else {
            continue;
        };

        let packet = BlockChangeAck {
            sequence: event.sequence,
        };

        if let Err(err) = writer.send_packet_ref(&packet) {
            error!(
                "Failed to send block change ack to player: {}",
                err
            );
        }
    }
}
