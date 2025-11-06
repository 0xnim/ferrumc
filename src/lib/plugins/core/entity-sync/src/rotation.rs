//! Rotation state updates
//!
//! Updates Rotation/OnGround components from rotation events.
//! NO validation, NO broadcasting - just state updates.

use ferrumc_plugin_api::prelude::*;
use ferrumc_movement_api::{MovementAPI, PlayerRotateEvent};
use tracing::trace;

/// Update rotation state from PlayerRotateEvent
///
/// - Updates Rotation and OnGround components
/// - Requests ECS update via MovementAPI
pub fn update_rotation_from_rotate(
    mut api: MovementAPI,
) {
    let events: Vec<_> = api.rotate_events().cloned().collect();
    for event in events {
        // Apply rotation update to ECS
        api.apply_movement(
            event.player,
            None,
            Some(event.new_rotation.clone()),
            event.on_ground,
        );
        
        trace!(
            "Updated rotation for player {}: (yaw: {}, pitch: {})",
            event.player.index(),
            event.new_rotation.yaw,
            event.new_rotation.pitch
        );
    }
}
