use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_world::block_state_id::BlockStateId;

use crate::events::{
    BreakBlockRequest, PlaceBlockRequest, SendBlockChangeAckRequest, SendBlockUpdateRequest,
};
use ferrumc_net_codec::net_types::var_int::VarInt;

/// Plugin API for requesting block operations (Plugin → Core direction)
///
/// This SystemParam is used by plugins to request block operations.
/// Core systems handle the actual I/O (chunk loading/saving).
///
/// # Example
///
/// ```rust,no_run
/// use bevy_ecs::prelude::*;
/// use ferrumc_block_api::BlockRequests;
///
/// fn my_system(mut blocks: BlockRequests) {
///     // Request a block placement (core handles I/O)
///     blocks.place_block(player, position, block_state, sequence);
/// }
/// ```
#[derive(SystemParam)]
pub struct BlockRequests<'w> {
    place_events: EventWriter<'w, PlaceBlockRequest>,
    break_events: EventWriter<'w, BreakBlockRequest>,
}

impl<'w> BlockRequests<'w> {
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

/// Core API for broadcasting block changes (Core → Network direction)
///
/// This SystemParam is used by core systems to broadcast block updates
/// and send acknowledgments to players.
///
/// # Example
///
/// ```rust,no_run
/// use bevy_ecs::prelude::*;
/// use ferrumc_block_api::BlockBroadcasts;
///
/// fn my_system(mut blocks: BlockBroadcasts) {
///     // Broadcast a block update to all players
///     blocks.broadcast_block_update(position, block_state);
/// }
/// ```
#[derive(SystemParam)]
pub struct BlockBroadcasts<'w> {
    update_events: EventWriter<'w, SendBlockUpdateRequest>,
    ack_events: EventWriter<'w, SendBlockChangeAckRequest>,
}

impl<'w> BlockBroadcasts<'w> {
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
}

/// Backward compatibility alias
/// 
/// Plugins should use BlockRequests for requesting block operations.
pub type BlockAPI<'w> = BlockRequests<'w>;
