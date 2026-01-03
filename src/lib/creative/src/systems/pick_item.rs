//! Creative mode pick item handling.
//!
//! When a creative mode player middle-clicks on a block and doesn't have
//! the item in their inventory, this system spawns a new stack.

use bevy_ecs::prelude::{Entity, MessageReader, Query};
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_inventories::slot::InventorySlot;
use ferrumc_messages::PickItemNotFound;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_held_slot::SetHeldItem;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::{debug, error, warn};

use crate::CreativeMode;

/// Handle pick item events for creative mode players.
///
/// When a creative mode player picks an item that isn't in their inventory,
/// this system creates a new stack of that item in their hotbar.
pub fn handle_pick_item(
    mut events: MessageReader<PickItemNotFound>,
    mut query: Query<(
        Entity,
        &mut Inventory,
        &mut Hotbar,
        &StreamWriter,
        Option<&CreativeMode>,
    )>,
) {
    for event in events.read() {
        let Ok((entity, mut inventory, hotbar, writer, creative_mode)) =
            query.get_mut(event.player)
        else {
            continue;
        };

        // Only handle creative mode players
        if creative_mode.is_none() {
            debug!(
                "Player {:?} is not in creative mode, ignoring PickItemNotFound",
                entity
            );
            continue;
        }

        debug!(
            "Creative player {:?} picking item {:?}",
            entity, event.item_id
        );

        // Create a new item stack
        let new_slot = InventorySlot {
            item_id: Some(event.item_id),
            count: VarInt::new(1),
            ..Default::default()
        };

        // TODO: Handle NBT data when include_data is true
        if event.include_data {
            warn!("PickBlock: NBT data request (include_data=true) is not implemented yet.");
        }

        // Find an empty slot in the hotbar
        let Some(new_index) = hotbar.get_lowest_open_slot(&inventory) else {
            debug!("No empty hotbar slots for creative pick item");
            continue;
        };

        // Set the item in the hotbar
        if let Err(e) = hotbar.set_item_with_update(&mut inventory, new_index, new_slot, entity) {
            warn!("Failed to set creative item in hotbar: {:?}", e);
            continue;
        }

        // Switch to the new slot
        let packet = SetHeldItem { slot: new_index };
        if let Err(e) = writer.send_packet_ref(&packet) {
            error!("Failed to send SetHeldItem packet: {:?}", e);
        }
    }
}
