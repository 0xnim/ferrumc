//! Movement API for FerrumC
//!
//! This API handles player movement, rotation, and position updates.
//!
//! # Architecture
//!
//! **Events (Core → Plugin):**
//! - `PlayerMoveEvent` - Fired when a player changes position
//! - `PlayerRotateEvent` - Fired when a player changes rotation
//! - `PlayerMoveAndRotateEvent` - Fired when a player changes both
//!
//! **Requests (Plugin → Core):**
//! - `ApplyMovementRequest` - Request to apply validated movement
//! - `BroadcastMovementRequest` - Request to broadcast movement to nearby players
//!
//! # Example
//!
//! ```rust,no_run
//! fn handle_player_move(
//!     mut events: EventReader<PlayerMoveEvent>,
//!     mut api: MovementAPI,
//! ) {
//!     for event in events.read() {
//!         // Validate movement (game logic)
//!         if is_valid_movement(&event.old_position, &event.new_position) {
//!             // Apply movement
//!             api.apply_movement(event.player, event.new_position, event.on_ground);
//!         }
//!     }
//! }
//! ```

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;

/// Event fired when a player moves (position change only)
///
/// Core fires this event when receiving movement packets.
/// Plugins validate and apply the movement.
#[derive(Event, Clone)]
pub struct PlayerMoveEvent {
    /// The entity ID of the moving player
    pub player: Entity,
    
    /// Previous position
    pub old_position: Position,
    
    /// New position from packet
    pub new_position: Position,
    
    /// On ground state
    pub on_ground: bool,
}

/// Event fired when a player rotates (rotation change only)
///
/// Core fires this event when receiving rotation packets.
/// Plugins validate and apply the rotation.
#[derive(Event, Clone)]
pub struct PlayerRotateEvent {
    /// The entity ID of the rotating player
    pub player: Entity,
    
    /// Previous rotation
    pub old_rotation: Rotation,
    
    /// New rotation from packet
    pub new_rotation: Rotation,
    
    /// On ground state
    pub on_ground: bool,
}

/// Event fired when a player moves and rotates
///
/// Core fires this event when receiving combined movement packets.
/// Plugins validate and apply both changes.
#[derive(Event, Clone)]
pub struct PlayerMoveAndRotateEvent {
    /// The entity ID of the moving player
    pub player: Entity,
    
    /// Previous position
    pub old_position: Position,
    
    /// New position from packet
    pub new_position: Position,
    
    /// Previous rotation
    pub old_rotation: Rotation,
    
    /// New rotation from packet
    pub new_rotation: Rotation,
    
    /// On ground state
    pub on_ground: bool,
}

/// Event fired when head rotation changes
///
/// Core fires this when rotation changes (for broadcasting to other players).
/// Plugins can listen to sync head rotation packets.
#[derive(Event, Clone)]
pub struct HeadRotationEvent {
    /// The entity ID of the player
    pub player: Entity,
    
    /// New rotation
    pub rotation: Rotation,
}

/// Request to apply validated movement
///
/// Plugin sends this after validating movement.
/// Core updates ECS components and broadcasts.
#[derive(Event, Clone)]
pub struct ApplyMovementRequest {
    /// The player to update
    pub player: Entity,
    
    /// New position (if changed)
    pub position: Option<Position>,
    
    /// New rotation (if changed)
    pub rotation: Option<Rotation>,
    
    /// On ground state
    pub on_ground: bool,
}

/// Type of movement packet to broadcast
#[derive(Clone, Copy, Debug)]
pub enum MovementBroadcastType {
    /// Small position change
    UpdatePosition,
    
    /// Rotation change
    UpdateRotation,
    
    /// Position and rotation change
    UpdatePositionAndRotation,
    
    /// Large movement (teleport)
    Teleport,
}

/// Request to broadcast movement to other players
///
/// Plugin sends this after determining what type of broadcast is needed.
/// Core handles the actual packet sending.
#[derive(Event, Clone)]
pub struct BroadcastMovementRequest {
    /// The player who moved
    pub player: Entity,
    
    /// Type of broadcast packet to send
    pub broadcast_type: MovementBroadcastType,
    
    /// Position delta for incremental updates (in fixed-point format)
    pub delta_pos: Option<(i16, i16, i16)>,
    
    /// New rotation for rotation updates
    pub rotation: Option<Rotation>,
    
    /// Absolute position for teleport
    pub position: Option<Position>,
    
    /// On ground state
    pub on_ground: bool,
}

/// Request to broadcast head rotation to other players
///
/// Plugin sends this when head rotation changes.
/// Core handles the actual packet sending.
#[derive(Event, Clone)]
pub struct BroadcastHeadRotationRequest {
    /// The player whose head rotated
    pub player: Entity,
    
    /// New head rotation (yaw)
    pub yaw: f32,
}

/// Movement API - SystemParam for plugins
///
/// Provides methods to apply movement and request broadcasts.
///
/// # Example
///
/// ```rust,no_run
/// fn my_system(mut api: MovementAPI) {
///     api.apply_movement(player, Some(new_pos), None, true);
///     api.broadcast_movement(player, MovementBroadcastType::UpdatePosition, ...);
///     api.broadcast_head_rotation(player, yaw);
/// }
/// ```
#[derive(SystemParam)]
pub struct MovementAPI<'w> {
    apply_requests: EventWriter<'w, ApplyMovementRequest>,
    broadcast_requests: EventWriter<'w, BroadcastMovementRequest>,
    head_rotation_requests: EventWriter<'w, BroadcastHeadRotationRequest>,
}

impl<'w> MovementAPI<'w> {
    /// Apply validated movement
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// // Update position only
    /// api.apply_movement(player, Some(new_pos), None, true);
    /// 
    /// // Update rotation only
    /// api.apply_movement(player, None, Some(new_rot), false);
    /// 
    /// // Update both
    /// api.apply_movement(player, Some(new_pos), Some(new_rot), true);
    /// ```
    pub fn apply_movement(
        &mut self,
        player: Entity,
        position: Option<Position>,
        rotation: Option<Rotation>,
        on_ground: bool,
    ) {
        self.apply_requests.write(ApplyMovementRequest {
            player,
            position,
            rotation,
            on_ground,
        });
    }
    
    /// Request movement broadcast
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// // Broadcast small position change
    /// api.broadcast_movement(
    ///     player,
    ///     MovementBroadcastType::UpdatePosition,
    ///     Some((delta_x, delta_y, delta_z)),
    ///     None,
    ///     None,
    ///     true,
    /// );
    /// ```
    pub fn broadcast_movement(
        &mut self,
        player: Entity,
        broadcast_type: MovementBroadcastType,
        delta_pos: Option<(i16, i16, i16)>,
        rotation: Option<Rotation>,
        position: Option<Position>,
        on_ground: bool,
    ) {
        self.broadcast_requests.write(BroadcastMovementRequest {
            player,
            broadcast_type,
            delta_pos,
            rotation,
            position,
            on_ground,
        });
    }
    
    /// Request head rotation broadcast
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// api.broadcast_head_rotation(player, new_yaw);
    /// ```
    pub fn broadcast_head_rotation(&mut self, player: Entity, yaw: f32) {
        self.head_rotation_requests.write(BroadcastHeadRotationRequest {
            player,
            yaw,
        });
    }
}
