use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_world::block_state_id::BlockStateId;

use crate::events::{
    BreakBlockRequest, PlaceBlockRequest, SendBlockChangeAckRequest, SendBlockUpdateRequest,
};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Plugin API for block operations
///
/// This is a system parameter that plugins use to interact with blocks
/// without knowing about the underlying network implementation.
///
/// # Example
///
/// ```rust,no_run
/// use bevy_ecs::prelude::*;
/// use ferrumc_block_api::BlockAPI;
///
/// fn my_system(mut blocks: BlockAPI) {
///     // Update a block and broadcast to all players
///     blocks.broadcast_block_update(position, block_state);
/// }
/// ```
#[derive(SystemParam)]
pub struct BlockAPI<'w> {
    update_events: EventWriter<'w, SendBlockUpdateRequest>,
    ack_events: EventWriter<'w, SendBlockChangeAckRequest>,
    place_events: EventWriter<'w, PlaceBlockRequest>,
    break_events: EventWriter<'w, BreakBlockRequest>,
}

impl<'w> BlockAPI<'w> {
    /// Broadcast a block update to all players
    pub fn broadcast_block_update(&mut self, position: NetworkPosition, block: BlockStateId) {
        self.update_events.write(SendBlockUpdateRequest {
            position,
            block,
            exclude_player: None,
        });
    }

    /// Broadcast a block update to all players except one
    pub fn broadcast_block_update_except(
        &mut self,
        position: NetworkPosition,
        block: BlockStateId,
        exclude_player: Entity,
    ) {
        self.update_events.write(SendBlockUpdateRequest {
            position,
            block,
            exclude_player: Some(exclude_player),
        });
    }

    /// Send a block change acknowledgment to a player
    pub fn send_ack(&mut self, player: Entity, sequence: VarInt) {
        self.ack_events.write(SendBlockChangeAckRequest {
            player,
            sequence,
        });
    }

    /// Request to place a block (core handles chunk I/O)
    ///
    /// This should be called after validation logic in plugins.
    /// Core systems will load the chunk, place the block, save the chunk,
    /// and broadcast the update.
    pub fn place_block(
        &mut self,
        player: Entity,
        position: NetworkPosition,
        block: BlockStateId,
        sequence: VarInt,
    ) {
        self.place_events.write(PlaceBlockRequest {
            player,
            position,
            block,
            sequence,
        });
    }

    /// Request to break a block (core handles chunk I/O)
    ///
    /// This should be called after validation logic in plugins.
    /// Core systems will load the chunk, break the block (set to air),
    /// save the chunk, and broadcast the update.
    pub fn break_block(&mut self, player: Entity, position: NetworkPosition, sequence: VarInt) {
        self.break_events.write(BreakBlockRequest {
            player,
            position,
            sequence,
        });
    }
}
