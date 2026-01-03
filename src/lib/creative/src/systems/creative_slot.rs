//! Creative mode slot handling.
//!
//! Handles the SetCreativeModeSlot packet which is sent when a player
//! in creative mode places an item in their inventory from the creative menu.

use bevy_ecs::prelude::{Query, Res};
use ferrumc_inventories::inventory::Inventory;
use ferrumc_net::SetCreativeModeSlotReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error};

/// Handle creative mode slot updates.
///
/// This system processes SetCreativeModeSlot packets from players in creative mode,
/// allowing them to place items directly into their inventory from the creative menu.
pub fn handle(
    receiver: Res<SetCreativeModeSlotReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<&mut Inventory>,
) {
    for (event, entity) in receiver.0.try_iter() {
        debug!(
            "Creative slot {} placed at {} by player {}",
            event.slot, event.slot_index, entity
        );

        if !state.0.players.is_connected(entity) {
            continue;
        }

        let Ok(mut inventory) = query.get_mut(entity) else {
            continue;
        };

        if event.slot.count.0 == 0 {
            // Clear the slot
            if let Err(e) = inventory.clear_slot_with_update(event.slot_index as usize, entity) {
                error!(
                    "Failed to clear slot {} for player {}: {:?}",
                    event.slot_index, entity, e
                );
            }
        } else {
            // Set the item
            if let Err(e) = inventory.set_item_with_update(event.slot_index as usize, event.slot, entity)
            {
                error!(
                    "Failed to set item in slot {} for player {}: {:?}",
                    event.slot_index, entity, e
                );
            }
        }
    }
}
