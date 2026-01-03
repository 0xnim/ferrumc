//! Creative mode instant block breaking.
//!
//! In creative mode, blocks break instantly when the player starts digging (status 0).
//! The client expects the server to immediately update the block without waiting for
//! a "finished digging" packet.

use bevy_ecs::prelude::*;
use ferrumc_messages::player_digging::PlayerStartedDigging;
use ferrumc_messages::BlockBrokenEvent;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::BlockPos;
use tracing::{debug, error};

use crate::CreativeMode;
use ferrumc_components::player::dimension::PlayerDimension;

/// Handle instant block breaking for creative mode players.
///
/// This system intercepts `PlayerStartedDigging` events and immediately breaks
/// the block if the player is in creative mode, bypassing the normal digging timer.
pub fn handle_instant_break(
    mut events: MessageReader<PlayerStartedDigging>,
    state: Res<GlobalStateResource>,
    player_query: Query<(Entity, &StreamWriter, Option<&CreativeMode>, &PlayerDimension)>,
    broadcast_query: Query<(Entity, &StreamWriter)>,
    mut block_break_events: MessageWriter<BlockBrokenEvent>,
) {
    for event in events.read() {
        // Check if the player is in creative mode
        let Ok((entity, _writer, creative_mode, dimension)) = player_query.get(event.player) else {
            continue;
        };

        // Only handle creative mode players
        if creative_mode.is_none() {
            continue;
        }

        debug!(
            "Creative player {:?} instant-breaking block at {:?}",
            entity, event.position
        );

        let pos: BlockPos = event.position.clone().into();

        // Break the block in the world
        let Ok(mut chunk) = ferrumc_utils::world::load_or_generate_mut(
            &state.0,
            pos.chunk(),
            dimension.as_str(),
        ) else {
            error!("Failed to load or generate chunk for creative break");
            continue;
        };

        if let Err(e) = chunk.set_block(pos.chunk_block_pos(), BlockStateId::default()) {
            error!("Failed to set block in creative mode: {:?}", e);
            continue;
        }

        // Send block broken event for physics/un-grounding system
        block_break_events.write(BlockBrokenEvent { position: pos });

        // Broadcast the block update to all connected players
        let block_update_packet = BlockUpdate {
            location: event.position.clone(),
            block_state_id: VarInt::from(BlockStateId::default()),
        };

        for (eid, conn) in &broadcast_query {
            if !state.0.players.is_connected(eid) {
                continue;
            }

            if let Err(e) = conn.send_packet_ref(&block_update_packet) {
                error!("Failed to send block update to {:?}: {:?}", eid, e);
            }

            // Send ACK only to the player who broke the block
            if eid == event.player {
                let ack_packet = BlockChangeAck {
                    sequence: event.sequence,
                };
                if let Err(e) = conn.send_packet_ref(&ack_packet) {
                    error!("Failed to send block change ACK to {:?}: {:?}", eid, e);
                }
            }
        }
    }
}
