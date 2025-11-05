//! Movement Plugin for FerrumC
//!
//! This plugin handles player movement validation and broadcasting.
//!
//! # Architecture
//!
//! - Core fires PlayerMoveEvent / PlayerRotateEvent from packets
//! - This plugin validates movement and detects teleports (game logic)
//! - Plugin uses MovementAPI to apply changes and broadcast
//! - Core handles actual ECS updates and packet sending (I/O)

use ferrumc_plugin_api::prelude::*;
use ferrumc_movement_api::{
    ApplyMovementRequest, BroadcastHeadRotationRequest, BroadcastMovementRequest,
    HeadRotationEvent, MovementAPI, MovementBroadcastType, PlayerMoveAndRotateEvent,
    PlayerMoveEvent, PlayerRotateEvent,
};
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use tracing::{debug, trace};

#[derive(Default)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn name(&self) -> &'static str {
        "movement"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Handles player movement validation and broadcasting"
    }

    fn priority(&self) -> i32 {
        100 // High priority - movement is fundamental
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_movement_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        trace!("Loading movement plugin");

        // Register events from movement API
        ctx.events()
            .register::<PlayerMoveEvent>()
            .register::<PlayerRotateEvent>()
            .register::<PlayerMoveAndRotateEvent>()
            .register::<HeadRotationEvent>()
            .register::<ApplyMovementRequest>()
            .register::<BroadcastMovementRequest>()
            .register::<BroadcastHeadRotationRequest>()
            .register::<CrossChunkBoundaryEvent>();

        // Register our gameplay logic systems
        ctx.systems()
            .add_tick(handle_player_move)
            .add_tick(handle_player_rotate)
            .add_tick(handle_player_move_and_rotate)
            .add_tick(handle_head_rotation);

        trace!("Movement plugin loaded successfully");
    }
}

/// Maximum delta for incremental position updates (7.5 blocks in fixed-point)
const MAX_DELTA: i16 = (7.5 * 4096.0) as i16;

/// Handle position-only movement events
///
/// This is pure game logic:
/// - Validates movement deltas
/// - Detects teleports (delta > 7.5 blocks)
/// - Fires CrossChunkBoundaryEvent if chunk changed
/// - Determines broadcast type and sends requests
fn handle_player_move(
    mut events: EventReader<PlayerMoveEvent>,
    mut api: MovementAPI,
    mut cross_chunk_events: EventWriter<CrossChunkBoundaryEvent>,
) {
    for event in events.read() {
        let old_pos = &event.old_position;
        let new_pos = &event.new_position;
        
        // Calculate delta in fixed-point format
        let delta_x = ((new_pos.x * 4096.0) - (old_pos.x * 4096.0)) as i16;
        let delta_y = ((new_pos.y * 4096.0) - (old_pos.y * 4096.0)) as i16;
        let delta_z = ((new_pos.z * 4096.0) - (old_pos.z * 4096.0)) as i16;
        
        // Check if movement exceeds threshold (needs teleport)
        let needs_teleport = delta_x == i16::MIN || delta_y == i16::MIN || delta_z == i16::MIN
            || delta_x.abs() > MAX_DELTA || delta_y.abs() > MAX_DELTA || delta_z.abs() > MAX_DELTA;
        
        // Check for chunk boundary crossing
        let old_chunk = (old_pos.x as i32 >> 4, old_pos.z as i32 >> 4);
        let new_chunk = (new_pos.x as i32 >> 4, new_pos.z as i32 >> 4);
        
        if old_chunk != new_chunk {
            cross_chunk_events.write(CrossChunkBoundaryEvent {
                player: event.player,
                old_chunk,
                new_chunk,
            });
            debug!(
                "Player {} crossed chunk boundary: {:?} -> {:?}",
                event.player.index(),
                old_chunk,
                new_chunk
            );
        }
        
        // Apply the movement
        api.apply_movement(event.player, Some(new_pos.clone()), None, event.on_ground);
        
        // Broadcast to other players
        if needs_teleport {
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
        
        trace!(
            "Player {} moved to ({}, {}, {})",
            event.player.index(),
            new_pos.x,
            new_pos.y,
            new_pos.z
        );
    }
}

/// Handle rotation-only movement events
///
/// This is pure game logic:
/// - Applies rotation changes
/// - Broadcasts rotation updates
fn handle_player_rotate(
    mut events: EventReader<PlayerRotateEvent>,
    mut api: MovementAPI,
) {
    for event in events.read() {
        // Apply the rotation
        api.apply_movement(event.player, None, Some(event.new_rotation.clone()), event.on_ground);
        
        // Broadcast rotation update
        api.broadcast_movement(
            event.player,
            MovementBroadcastType::UpdateRotation,
            None,
            Some(event.new_rotation.clone()),
            None,
            event.on_ground,
        );
        
        trace!(
            "Player {} rotated to (yaw: {}, pitch: {})",
            event.player.index(),
            event.new_rotation.yaw,
            event.new_rotation.pitch
        );
    }
}

/// Handle combined position and rotation movement events
///
/// This is pure game logic:
/// - Validates movement deltas
/// - Detects teleports
/// - Fires CrossChunkBoundaryEvent if needed
/// - Broadcasts combined updates
fn handle_player_move_and_rotate(
    mut events: EventReader<PlayerMoveAndRotateEvent>,
    mut api: MovementAPI,
    mut cross_chunk_events: EventWriter<CrossChunkBoundaryEvent>,
) {
    for event in events.read() {
        let old_pos = &event.old_position;
        let new_pos = &event.new_position;
        
        // Calculate delta in fixed-point format
        let delta_x = ((new_pos.x * 4096.0) - (old_pos.x * 4096.0)) as i16;
        let delta_y = ((new_pos.y * 4096.0) - (old_pos.y * 4096.0)) as i16;
        let delta_z = ((new_pos.z * 4096.0) - (old_pos.z * 4096.0)) as i16;
        
        // Check if movement exceeds threshold
        let needs_teleport = delta_x == i16::MIN || delta_y == i16::MIN || delta_z == i16::MIN
            || delta_x.abs() > MAX_DELTA || delta_y.abs() > MAX_DELTA || delta_z.abs() > MAX_DELTA;
        
        // Check for chunk boundary crossing
        let old_chunk = (old_pos.x as i32 >> 4, old_pos.z as i32 >> 4);
        let new_chunk = (new_pos.x as i32 >> 4, new_pos.z as i32 >> 4);
        
        if old_chunk != new_chunk {
            cross_chunk_events.write(CrossChunkBoundaryEvent {
                player: event.player,
                old_chunk,
                new_chunk,
            });
            debug!(
                "Player {} crossed chunk boundary: {:?} -> {:?}",
                event.player.index(),
                old_chunk,
                new_chunk
            );
        }
        
        // Apply the movement and rotation
        api.apply_movement(
            event.player,
            Some(new_pos.clone()),
            Some(event.new_rotation.clone()),
            event.on_ground,
        );
        
        // Broadcast to other players
        if needs_teleport {
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
        
        trace!(
            "Player {} moved and rotated to ({}, {}, {}) (yaw: {}, pitch: {})",
            event.player.index(),
            new_pos.x,
            new_pos.y,
            new_pos.z,
            event.new_rotation.yaw,
            event.new_rotation.pitch
        );
    }
}

/// Handle head rotation events
///
/// This is pure game logic:
/// - Broadcasts head rotation to other players
fn handle_head_rotation(
    mut events: EventReader<HeadRotationEvent>,
    mut api: MovementAPI,
) {
    for event in events.read() {
        api.broadcast_head_rotation(event.player, event.rotation.yaw);
        
        trace!(
            "Player {} head rotated to yaw: {}",
            event.player.index(),
            event.rotation.yaw
        );
    }
}
