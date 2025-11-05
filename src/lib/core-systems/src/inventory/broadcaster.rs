//! Inventory Broadcaster
//!
//! This module handles the I/O for inventory updates.
//! - Plugins send requests via InventoryAPI
//! - This system reads those requests and delivers them (network I/O)

use bevy_ecs::prelude::*;
use ferrumc_inventory_api::SendInventoryUpdateRequest;
use ferrumc_inventories::defined_slots::player::HEAD_SLOT;
use ferrumc_inventories::item::ItemID;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use std::sync::atomic::Ordering;
use tracing::{debug, error};

/// Send inventory updates to players
///
/// This is PURE I/O - no game logic!
/// - Reads SendInventoryUpdateRequest events from plugins
/// - Sends SetContainerSlot packets to players
pub fn send_inventory_updates(
    mut events: EventReader<SendInventoryUpdateRequest>,
    mut conn_query: Query<&StreamWriter>,
    state: Res<GlobalStateResource>,
) {
    for request in events.read() {
        if !state.0.players.is_connected(request.player) {
            error!("Player {} is not connected", request.player);
            continue;
        }

        let Ok(writer) = conn_query.get_mut(request.player) else {
            error!("Could not find writer for player {}", request.player);
            continue;
        };

        if !writer.running.load(Ordering::Relaxed) {
            continue;
        }

        // TODO: This hardcodes HEAD_SLOT and specific item values
        // This should be made configurable or removed
        let packet = ferrumc_net::packets::outgoing::set_container_slot::SetContainerSlot {
            window_id: VarInt::new(0),
            state_id: VarInt::new(0),
            slot_index: HEAD_SLOT as i16,
            slot: ferrumc_inventories::slot::InventorySlot {
                count: VarInt::new(65),
                item_id: Some(ItemID::new(872)),
                components_to_add_count: Some(VarInt::new(0)),
                components_to_remove_count: Some(VarInt::new(0)),
                components_to_add: None,
                components_to_remove: None,
            },
        };

        if let Err(e) = writer.send_packet_ref(&packet) {
            error!("Failed to send inventory update packet: {}", e);
        } else {
            debug!("Sent inventory update for player {}", request.player);
        }
    }
}
