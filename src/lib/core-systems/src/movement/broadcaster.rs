//! Movement Broadcaster
//!
//! This module handles the I/O for movement updates.
//! - Plugins validate movement and send requests via MovementAPI
//! - This system reads those requests and delivers them (network I/O + ECS updates)

use bevy_ecs::prelude::*;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_macros::NetEncode;
use ferrumc_movement_api::{
    ApplyMovementRequest, BroadcastHeadRotationRequest, BroadcastMovementRequest,
    MovementBroadcastType,
};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::entity_position_sync::TeleportEntityPacket;
use ferrumc_net::packets::outgoing::set_head_rotation::SetHeadRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_position::UpdateEntityPositionPacket;
use ferrumc_net::packets::outgoing::update_entity_position_and_rotation::UpdateEntityPositionAndRotationPacket;
use ferrumc_net::packets::outgoing::update_entity_rotation::UpdateEntityRotationPacket;
use ferrumc_net_codec::net_types::angle::NetAngle;
use ferrumc_state::GlobalStateResource;
use std::sync::atomic::Ordering;
use tracing::{error, trace, warn};

/// Apply movement requests (update ECS components)
///
/// This is PURE I/O - updates ECS state based on plugin requests.
/// - Reads ApplyMovementRequest events from plugins
/// - Updates Position/Rotation/OnGround components
pub fn apply_movement_requests(
    mut events: EventReader<ApplyMovementRequest>,
    mut transform_query: Query<(&mut Position, &mut Rotation, &mut OnGround)>,
) {
    for request in events.read() {
        let Ok((mut position, mut rotation, mut on_ground)) = transform_query.get_mut(request.player) else {
            error!("Failed to get transform for player {}", request.player);
            continue;
        };

        // Update position if provided
        if let Some(new_pos) = &request.position {
            *position = new_pos.clone();
        }

        // Update rotation if provided
        if let Some(new_rot) = &request.rotation {
            *rotation = new_rot.clone();
        }

        // Update on ground state
        *on_ground = OnGround(request.on_ground);

        trace!("Applied movement for player {}", request.player);
    }
}

/// Enum for different movement packet types
#[derive(NetEncode, Clone)]
enum MovementPacket {
    UpdateEntityPosition(UpdateEntityPositionPacket),
    UpdateEntityPositionAndRotation(UpdateEntityPositionAndRotationPacket),
    UpdateEntityRotation(UpdateEntityRotationPacket),
    TeleportEntity(TeleportEntityPacket),
}

/// Broadcast movement to other players
///
/// This is PURE I/O - no game logic!
/// - Reads BroadcastMovementRequest events from plugins
/// - Sends packets to all connected players
pub fn broadcast_movement_updates(
    mut events: EventReader<BroadcastMovementRequest>,
    identity_query: Query<&PlayerIdentity>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for request in events.read() {
        let Ok(identity) = identity_query.get(request.player) else {
            error!("Failed to get identity for player {}", request.player);
            continue;
        };

        // Build the appropriate packet based on broadcast type
        let packet = match request.broadcast_type {
            MovementBroadcastType::UpdatePosition => {
                let Some(delta_pos) = request.delta_pos else {
                    error!("UpdatePosition broadcast missing delta_pos");
                    continue;
                };
                MovementPacket::UpdateEntityPosition(UpdateEntityPositionPacket::new(
                    identity,
                    delta_pos,
                    request.on_ground,
                ))
            }
            MovementBroadcastType::UpdateRotation => {
                let Some(rotation) = &request.rotation else {
                    error!("UpdateRotation broadcast missing rotation");
                    continue;
                };
                MovementPacket::UpdateEntityRotation(UpdateEntityRotationPacket::new(
                    identity,
                    rotation,
                    request.on_ground,
                ))
            }
            MovementBroadcastType::UpdatePositionAndRotation => {
                let Some(delta_pos) = request.delta_pos else {
                    error!("UpdatePositionAndRotation broadcast missing delta_pos");
                    continue;
                };
                let Some(rotation) = &request.rotation else {
                    error!("UpdatePositionAndRotation broadcast missing rotation");
                    continue;
                };
                MovementPacket::UpdateEntityPositionAndRotation(
                    UpdateEntityPositionAndRotationPacket::new(
                        identity,
                        delta_pos,
                        rotation,
                        request.on_ground,
                    ),
                )
            }
            MovementBroadcastType::Teleport => {
                let Some(position) = &request.position else {
                    error!("Teleport broadcast missing position");
                    continue;
                };
                let default_rotation = Rotation::default();
                let rotation = request.rotation.as_ref().unwrap_or(&default_rotation);
                MovementPacket::TeleportEntity(TeleportEntityPacket::new(
                    identity,
                    position,
                    rotation,
                    request.on_ground,
                ))
            }
        };

        // Broadcast to all connected players (including the moving player)
        for (entity, conn) in conn_query.iter() {
            if !state.0.players.is_connected(entity) || !conn.running.load(Ordering::Relaxed) {
                warn!("Player {} is not connected, skipping broadcast", entity);
                continue;
            }

            if let Err(e) = conn.send_packet_ref(&packet) {
                error!("Failed to send movement packet to player {}: {}", entity, e);
            }
        }
    }
}

/// Broadcast head rotation to other players
///
/// This is PURE I/O - no game logic!
/// - Reads BroadcastHeadRotationRequest events from plugins
/// - Sends head rotation packets to all connected players
pub fn broadcast_head_rotation(
    mut events: EventReader<BroadcastHeadRotationRequest>,
    identity_query: Query<&PlayerIdentity>,
    conn_query: Query<&StreamWriter>,
    state: Res<GlobalStateResource>,
) {
    for request in events.read() {
        let Ok(identity) = identity_query.get(request.player) else {
            error!("Failed to get identity for player {}", request.player);
            continue;
        };

        let head_rot_packet = SetHeadRotationPacket::new(
            identity.uuid.as_u128() as i32,
            NetAngle::from_degrees(request.yaw as f64),
        );

        for writer in conn_query.iter() {
            if !writer.running.load(Ordering::Relaxed) {
                continue;
            }

            if let Err(e) = writer.send_packet_ref(&head_rot_packet) {
                error!("Failed to send head rotation packet: {}", e);
            }
        }

        trace!("Broadcast head rotation for player {}", request.player);
    }
}
