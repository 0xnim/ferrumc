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
            // Sprinting is only a state bit (0x08), keeps Standing pose
            EntityPose::Sprinting => vec![
                EntityMetadata::entity_standing(),
                EntityMetadata::entity_sprinting(),
            ],
            EntityPose::Swimming => vec![
                EntityMetadata::entity_swimming_pose(),
                EntityMetadata::entity_swimming_state(),
            ],
            EntityPose::Sleeping => vec![EntityMetadata::entity_sleeping()],
            EntityPose::FlyingWithElytra => vec![
                EntityMetadata::entity_elytra_pose(),
                EntityMetadata::entity_elytra_state(),
            ],
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
