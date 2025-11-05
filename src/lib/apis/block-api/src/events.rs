use bevy_ecs::prelude::*;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_world::block_state_id::BlockStateId;

use crate::types::{BlockFace, Hand};

/// High-level event: Player attempted to place a block
///
/// # Event Flow
///
/// 1. Network layer receives `PlaceBlock` packet
/// 2. Core emits `BlockPlaceAttemptEvent`
/// 3. Blocks plugin validates placement (collision, permissions, etc.)
/// 4. Plugin emits `BlockPlacedEvent` if successful
/// 5. Core broadcasts `BlockUpdate` and `BlockChangeAck` packets
#[derive(Event, Clone)]
pub struct BlockPlaceAttemptEvent {
    pub player: Entity,
    pub position: NetworkPosition,
    pub face: BlockFace,
    pub hand: Hand,
    pub sequence: VarInt,
}

/// High-level event: Player attempted to break a block
///
/// # Event Flow
///
/// 1. Network layer receives `PlayerAction` packet (status 0 = start breaking)
/// 2. Core emits `BlockBreakAttemptEvent`
/// 3. Blocks plugin validates break (permissions, tool, gamemode)
/// 4. Plugin emits `BlockBrokenEvent` if successful
/// 5. Core broadcasts `BlockUpdate` and `BlockChangeAck` packets
#[derive(Event, Clone)]
pub struct BlockBreakAttemptEvent {
    pub player: Entity,
    pub position: NetworkPosition,
    pub sequence: VarInt,
}

/// Event: A block was successfully placed by a player
#[derive(Event, Clone)]
pub struct BlockPlacedEvent {
    pub player: Entity,
    pub position: NetworkPosition,
    pub block: BlockStateId,
    pub sequence: VarInt,
}

/// Event: A block was successfully broken by a player
#[derive(Event, Clone)]
pub struct BlockBrokenEvent {
    pub player: Entity,
    pub position: NetworkPosition,
    pub sequence: VarInt,
}

/// Request to broadcast a block update to players
#[derive(Event, Clone)]
pub struct SendBlockUpdateRequest {
    pub position: NetworkPosition,
    pub block: BlockStateId,
    pub exclude_player: Option<Entity>,
}

/// Request to send block change acknowledgment to a player
#[derive(Event, Clone)]
pub struct SendBlockChangeAckRequest {
    pub player: Entity,
    pub sequence: VarInt,
}

/// Request to place a block in the world
///
/// This is emitted by plugins after validation.
/// Core systems handle the actual I/O (chunk loading, saving, broadcasting).
#[derive(Event, Clone)]
pub struct PlaceBlockRequest {
    pub player: Entity,
    pub position: NetworkPosition,
    pub block: BlockStateId,
    pub sequence: VarInt,
}

/// Request to break a block in the world
///
/// This is emitted by plugins after validation.
/// Core systems handle the actual I/O (chunk loading, saving, broadcasting).
#[derive(Event, Clone)]
pub struct BreakBlockRequest {
    pub player: Entity,
    pub position: NetworkPosition,
    pub sequence: VarInt,
}
