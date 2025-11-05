//! Movement Packet Handlers
//!
//! This module handles the I/O for movement packets.
//! - Receives movement packets from network
//! - Converts them to events for plugins
//! - Pure I/O - no game logic!

use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_movement_api::{
    HeadRotationEvent, PlayerMoveAndRotateEvent, PlayerMoveEvent, PlayerRotateEvent,
};
use ferrumc_net::packets::packet_events::TransformEvent;
use ferrumc_net::{
    SetPlayerPositionAndRotationPacketReceiver, SetPlayerPositionPacketReceiver,
    SetPlayerRotationPacketReceiver,
};
use ferrumc_state::GlobalStateResource;
use tracing::error;

/// Handle set player position packets
///
/// This is PURE I/O - no game logic!
/// - Reads position packets from network
/// - Fires PlayerMoveEvent for plugins to handle
pub fn handle_set_player_position_packets(
    events: Res<SetPlayerPositionPacketReceiver>,
    mut move_events: EventWriter<PlayerMoveEvent>,
    pos_query: Query<&Position>,
    state: Res<GlobalStateResource>,
) {
    for (packet, entity) in events.0.try_iter() {
        if !state.0.players.is_connected(entity) {
            error!(
                "Player {} is not connected, skipping SetPlayerPositionPacket",
                entity
            );
            continue;
        }

        let Ok(old_position) = pos_query.get(entity) else {
            error!("Failed to get position for player {}", entity);
            continue;
        };

        let new_position = Position::new(packet.x, packet.feet_y, packet.z);

        move_events.write(PlayerMoveEvent {
            player: entity,
            old_position: old_position.clone(),
            new_position,
            on_ground: packet.on_ground,
        });
    }
}

/// Handle set player rotation packets
///
/// This is PURE I/O - no game logic!
/// - Reads rotation packets from network
/// - Fires PlayerRotateEvent for plugins to handle
pub fn handle_set_player_rotation_packets(
    events: Res<SetPlayerRotationPacketReceiver>,
    mut rotate_events: EventWriter<PlayerRotateEvent>,
    rot_query: Query<&Rotation>,
    state: Res<GlobalStateResource>,
) {
    for (packet, entity) in events.0.try_iter() {
        if !state.0.players.is_connected(entity) {
            error!(
                "Player {} is not connected, skipping SetPlayerRotationPacket",
                entity
            );
            continue;
        }

        let Ok(old_rotation) = rot_query.get(entity) else {
            error!("Failed to get rotation for player {}", entity);
            continue;
        };

        let new_rotation = Rotation::new(packet.yaw, packet.pitch);
        let on_ground = (packet.flags & 0x01) != 0;

        rotate_events.write(PlayerRotateEvent {
            player: entity,
            old_rotation: old_rotation.clone(),
            new_rotation,
            on_ground,
        });
    }
}

/// Handle set player position and rotation packets
///
/// This is PURE I/O - no game logic!
/// - Reads combined movement packets from network
/// - Fires PlayerMoveAndRotateEvent for plugins to handle
pub fn handle_set_player_position_and_rotation_packets(
    events: Res<SetPlayerPositionAndRotationPacketReceiver>,
    mut move_rotate_events: EventWriter<PlayerMoveAndRotateEvent>,
    transform_query: Query<(&Position, &Rotation)>,
    state: Res<GlobalStateResource>,
) {
    for (packet, entity) in events.0.try_iter() {
        if !state.0.players.is_connected(entity) {
            error!(
                "Player {} is not connected, skipping SetPlayerPositionAndRotationPacket",
                entity
            );
            continue;
        }

        let Ok((old_position, old_rotation)) = transform_query.get(entity) else {
            error!("Failed to get transform for player {}", entity);
            continue;
        };

        let new_position = Position::new(packet.x, packet.feet_y, packet.z);
        let new_rotation = Rotation::new(packet.yaw, packet.pitch);
        let on_ground = (packet.flags & 0x01) != 0;

        move_rotate_events.write(PlayerMoveAndRotateEvent {
            player: entity,
            old_position: old_position.clone(),
            new_position,
            old_rotation: old_rotation.clone(),
            new_rotation,
            on_ground,
        });
    }
}

/// Handle transform events (for head rotation)
///
/// This is PURE I/O - no game logic!
/// - Reads TransformEvent from other systems (legacy)
/// - Fires HeadRotationEvent for plugins to handle
pub fn handle_transform_events(
    mut events: EventReader<TransformEvent>,
    mut head_rotation_events: EventWriter<HeadRotationEvent>,
    rotation_query: Query<&Rotation>,
) {
    for event in events.read() {
        // Only handle rotation changes (head rotation)
        if event.rotation.is_some() {
            let Ok(rotation) = rotation_query.get(event.entity) else {
                continue;
            };

            head_rotation_events.write(HeadRotationEvent {
                player: event.entity,
                rotation: rotation.clone(),
            });
        }
    }
}
