use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_net_codec::net_types::var_int::VarInt;

use crate::events::{PlayAnimationRequest, SetEntityPoseRequest, PlayerSwingArmEvent, PlayerCommandEvent, PlayerInputEvent};
use crate::types::{AnimationType, EntityPose};

/// Plugin API for triggering animations
///
/// This is a system parameter that plugins use to request animations
/// without knowing about the underlying network implementation.
///
/// # Example
///
/// ```rust,no_run
/// use bevy_ecs::prelude::*;
/// use ferrumc_animation_api::{AnimationAPI, AnimationType};
///
/// fn my_system(mut animations: AnimationAPI) {
///     // Read swing events
///     for event in animations.swing_events() {
///         // Handle swing
///     }
///     
///     // Play animation for an entity
///     animations.play_animation(entity, AnimationType::SwingMainArm);
/// }
/// ```
#[derive(SystemParam)]
pub struct AnimationAPI<'w, 's> {
    // Write requests
    animation_events: EventWriter<'w, PlayAnimationRequest>,
    pose_events: EventWriter<'w, SetEntityPoseRequest>,
    
    // Read input events
    swing_reader: EventReader<'w, 's, PlayerSwingArmEvent>,
    command_reader: EventReader<'w, 's, PlayerCommandEvent>,
    input_reader: EventReader<'w, 's, PlayerInputEvent>,
}

impl<'w, 's> AnimationAPI<'w, 's> {
    // ===== Read Methods (Input Events from Core) =====
    
    /// Read player swing arm events
    ///
    /// Returns an iterator over swing events emitted by core when players swing their arm.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// for event in animations.swing_events() {
    ///     let animation = match event.hand {
    ///         Hand::Main => AnimationType::SwingMainArm,
    ///         Hand::Off => AnimationType::SwingOffhand,
    ///     };
    ///     animations.play_animation(event.player, animation);
    /// }
    /// ```
    pub fn swing_events(&mut self) -> impl Iterator<Item = &PlayerSwingArmEvent> + '_ {
        self.swing_reader.read()
    }
    
    /// Read player command events (sneak, sprint, etc.)
    ///
    /// Returns an iterator over command events emitted by core.
    pub fn command_events(&mut self) -> impl Iterator<Item = &PlayerCommandEvent> + '_ {
        self.command_reader.read()
    }
    
    /// Read player input events (jump, sneak, sprint flags)
    ///
    /// Returns an iterator over input events emitted by core.
    pub fn input_events(&mut self) -> impl Iterator<Item = &PlayerInputEvent> + '_ {
        self.input_reader.read()
    }
    
    // ===== Write Methods (Requests to Core) =====
    
    /// Play an animation for an entity, visible to all players
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to animate
    /// * `animation` - The animation type to play
    pub fn play_animation(&mut self, entity: Entity, animation: AnimationType) {
        self.animation_events.write(PlayAnimationRequest {
            entity,
            animation,
            exclude_player: None,
        });
    }

    /// Play an animation for an entity, visible to all players except the triggering player
    ///
    /// This is the typical use case for player-triggered animations (swing, etc.)
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to animate
    /// * `animation` - The animation type to play
    /// * `exclude` - The player to exclude from the broadcast (usually the triggering player)
    pub fn play_animation_except(&mut self, entity: Entity, animation: AnimationType, exclude: Entity) {
        self.animation_events.write(PlayAnimationRequest {
            entity,
            animation,
            exclude_player: Some(exclude),
        });
    }

    /// Set an entity's pose, visible to all players
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity whose pose to change
    /// * `entity_id` - The entity's network ID
    /// * `pose` - The new pose
    pub fn set_pose(&mut self, entity: Entity, entity_id: VarInt, pose: EntityPose) {
        self.pose_events.write(SetEntityPoseRequest {
            entity,
            entity_id,
            pose,
            exclude_player: None,
        });
    }

    /// Set an entity's pose, visible to all players except the triggering player
    ///
    /// This is the typical use case for player-triggered pose changes (sneak, sprint, etc.)
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity whose pose to change
    /// * `entity_id` - The entity's network ID
    /// * `pose` - The new pose
    /// * `exclude` - The player to exclude from the broadcast (usually the triggering player)
    pub fn set_pose_except(&mut self, entity: Entity, entity_id: VarInt, pose: EntityPose, exclude: Entity) {
        self.pose_events.write(SetEntityPoseRequest {
            entity,
            entity_id,
            pose,
            exclude_player: Some(exclude),
        });
    }
}
