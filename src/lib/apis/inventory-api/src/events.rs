//! Inventory events
//!
//! These events represent inventory actions triggered by network packets.

use bevy_ecs::prelude::*;
use ferrumc_inventories::slot::InventorySlot;

/// Event fired when a player sets an item in their creative mode inventory
#[derive(Event, Debug, Clone)]
pub struct SetCreativeSlotEvent {
    /// The player who is setting the slot
    pub player: Entity,
    /// The slot index being modified
    pub slot_index: i16,
    /// The item being placed in the slot
    pub slot: InventorySlot,
}

/// Event fired when a player changes their selected hotbar slot
#[derive(Event, Debug, Clone)]
pub struct SetHeldItemEvent {
    /// The player who is changing their held item
    pub player: Entity,
    /// The hotbar slot index (0-8)
    pub slot_index: i16,
}
