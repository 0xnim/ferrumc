//! Packet handlers: Block packets → Block events
//!
//! This is pure I/O layer - no game logic!

use bevy_ecs::prelude::*;
use ferrumc_block_api::{BlockBreakAttemptEvent, BlockFace, BlockPlaceAttemptEvent, Hand};
use ferrumc_net::{PlaceBlockReceiver, PlayerActionReceiver};

/// Convert incoming PlaceBlock packets into BlockPlaceAttemptEvent
///
/// # Architecture
///
/// This system sits at the I/O boundary:
/// - Reads raw PlaceBlock packets from the network layer
/// - Converts them into high-level BlockPlaceAttemptEvent
/// - Emits events for the blocks plugin to process
///
/// No game logic here - that belongs in the blocks plugin!
pub fn handle_place_block_packets(
    receiver: Res<PlaceBlockReceiver>,
    mut events: EventWriter<BlockPlaceAttemptEvent>,
) {
    for (packet, player) in receiver.0.try_iter() {
        let Some(face) = BlockFace::from_var_int(packet.face) else {
            continue;
        };
        let Some(hand) = Hand::from_var_int(packet.hand) else {
            continue;
        };

        events.write(BlockPlaceAttemptEvent {
            player,
            position: packet.position,
            face,
            hand,
            sequence: packet.sequence,
        });
    }
}

/// Convert incoming PlayerAction packets into BlockBreakAttemptEvent
///
/// # Architecture
///
/// This system sits at the I/O boundary:
/// - Reads raw PlayerAction packets from the network layer
/// - Filters for "start digging" (status 0)
/// - Converts them into high-level BlockBreakAttemptEvent
/// - Emits events for the blocks plugin to process
///
/// No game logic here - that belongs in the blocks plugin!
pub fn handle_player_action_packets(
    receiver: Res<PlayerActionReceiver>,
    mut events: EventWriter<BlockBreakAttemptEvent>,
) {
    for (packet, player) in receiver.0.try_iter() {
        // Status 0 = started digging (block break in creative)
        if packet.status.0 == 0 {
            events.write(BlockBreakAttemptEvent {
                player,
                position: packet.location,
                sequence: packet.sequence,
            });
        }
    }
}
