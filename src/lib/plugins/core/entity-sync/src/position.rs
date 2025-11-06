//! Position state updates
//!
//! Updates Position/OnGround components from movement events.
//! NO validation, NO broadcasting - just state updates.

use ferrumc_plugin_api::prelude::*;
use ferrumc_movement_api::MovementAPI;
use tracing::trace;

/// Update position state from PlayerMoveEvent
///
/// - Updates Position and OnGround components
/// - Requests ECS update via MovementAPI
pub fn update_position_from_move(
    mut api: MovementAPI,
) {
    let events: Vec<_> = api.move_events().cloned().collect();
    for event in events {
        let new_pos = &event.new_position;
        
        // Apply the position update to ECS
        api.apply_movement(
            event.player,
            Some(new_pos.clone()),
            None,
            event.on_ground,
        );
        
        trace!(
            "Updated position for player {}: ({}, {}, {})",
            event.player.index(),
            new_pos.x,
            new_pos.y,
            new_pos.z
        );
    }
}

/// Update position and rotation state from PlayerMoveAndRotateEvent
///
/// - Updates Position, Rotation, and OnGround components
/// - Requests ECS update via MovementAPI
pub fn update_position_and_rotation_from_move_rotate(
    mut api: MovementAPI,
) {
    let events: Vec<_> = api.move_and_rotate_events().cloned().collect();
    for event in events {
        let new_pos = &event.new_position;
        
        // Apply both position and rotation to ECS
        api.apply_movement(
            event.player,
            Some(new_pos.clone()),
            Some(event.new_rotation.clone()),
            event.on_ground,
        );
        
        trace!(
            "Updated position and rotation for player {}: ({}, {}, {}) (yaw: {}, pitch: {})",
            event.player.index(),
            new_pos.x,
            new_pos.y,
            new_pos.z,
            event.new_rotation.yaw,
            event.new_rotation.pitch
        );
    }
}
