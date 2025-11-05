//! Packet → Event converters for inventory
//!
//! These systems receive network packets and convert them into
//! high-level domain events that plugins can listen to.

use bevy_ecs::prelude::*;
use ferrumc_inventory_api::{SetCreativeSlotEvent, SetHeldItemEvent};
use ferrumc_net::{SetCreativeModeSlotReceiver, SetHeldItemReceiver};
use ferrumc_state::GlobalStateResource;
use tracing::debug;

/// Convert SetCreativeModeSlot packets into SetCreativeSlotEvent events
///
/// This is pure I/O - no game logic here!
pub fn handle_set_creative_slot_packets(
    packet_events: Res<SetCreativeModeSlotReceiver>,
    state: Res<GlobalStateResource>,
    mut events: EventWriter<SetCreativeSlotEvent>,
) {
    for (packet, entity) in packet_events.0.try_iter() {
        if !state.0.players.is_connected(entity) {
            continue;
        }

        debug!(
            "Received SetCreativeModeSlot packet from player {:?}: slot_index={}, count={}",
            entity, packet.slot_index, packet.slot.count.0
        );

        // Convert packet → event
        events.write(SetCreativeSlotEvent {
            player: entity,
            slot_index: packet.slot_index,
            slot: packet.slot,
        });
    }
}

/// Convert SetHeldItem packets into SetHeldItemEvent events
///
/// This is pure I/O - no game logic here!
pub fn handle_set_held_item_packets(
    packet_events: Res<SetHeldItemReceiver>,
    state: Res<GlobalStateResource>,
    mut events: EventWriter<SetHeldItemEvent>,
) {
    for (packet, entity) in packet_events.0.try_iter() {
        if !state.0.players.is_connected(entity) {
            continue;
        }

        debug!(
            "Received SetHeldItem packet from player {:?}: slot_index={}",
            entity, packet.slot_index
        );

        // Convert packet → event
        events.write(SetHeldItemEvent {
            player: entity,
            slot_index: packet.slot_index,
        });
    }
}
