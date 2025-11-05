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

use bevy_ecs::prelude::*;
use ferrumc_inventories::defined_slots::player::HOTBAR_SLOT_6;
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::item::ItemID;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_inventory_api::{SetCreativeSlotEvent, SetHeldItemEvent};
use ferrumc_plugin_api::{register_events, Plugin, PluginContext};
use ferrumc_state::GlobalStateResource;
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

    fn build(&self, ctx: &mut PluginContext<'_>) {
        info!("Loading inventory plugin");

        // Register events from inventory API
        register_events!(ctx, SetCreativeSlotEvent, SetHeldItemEvent);

        // Register our gameplay logic systems
        ctx.add_tick_system(handle_creative_slot_changes);
        ctx.add_tick_system(handle_held_item_changes);

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
    state: Res<GlobalStateResource>,
    mut query: Query<&mut Inventory>,
) {
    for event in events.read() {
        debug!(
            "Handling creative slot change for player {:?}: slot_index={}, count={}",
            event.player, event.slot_index, event.slot.count.0
        );

        if !state.0.players.is_connected(event.player) {
            continue;
        }

        let Ok(mut inventory) = query.get_mut(event.player) else {
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

        // Display the updated inventory (example item in hotbar slot 6)
        if let Err(err) = inventory.set_item_with_update(
            HOTBAR_SLOT_6 as usize,
            InventorySlot {
                count: 1.into(),
                item_id: Some(ItemID::new(872)), // Example item ID for the creative mode slot
                components_to_add_count: None,
                components_to_remove_count: None,
                components_to_add: None,
                components_to_remove: None,
            },
            event.player,
        ) {
            error!(
                "Failed to update creative mode slot for player {:?}: {:?}",
                event.player, err
            );
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
    state: Res<GlobalStateResource>,
    mut query: Query<&mut Hotbar>,
) {
    for event in events.read() {
        debug!(
            "Handling held item change for player {:?}: slot_index={}",
            event.player, event.slot_index
        );

        if !state.0.players.is_connected(event.player) {
            error!("Player {:?} is not connected", event.player);
            continue;
        }

        // Validate slot index (0-8)
        if !(0..=8).contains(&event.slot_index) {
            error!(
                "Invalid slot index {} for player {:?}",
                event.slot_index, event.player
            );
            continue;
        }

        let Ok(mut hotbar) = query.get_mut(event.player) else {
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
