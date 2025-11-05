//! Player input packet handler
//!
//! Converts PlayerInputPacket into PlayerInputEvent
//! Only emits events when the sneak state changes to avoid flooding

use bevy_ecs::prelude::*;
use ferrumc_animation_api::PlayerInputEvent;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::PlayerInputPacketReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::collections::HashMap;

/// Core system: Converts PlayerInputPacket into PlayerInputEvent
/// Tracks sneak state per player and only emits events when it changes
pub fn handle_player_input_packets(
    packets: Res<PlayerInputPacketReceiver>,
    mut events: EventWriter<PlayerInputEvent>,
    query: Query<&PlayerIdentity>,
    mut state: Local<HashMap<Entity, bool>>,
) {
    for (packet, entity) in packets.0.try_iter() {
        let Ok(identity) = query.get(entity) else {
            continue;
        };
        
        let is_sneaking = packet.is_sneaking();
        let was_sneaking = state.get(&entity).copied().unwrap_or(false);
        
        // Only emit event when sneak state changes
        if is_sneaking != was_sneaking {
            events.write(PlayerInputEvent {
                player: entity,
                entity_id: VarInt(identity.short_uuid),
                flags: packet.flags,
                is_jumping: packet.is_jumping(),
                is_sneaking,
                is_sprinting: packet.is_sprinting(),
            });
            
            state.insert(entity, is_sneaking);
        }
    }
}
