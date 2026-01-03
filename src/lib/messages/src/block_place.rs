//! Block placement events.
//!
//! Fired when a player successfully places a block.

use bevy_ecs::prelude::{Entity, Message};
use ferrumc_world::pos::BlockPos;

/// Fired when a player successfully places a block.
///
/// This event is used by survival mode to consume items from inventory.
/// Creative mode ignores this event.
#[derive(Message)]
pub struct BlockPlacedEvent {
    /// The player entity that placed the block
    pub player: Entity,
    /// The position where the block was placed
    pub position: BlockPos,
    /// The hotbar slot index used for placement
    pub slot_index: usize,
}
