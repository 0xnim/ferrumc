//! Inventory Plugin for FerrumC
//!
//! This plugin implements the gameplay logic for inventory management.
//! It handles creative mode slot updates and hotbar selection.
//!
//! # Architecture
//!
//! - Core converts packets → events (SetCreativeSlotEvent, SetHeldItemEvent)
//! - This plugin reads events and applies game logic
//! - Plugin uses InventoryAPI to interact with inventory state

use ferrumc_plugin_api::prelude::*;
use ferrumc_inventory_api::{SetCreativeSlotEvent, SetHeldItemEvent};
use tracing::{debug, error, info};

#[derive(Default)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn name(&self) -> &'static str {
        "inventory"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Handles creative mode inventory and hotbar selection"
    }

    fn priority(&self) -> i32 {
        40 // Inventory management logic
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_inventory_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        info!("Loading inventory plugin");

        // Register events from inventory API
        ctx.events()
            .register::<SetCreativeSlotEvent>()
            .register::<SetHeldItemEvent>();

        // Register our gameplay logic systems
        ctx.systems()
            .add_tick(handle_creative_slot_changes)
            .add_tick(handle_held_item_changes);

        info!("Inventory plugin loaded successfully");
    }
}

/// Handle creative mode slot changes
///
/// This is pure game logic:
/// - Validates the slot change
/// - Updates the inventory
/// - Handles special cases (count=0 clears slot)
fn handle_creative_slot_changes(
    mut events: EventReader<SetCreativeSlotEvent>,
    mut inventories: InventoryQueriesMut,
) {
    for event in events.read() {
        debug!(
            "Handling creative slot change for player {:?}: slot_index={}, count={}",
            event.player, event.slot_index, event.slot.count.0
        );

        let Some((mut inventory, _hotbar)) = inventories.get_mut(event.player) else {
            error!("Could not find inventory for player {:?}", event.player);
            continue;
        };

        if event.slot.count.0 == 0 {
            // Clear the slot if the count is zero
            if let Err(e) = inventory.remove_item(event.slot_index as usize) {
                error!(
                    "Failed to clear slot {} for player {:?}: {:?}",
                    event.slot_index, event.player, e
                );
            }
        } else {
            // Set the item in the specified slot
            if let Err(e) = inventory.set_item(event.slot_index as usize, event.slot.clone()) {
                error!(
                    "Failed to set item in slot {} for player {:?}: {:?}",
                    event.slot_index, event.player, e
                );
            }
        }
    }
}

/// Handle hotbar slot selection changes
///
/// This is pure game logic:
/// - Validates the slot index (0-8)
/// - Updates the selected hotbar slot
fn handle_held_item_changes(
    mut events: EventReader<SetHeldItemEvent>,
    mut inventories: InventoryQueriesMut,
) {
    for event in events.read() {
        debug!(
            "Handling held item change for player {:?}: slot_index={}",
            event.player, event.slot_index
        );

        // Validate slot index (0-8)
        if !(0..=8).contains(&event.slot_index) {
            error!(
                "Invalid slot index {} for player {:?}",
                event.slot_index, event.player
            );
            continue;
        }

        let Some((_inventory, mut hotbar)) = inventories.get_mut(event.player) else {
            error!("Could not find hotbar for player {:?}", event.player);
            continue;
        };

        hotbar.selected_slot = event.slot_index as u8;
        debug!(
            "Set held item for player {:?} to slot {}",
            event.player, event.slot_index
        );
    }
}
