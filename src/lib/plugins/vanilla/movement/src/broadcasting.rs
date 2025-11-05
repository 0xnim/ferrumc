//! Vanilla broadcasting logic
//!
//! Decides who can see movement updates and broadcasts them.
//! This is where vanilla Minecraft rules live.

use ferrumc_plugin_api::prelude::*;
use ferrumc_movement_api::{
    HeadRotationEvent, MovementAPI, MovementBroadcastType, PlayerMoveAndRotateEvent,
    PlayerMoveEvent, PlayerRotateEvent,
};
use tracing::trace;

/// Maximum delta for incremental position updates (7.5 blocks in fixed-point)
const MAX_DELTA: i16 = (7.5 * 4096.0) as i16;

/// Broadcast position updates (vanilla rules)
///
/// For each position change:
/// - Calculate movement delta
/// - Detect if it's a teleport (delta > 7.5 blocks)
/// - Broadcast to ALL other players (vanilla behavior)
pub fn broadcast_position_updates(
    mut events: EventReader<PlayerMoveEvent>,
    mut api: MovementAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        let old_pos = &event.old_position;
        let new_pos = &event.new_position;
        
        // Calculate delta in fixed-point format
        let delta_x = ((new_pos.x * 4096.0) - (old_pos.x * 4096.0)) as i16;
        let delta_y = ((new_pos.y * 4096.0) - (old_pos.y * 4096.0)) as i16;
        let delta_z = ((new_pos.z * 4096.0) - (old_pos.z * 4096.0)) as i16;
        
        // Detect teleport (movement too large)
        let is_teleport = delta_x == i16::MIN || delta_y == i16::MIN || delta_z == i16::MIN
            || delta_x.abs() > MAX_DELTA || delta_y.abs() > MAX_DELTA || delta_z.abs() > MAX_DELTA;
        
        // Vanilla rule: Broadcast to ALL other players
        for (other, _, _) in entities.iter_players() {
            if other == event.player {
                continue; // Don't send to self
            }
            
            if is_teleport {
                api.broadcast_movement(
                    event.player,
                    MovementBroadcastType::Teleport,
                    None,
                    None,
                    Some(new_pos.clone()),
                    event.on_ground,
                );
            } else {
                api.broadcast_movement(
                    event.player,
                    MovementBroadcastType::UpdatePosition,
                    Some((delta_x, delta_y, delta_z)),
                    None,
                    None,
                    event.on_ground,
                );
            }
        }
        
        trace!(
            "Broadcasted position update for player {} (teleport: {})",
            event.player.index(),
            is_teleport
        );
    }
}

/// Broadcast rotation updates (vanilla rules)
///
/// Vanilla rule: Broadcast to ALL other players
pub fn broadcast_rotation_updates(
    mut events: EventReader<PlayerRotateEvent>,
    mut api: MovementAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        // Vanilla rule: Broadcast to ALL other players
        for (other, _, _) in entities.iter_players() {
            if other == event.player {
                continue;
            }
            
            api.broadcast_movement(
                event.player,
                MovementBroadcastType::UpdateRotation,
                None,
                Some(event.new_rotation.clone()),
                None,
                event.on_ground,
            );
        }
        
        trace!(
            "Broadcasted rotation update for player {}",
            event.player.index()
        );
    }
}

/// Broadcast combined position and rotation updates (vanilla rules)
///
/// For combined updates:
/// - Calculate delta, detect teleports
/// - Broadcast to ALL other players
pub fn broadcast_combined_updates(
    mut events: EventReader<PlayerMoveAndRotateEvent>,
    mut api: MovementAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        let old_pos = &event.old_position;
        let new_pos = &event.new_position;
        
        // Calculate delta
        let delta_x = ((new_pos.x * 4096.0) - (old_pos.x * 4096.0)) as i16;
        let delta_y = ((new_pos.y * 4096.0) - (old_pos.y * 4096.0)) as i16;
        let delta_z = ((new_pos.z * 4096.0) - (old_pos.z * 4096.0)) as i16;
        
        // Detect teleport
        let is_teleport = delta_x == i16::MIN || delta_y == i16::MIN || delta_z == i16::MIN
            || delta_x.abs() > MAX_DELTA || delta_y.abs() > MAX_DELTA || delta_z.abs() > MAX_DELTA;
        
        // Vanilla rule: Broadcast to ALL other players
        for (other, _, _) in entities.iter_players() {
            if other == event.player {
                continue;
            }
            
            if is_teleport {
                api.broadcast_movement(
                    event.player,
                    MovementBroadcastType::Teleport,
                    None,
                    Some(event.new_rotation.clone()),
                    Some(new_pos.clone()),
                    event.on_ground,
                );
            } else {
                api.broadcast_movement(
                    event.player,
                    MovementBroadcastType::UpdatePositionAndRotation,
                    Some((delta_x, delta_y, delta_z)),
                    Some(event.new_rotation.clone()),
                    None,
                    event.on_ground,
                );
            }
        }
        
        trace!(
            "Broadcasted combined update for player {} (teleport: {})",
            event.player.index(),
            is_teleport
        );
    }
}

/// Broadcast head rotation (vanilla rules)
///
/// Vanilla rule: Broadcast to ALL other players
pub fn broadcast_head_rotation(
    mut events: EventReader<HeadRotationEvent>,
    mut api: MovementAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        // Vanilla rule: Broadcast to ALL other players
        for (other, _, _) in entities.iter_players() {
            if other == event.player {
                continue;
            }
            
            api.broadcast_head_rotation(event.player, event.rotation.yaw);
        }
        
        trace!(
            "Broadcasted head rotation for player {}",
            event.player.index()
        );
    }
}
