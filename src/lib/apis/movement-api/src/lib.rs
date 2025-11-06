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
/// Plugin sends this via MovementAPI after validating movement.
/// Core updates ECS components and broadcasts.
///
/// **NOTE:** This is `pub(crate)` - plugins cannot create this directly.
/// Use `MovementAPI::apply_movement()` instead.
#[derive(Event, Clone)]
pub struct ApplyMovementRequest {
    pub(crate) player: Entity,
    pub(crate) position: Option<Position>,
    pub(crate) rotation: Option<Rotation>,
    pub(crate) on_ground: bool,
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
/// Plugin sends this via MovementAPI after determining what type of broadcast is needed.
/// Core handles the actual packet sending.
///
/// **NOTE:** This is `pub(crate)` - plugins cannot create this directly.
/// Use `MovementAPI::broadcast_movement_*()` instead.
#[derive(Event, Clone)]
pub struct BroadcastMovementRequest {
    pub(crate) player: Entity,
    pub(crate) broadcast_type: MovementBroadcastType,
    pub(crate) delta_pos: Option<(i16, i16, i16)>,
    pub(crate) rotation: Option<Rotation>,
    pub(crate) position: Option<Position>,
    pub(crate) on_ground: bool,
    pub(crate) receiver: Option<Entity>,
}

/// Request to broadcast head rotation to other players
///
/// Plugin sends this via MovementAPI when head rotation changes.
/// Core handles the actual packet sending.
///
/// **NOTE:** This is `pub(crate)` - plugins cannot create this directly.
/// Use `MovementAPI::broadcast_head_rotation_*()` instead.
#[derive(Event, Clone)]
pub struct BroadcastHeadRotationRequest {
    pub(crate) player: Entity,
    pub(crate) yaw: f32,
    pub(crate) receiver: Option<Entity>,
}

/// Request to teleport a player to a specific position
///
/// Plugin sends this via MovementAPI when a player should be teleported.
/// Core handles the actual packet sending and ECS state updates.
///
/// **NOTE:** This is `pub(crate)` - plugins cannot create this directly.
/// Use `MovementAPI::teleport()` instead.
#[derive(Event, Clone)]
pub struct TeleportPlayerRequest {
    pub(crate) player: Entity,
    pub(crate) position: Position,
    pub(crate) rotation: Option<Rotation>,
    pub(crate) velocity: Option<(f64, f64, f64)>,
}

/// Movement API - SystemParam for plugins
///
/// Provides methods to apply movement and request broadcasts.
///
/// # Example
///
/// ```rust,no_run
/// fn my_system(mut api: MovementAPI) {
///     // Read movement events
///     for event in api.move_events() {
///         // Validate and apply
///         api.apply_movement(event.player, Some(event.new_position), None, event.on_ground);
///     }
/// }
/// ```
#[derive(SystemParam)]
pub struct MovementAPI<'w, 's> {
    // Write requests
    apply_requests: EventWriter<'w, ApplyMovementRequest>,
    broadcast_requests: EventWriter<'w, BroadcastMovementRequest>,
    head_rotation_requests: EventWriter<'w, BroadcastHeadRotationRequest>,
    teleport_requests: EventWriter<'w, TeleportPlayerRequest>,
    
    // Read input events
    move_reader: EventReader<'w, 's, PlayerMoveEvent>,
    rotate_reader: EventReader<'w, 's, PlayerRotateEvent>,
    move_rotate_reader: EventReader<'w, 's, PlayerMoveAndRotateEvent>,
    head_rotation_reader: EventReader<'w, 's, HeadRotationEvent>,
}

impl<'w, 's> MovementAPI<'w, 's> {
    // ===== Read Methods (Input Events from Core) =====
    
    /// Read player move events (position change only)
    ///
    /// Returns an iterator over movement events emitted by core.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// for event in api.move_events() {
    ///     if validate_movement(&event.old_position, &event.new_position) {
    ///         api.apply_movement(event.player, Some(event.new_position), None, event.on_ground);
    ///     }
    /// }
    /// ```
    pub fn move_events(&mut self) -> impl Iterator<Item = &PlayerMoveEvent> + '_ {
        self.move_reader.read()
    }
    
    /// Read player rotate events (rotation change only)
    ///
    /// Returns an iterator over rotation events emitted by core.
    pub fn rotate_events(&mut self) -> impl Iterator<Item = &PlayerRotateEvent> + '_ {
        self.rotate_reader.read()
    }
    
    /// Read player move and rotate events (both position and rotation change)
    ///
    /// Returns an iterator over combined movement events emitted by core.
    pub fn move_and_rotate_events(&mut self) -> impl Iterator<Item = &PlayerMoveAndRotateEvent> + '_ {
        self.move_rotate_reader.read()
    }
    
    /// Read head rotation events
    ///
    /// Returns an iterator over head rotation events emitted by core.
    pub fn head_rotation_events(&mut self) -> impl Iterator<Item = &HeadRotationEvent> + '_ {
        self.head_rotation_reader.read()
    }
    
    // ===== Write Methods (Requests to Core) =====
    
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
    
    /// Request movement broadcast to all players
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// // Broadcast small position change to everyone
    /// api.broadcast_movement_all(
    ///     player,
    ///     MovementBroadcastType::UpdatePosition,
    ///     Some((delta_x, delta_y, delta_z)),
    ///     None,
    ///     None,
    ///     true,
    /// );
    /// ```
    pub fn broadcast_movement_all(
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
            receiver: None,
        });
    }
    
    /// Request movement broadcast to a specific player
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// // Broadcast to one specific player (for invisibility, etc.)
    /// api.broadcast_movement_to(
    ///     receiver_player,
    ///     moving_player,
    ///     MovementBroadcastType::UpdatePosition,
    ///     Some((delta_x, delta_y, delta_z)),
    ///     None,
    ///     None,
    ///     true,
    /// );
    /// ```
    pub fn broadcast_movement_to(
        &mut self,
        receiver: Entity,
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
            receiver: Some(receiver),
        });
    }
    
    /// Request head rotation broadcast to all players
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// api.broadcast_head_rotation_all(player, new_yaw);
    /// ```
    pub fn broadcast_head_rotation_all(&mut self, player: Entity, yaw: f32) {
        self.head_rotation_requests.write(BroadcastHeadRotationRequest {
            player,
            yaw,
            receiver: None,
        });
    }
    
    /// Request head rotation broadcast to a specific player
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// api.broadcast_head_rotation_to(receiver_player, rotating_player, new_yaw);
    /// ```
    pub fn broadcast_head_rotation_to(&mut self, receiver: Entity, player: Entity, yaw: f32) {
        self.head_rotation_requests.write(BroadcastHeadRotationRequest {
            player,
            yaw,
            receiver: Some(receiver),
        });
    }

    /// Teleport a player to a specific position
    ///
    /// This sends a SynchronizePlayerPosition packet and updates the ECS state.
    /// Use this for spawn teleports, warps, respawns, etc.
    ///
    /// # Arguments
    ///
    /// * `player` - The entity to teleport
    /// * `position` - The target position
    /// * `rotation` - Optional rotation (defaults to current rotation if None)
    /// * `velocity` - Optional velocity (defaults to zero if None)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// // Teleport to spawn
    /// api.teleport(player, Position::new(0.0, 64.0, 0.0), None, None);
    ///
    /// // Teleport with rotation
    /// api.teleport(
    ///     player,
    ///     Position::new(100.0, 64.0, 100.0),
    ///     Some(Rotation { yaw: 0.0, pitch: 0.0 }),
    ///     None
    /// );
    /// ```
    pub fn teleport(
        &mut self,
        player: Entity,
        position: Position,
        rotation: Option<Rotation>,
        velocity: Option<(f64, f64, f64)>,
    ) {
        self.teleport_requests.write(TeleportPlayerRequest {
            player,
            position,
            rotation,
            velocity,
        });
    }
}
