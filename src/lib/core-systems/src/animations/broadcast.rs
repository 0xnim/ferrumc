//! Animation broadcasting
//!
//! Converts high-level animation requests into network packets.

use bevy_ecs::prelude::*;
use ferrumc_animation_api::PlayAnimationRequest;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_animation::EntityAnimationPacket;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// Core system: Broadcasts animation requests as network packets
pub fn broadcast_animations(
    mut requests: EventReader<PlayAnimationRequest>,
    query: Query<&PlayerIdentity>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    let count = requests.len();
    if count > 0 {
        tracing::debug!("Broadcasting {} animation requests", count);
    }
    for request in requests.read() {
        let Ok(identity) = query.get(request.entity) else {
            error!("Failed to get identity for entity {:?}", request.entity);
            continue;
        };

        let packet = EntityAnimationPacket::new(
            VarInt::new(identity.short_uuid),
            request.animation.id(),
        );

        for (entity, conn) in conn_query.iter() {
            // Skip excluded player (typically the one who triggered the animation)
            if let Some(excluded) = request.exclude_player {
                if entity == excluded {
                    continue;
                }
            }

            if !state.0.players.is_connected(entity) {
                continue;
            }

            if let Err(e) = conn.send_packet_ref(&packet) {
                error!("Failed to send animation packet: {}", e);
            }
        }
    }
}
