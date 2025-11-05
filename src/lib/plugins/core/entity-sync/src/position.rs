//! Position state updates
//!
//! Updates Position/OnGround components from movement events.
//! NO validation, NO broadcasting - just state updates.

use ferrumc_plugin_api::prelude::*;
use ferrumc_movement_api::{MovementAPI, PlayerMoveAndRotateEvent, PlayerMoveEvent};
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use tracing::trace;

/// Update position state from PlayerMoveEvent
///
/// - Updates Position and OnGround components
/// - Fires CrossChunkBoundaryEvent if chunk changed
/// - Requests ECS update via MovementAPI
pub fn update_position_from_move(
    mut events: EventReader<PlayerMoveEvent>,
    mut api: MovementAPI,
    mut cross_chunk_events: EventWriter<CrossChunkBoundaryEvent>,
) {
    for event in events.read() {
        let old_pos = &event.old_position;
        let new_pos = &event.new_position;
        
        // Check for chunk boundary crossing
        let old_chunk = (old_pos.x as i32 >> 4, old_pos.z as i32 >> 4);
        let new_chunk = (new_pos.x as i32 >> 4, new_pos.z as i32 >> 4);
        
        if old_chunk != new_chunk {
            cross_chunk_events.write(CrossChunkBoundaryEvent {
                player: event.player,
                old_chunk,
                new_chunk,
            });
        }
        
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
/// - Fires CrossChunkBoundaryEvent if chunk changed  
/// - Requests ECS update via MovementAPI
pub fn update_position_and_rotation_from_move_rotate(
    mut events: EventReader<PlayerMoveAndRotateEvent>,
    mut api: MovementAPI,
    mut cross_chunk_events: EventWriter<CrossChunkBoundaryEvent>,
) {
    for event in events.read() {
        let old_pos = &event.old_position;
        let new_pos = &event.new_position;
        
        // Check for chunk boundary crossing
        let old_chunk = (old_pos.x as i32 >> 4, old_pos.z as i32 >> 4);
        let new_chunk = (new_pos.x as i32 >> 4, new_pos.z as i32 >> 4);
        
        if old_chunk != new_chunk {
            cross_chunk_events.write(CrossChunkBoundaryEvent {
                player: event.player,
                old_chunk,
                new_chunk,
            });
        }
        
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
