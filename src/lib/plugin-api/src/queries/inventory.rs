//! Safe inventory queries for plugins
//!
//! Provides controlled access to inventory and hotbar data.

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::slot::InventorySlot as Slot;

/// Safe inventory queries for plugins
///
/// Provides read access to inventory and hotbar data.
/// For mutations, plugins should use InventoryAPI (when created).
///
/// # Example
///
/// ```rust,no_run
/// fn my_system(inventories: InventoryQueries) {
///     if let Some(slot) = inventories.get_slot(player, 0) {
///         println!("Slot 0: {:?}", slot);
///     }
///     
///     let selected = inventories.selected_slot(player);
/// }
/// ```
#[derive(SystemParam)]
pub struct InventoryQueries<'w, 's> {
    /// Query for inventories
    inventories: Query<'w, 's, &'static Inventory>,
    
    /// Query for hotbars
    hotbars: Query<'w, 's, &'static Hotbar>,
    
    /// Combined query
    both: Query<'w, 's, (&'static Inventory, &'static Hotbar)>,
}

impl<'w, 's> InventoryQueries<'w, 's> {
    /// Get player's inventory
    pub fn inventory(&self, entity: Entity) -> Option<&Inventory> {
        self.inventories.get(entity).ok()
    }
    
    /// Get player's hotbar
    pub fn hotbar(&self, entity: Entity) -> Option<&Hotbar> {
        self.hotbars.get(entity).ok()
    }
    
    /// Get both inventory and hotbar
    pub fn get(&self, entity: Entity) -> Option<(&Inventory, &Hotbar)> {
        self.both.get(entity).ok()
    }
    
    /// Get item in a specific slot
    pub fn get_slot(&self, entity: Entity, slot_index: usize) -> Option<Slot> {
        let inventory = self.inventory(entity)?;
        inventory.get_item(slot_index).ok()?.cloned()
    }
    
    /// Get selected hotbar slot index
    pub fn selected_slot(&self, entity: Entity) -> Option<u8> {
        let hotbar = self.hotbar(entity)?;
        Some(hotbar.selected_slot)
    }
    
    /// Get item in selected hotbar slot
    pub fn selected_item(&self, entity: Entity) -> Option<Slot> {
        let (inventory, hotbar) = self.get(entity)?;
        let slot_index = hotbar.selected_slot as usize;
        inventory.get_item(slot_index).ok()?.cloned()
    }
}

/// Mutable inventory queries for plugins
///
/// Provides write access to inventory and hotbar data.
/// Only available if plugin declares inventory_api capability.
#[derive(SystemParam)]
pub struct InventoryQueriesMut<'w, 's> {
    /// Combined query for both inventory and hotbar
    both: Query<'w, 's, (&'static mut Inventory, &'static mut Hotbar)>,
}

impl<'w, 's> InventoryQueriesMut<'w, 's> {
    /// Get mutable player's inventory and hotbar together
    ///
    /// Returns both components as a tuple. If you only need one,
    /// just ignore the other:
    ///
    /// ```rust,no_run
    /// if let Some((mut inv, _hotbar)) = inventories.get_mut(player) {
    ///     inv.set_item(0, slot);
    /// }
    /// ```
    pub fn get_mut(&mut self, entity: Entity) -> Option<(bevy_ecs::world::Mut<Inventory>, bevy_ecs::world::Mut<Hotbar>)> {
        self.both.get_mut(entity).ok()
    }
}
