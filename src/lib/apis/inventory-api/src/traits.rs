//! Inventory API trait
//!
//! This provides the high-level interface for plugins to interact with inventories.

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_inventories::slot::InventorySlot;

use crate::events::SendInventoryUpdateRequest;

/// Inventory API - SystemParam for plugins
///
/// Provides methods to send inventory updates.
///
/// # Example
///
/// ```rust,no_run
/// fn my_system(mut api: InventoryAPI) {
///     api.send_inventory_update(player, slot_index, slot_data);
/// }
/// ```
#[derive(SystemParam)]
pub struct InventoryAPI<'w> {
    update_requests: EventWriter<'w, SendInventoryUpdateRequest>,
}

impl<'w> InventoryAPI<'w> {
    /// Send an inventory update to a player
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// api.send_inventory_update(player, 0, slot);
    /// ```
    pub fn send_inventory_update(
        &mut self,
        player: Entity,
        slot_index: i16,
        slot: InventorySlot,
    ) {
        self.update_requests.write(SendInventoryUpdateRequest {
            player,
            slot_index,
            slot,
        });
    }
}
