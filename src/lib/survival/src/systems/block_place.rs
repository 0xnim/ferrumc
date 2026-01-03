//! Block placement item consumption for survival mode.
//!
//! When a survival mode player places a block, this system consumes
//! the item from their inventory.

use bevy_ecs::prelude::{Entity, MessageReader, Query};
use ferrumc_creative::CreativeMode;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_messages::BlockPlacedEvent;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::{debug, error};

/// Handle block placement events for survival mode players.
///
/// This system consumes items from the player's inventory when they place blocks.
/// Creative mode players are ignored (they don't consume items).
pub fn handle_block_placement(
    mut events: MessageReader<BlockPlacedEvent>,
    mut query: Query<(Entity, &mut Inventory, Option<&CreativeMode>)>,
) {
    for event in events.read() {
        let Ok((entity, mut inventory, creative_mode)) = query.get_mut(event.player) else {
            continue;
        };

        // Creative mode players don't consume items
        if creative_mode.is_some() {
            debug!(
                "Player {:?} is in creative mode, not consuming item",
                entity
            );
            continue;
        }

        // Get the current item in the slot
        let Ok(Some(slot)) = inventory.get_item(event.slot_index) else {
            debug!("No item in slot {} to consume", event.slot_index);
            continue;
        };

        let new_count = slot.count.0 - 1;
        if new_count <= 0 {
            // Remove the item completely
            if let Err(e) = inventory.clear_slot_with_update(event.slot_index, entity) {
                error!("Failed to clear slot: {:?}", e);
            }
        } else {
            // Decrement the count
            let mut new_slot = slot.clone();
            new_slot.count = VarInt::new(new_count);
            if let Err(e) = inventory.set_item_with_update(event.slot_index, new_slot, entity) {
                error!("Failed to update slot: {:?}", e);
            }
        }
    }
}
