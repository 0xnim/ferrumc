//! Entity pose broadcasting
//!
//! Converts SetEntityPoseRequest events into EntityMetadata packets

use bevy_ecs::prelude::*;
use ferrumc_animation_api::{EntityPose, SetEntityPoseRequest};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_metadata::{EntityMetadata, EntityMetadataPacket};
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// Core system: Broadcasts entity pose changes as EntityMetadata packets
pub fn broadcast_pose_changes(
    mut requests: EventReader<SetEntityPoseRequest>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for request in requests.read() {
        let metadata = match request.pose {
            EntityPose::Standing => vec![EntityMetadata::entity_standing()],
            EntityPose::Sneaking => vec![
                EntityMetadata::entity_sneaking_visual(),
                EntityMetadata::entity_sneaking_pressed(),
            ],
            EntityPose::Sprinting => {
                // TODO: Implement sprinting metadata when available
                continue;
            }
            EntityPose::Swimming | EntityPose::Sleeping | EntityPose::FlyingWithElytra => {
                // TODO: Implement other poses when needed
                continue;
            }
        };

        let packet = EntityMetadataPacket::new(request.entity_id, metadata);

        for (entity, conn) in conn_query.iter() {
            // Skip excluded player (typically the one who triggered the pose change)
            if let Some(excluded) = request.exclude_player {
                if entity == excluded {
                    continue;
                }
            }

            if !state.0.players.is_connected(entity) {
                continue;
            }

            if let Err(e) = conn.send_packet_ref(&packet) {
                error!("Failed to send entity metadata packet: {}", e);
            }
        }
    }
}
