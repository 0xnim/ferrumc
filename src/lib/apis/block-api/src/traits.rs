use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_world::block_state_id::BlockStateId;

use crate::events::{
    BreakBlockRequest, PlaceBlockRequest, SendBlockChangeAckRequest, SendBlockUpdateRequest,
    BlockPlaceAttemptEvent, BlockBreakAttemptEvent, BlockPlacedEvent, BlockBrokenEvent,
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
///     // Read block placement attempts
///     for attempt in blocks.place_attempts() {
///         // Validate and request placement
///         blocks.place_block(attempt.player, attempt.position, block_state, attempt.sequence);
///     }
/// }
/// ```
#[derive(SystemParam)]
pub struct BlockRequests<'w, 's> {
    // Write requests
    place_events: EventWriter<'w, PlaceBlockRequest>,
    break_events: EventWriter<'w, BreakBlockRequest>,
    
    // Read input events
    place_attempt_reader: EventReader<'w, 's, BlockPlaceAttemptEvent>,
    break_attempt_reader: EventReader<'w, 's, BlockBreakAttemptEvent>,
    placed_reader: EventReader<'w, 's, BlockPlacedEvent>,
    broken_reader: EventReader<'w, 's, BlockBrokenEvent>,
}

impl<'w, 's> BlockRequests<'w, 's> {
    // ===== Read Methods (Input Events from Core) =====
    
    /// Read block placement attempts from players
    ///
    /// Returns an iterator over block placement attempts emitted by core.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// for attempt in blocks.place_attempts() {
    ///     if validate_placement(&attempt) {
    ///         blocks.place_block(attempt.player, attempt.position, block_state, attempt.sequence);
    ///     }
    /// }
    /// ```
    pub fn place_attempts(&mut self) -> impl Iterator<Item = &BlockPlaceAttemptEvent> + '_ {
    self.place_attempt_reader.read()
    }

    /// Read block break attempts from players
    ///
    /// Returns an iterator over block break attempts emitted by core.
    pub fn break_attempts(&mut self) -> impl Iterator<Item = &BlockBreakAttemptEvent> + '_ {
    self.break_attempt_reader.read()
    }

    /// Read successful block placements
    ///
    /// Returns an iterator over blocks that were successfully placed.
    pub fn placed_blocks(&mut self) -> impl Iterator<Item = &BlockPlacedEvent> + '_ {
    self.placed_reader.read()
    }

    /// Read successful block breaks
    ///
    /// Returns an iterator over blocks that were successfully broken.
    pub fn broken_blocks(&mut self) -> impl Iterator<Item = &BlockBrokenEvent> + '_ {
    self.broken_reader.read()
    }
    
    // ===== Write Methods (Requests to Core) =====
    
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
pub type BlockAPI<'w, 's> = BlockRequests<'w, 's>;
