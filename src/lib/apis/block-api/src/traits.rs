use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_world::block_state_id::BlockStateId;

use crate::events::{SendBlockChangeAckRequest, SendBlockUpdateRequest};
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
}
