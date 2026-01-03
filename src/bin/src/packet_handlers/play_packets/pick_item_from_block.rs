//! Pick item from block handler.
//!
//! This handler processes middle-click (pick block) requests.
//! Common logic (finding items in inventory) is handled here.
//! Creative mode item spawning is delegated to ferrumc-creative mod via events.

use bevy_ecs::prelude::{Entity, MessageWriter, Query, Res};
use ferrumc_components::player::dimension::PlayerDimension;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_inventories::item::ItemID;
use ferrumc_inventories::{hotbar::Hotbar, inventory::Inventory};
use ferrumc_messages::PickItemNotFound;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_held_slot::SetHeldItem;
use ferrumc_state::GlobalStateResource;

use ferrumc_net::PickItemFromBlockReceiver;
use tracing::{debug, error, warn};

pub fn handle(
    events: Res<PickItemFromBlockReceiver>,
    state: Res<GlobalStateResource>,
    mut player_inv_query: Query<(
        Entity,
        &PlayerIdentity,
        &mut Inventory,
        &mut Hotbar,
        &StreamWriter,
        &PlayerDimension,
    )>,
    mut pick_not_found_events: MessageWriter<PickItemNotFound>,
) {
    for (packet, sender_entity) in events.0.try_iter() {
        // 1. Get player's components
        let Ok((entity, identity, mut inventory, mut hotbar, writer, dimension)) =
            player_inv_query.get_mut(sender_entity)
        else {
            warn!(
                "PickItemFromBlock: Player {:?} missing required components",
                sender_entity
            );
            continue;
        };

        debug!(
            "Player {} requested pick block at {:?} (Include Data: {})",
            identity.username, packet.location, packet.include_data,
        );

        // 2. Get block from world
        let pos = packet.location.clone().into();
        let block_state_id = match state.0.world.get_block_and_fetch(
            pos,
            dimension.as_str(),
        ) {
            Ok(id) => id,
            Err(e) => {
                warn!(
                    "PickItemFromBlock: Failed to get block at {:?}: {:?}",
                    packet.location, e
                );
                continue;
            }
        };

        // 3. Convert `BlockStateId` to `ItemId`
        let Some(item_id) = ItemID::from_block_state(block_state_id) else {
            debug!(
                "PickItemFromBlock: No item for block state {:?}",
                block_state_id
            );
            continue; // No item for this block (e.g., air)
        };

        debug!(
            "PickItemFromBlock: Block corresponds to ItemID: {:?}",
            item_id
        );

        // 4. Search the inventory for `ItemID`
        let found_slot_index = inventory.find_item(item_id);

        // 5a. Search hotbar first
        if let Some(hotbar_slot) = hotbar.find_item(&inventory, item_id) {
            // Item is in the hotbar. Check if we're already holding it.
            if hotbar.selected_slot == hotbar_slot {
                continue; // Do nothing
            }

            debug!(
                "Item found in hotbar slot {}. Switching held item.",
                hotbar_slot
            );

            // Update the server's state
            hotbar.selected_slot = hotbar_slot;

            // Send the packet to sync the client
            let packet = SetHeldItem { slot: hotbar_slot };
            if let Err(e) = writer.send_packet_ref(&packet) {
                error!("Failed to send SetHeldItem packet: {:?}", e);
            }
        }
        // 5b. Search rest of inventory
        else if let Some(inventory_slot_index) = found_slot_index {
            debug!(
                "Found item in slot {}. Swapping with hotbar slot {}.",
                inventory_slot_index, hotbar.selected_slot
            );

            // Check if the item is already in the selected hotbar slot
            if inventory_slot_index == hotbar.get_selected_inventory_index() {
                continue; // Nothing to do
            }

            if let Err(e) =
                hotbar.swap_with_inventory_slot(&mut inventory, inventory_slot_index, entity)
            {
                warn!("Failed to swap slots: {:?}", e);
            }
        }
        // 6. Item not found in inventory - fire event for mods to handle
        else {
            debug!(
                "Item {:?} not found in inventory. Firing PickItemNotFound event.",
                item_id
            );
            pick_not_found_events.write(PickItemNotFound {
                player: entity,
                item_id,
                include_data: packet.include_data,
            });
        }
    }
}
