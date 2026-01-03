//! Pick item from block events.
//!
//! Fired when a player attempts to pick an item using middle-click (pick block).

use bevy_ecs::prelude::{Entity, Message};
use ferrumc_inventories::item::ItemID;

/// Fired when a player picks an item that isn't in their inventory.
///
/// In creative mode, the item should be spawned into the player's hotbar.
/// In survival mode, nothing happens (no-op).
#[derive(Message)]
pub struct PickItemNotFound {
    /// The player entity
    pub player: Entity,
    /// The item ID to spawn
    pub item_id: ItemID,
    /// Whether the pick request included block data (NBT)
    pub include_data: bool,
}
