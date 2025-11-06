//! Teleport system - handles teleport requests from plugins
//!
//! This system:
//! - Sends SynchronizePlayerPosition packets
//! - Updates Position/Rotation components in ECS
//! - Does NOT broadcast to other players (use broadcasting system for that)

use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_movement_api::TeleportPlayerRequest;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::{debug, error, warn};

/// Handle teleport requests from plugins
///
/// This is a core system that performs I/O:
/// - Sends network packets
/// - Updates ECS components
///
/// Plugins should use MovementAPI::teleport() instead of sending packets directly.
pub fn handle_teleport_requests(
    mut events: EventReader<TeleportPlayerRequest>,
    mut writers: Query<&mut StreamWriter>,
    mut positions: Query<&mut Position>,
    mut rotations: Query<&mut Rotation>,
) {
    for request in events.read() {
        // Get current rotation if not specified
        let rotation = request.rotation.clone().or_else(|| {
            rotations.get(request.player).ok().map(|r| r.clone())
        }).unwrap_or_default();

        // Use zero velocity if not specified
        let velocity = request.velocity.unwrap_or((0.0, 0.0, 0.0));

        // Send the teleport packet
        let Ok(mut writer) = writers.get_mut(request.player) else {
            warn!("Could not get StreamWriter for player {:?}", request.player);
            continue;
        };

        // TODO: Generate proper teleport IDs (for now using 0)
        let packet = SynchronizePlayerPositionPacket::new(
            (request.position.x, request.position.y, request.position.z),
            velocity,
            rotation.yaw,
            rotation.pitch,
            0, // flags
            VarInt::new(0), // teleport_id
        );

        if let Err(e) = writer.send_packet_ref(&packet) {
            error!(
                "Failed to send teleport packet for player {:?}: {:?}",
                request.player, e
            );
            continue;
        }

        // Update ECS state
        if let Ok(mut pos) = positions.get_mut(request.player) {
            *pos = request.position.clone();
        } else {
            warn!("Could not update position for player {:?}", request.player);
        }

        if let Ok(mut rot) = rotations.get_mut(request.player) {
            *rot = rotation.clone();
        } else {
            warn!("Could not update rotation for player {:?}", request.player);
        }

        debug!(
            "Teleported player {:?} to ({}, {}, {})",
            request.player, request.position.x, request.position.y, request.position.z
        );
    }
}
