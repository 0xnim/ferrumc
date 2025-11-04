use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_net_codec::net_types::var_int::VarInt;

use crate::events::{PlayAnimationRequest, SetEntityPoseRequest};
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
///     // Play animation for an entity
///     animations.play_animation(entity, AnimationType::SwingMainArm);
/// }
/// ```
#[derive(SystemParam)]
pub struct AnimationAPI<'w> {
    animation_events: EventWriter<'w, PlayAnimationRequest>,
    pose_events: EventWriter<'w, SetEntityPoseRequest>,
}

impl<'w> AnimationAPI<'w> {
    /// Play an animation for an entity, visible to nearby players
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to animate
    /// * `animation` - The animation type to play
    pub fn play_animation(&mut self, entity: Entity, animation: AnimationType) {
        self.animation_events.write(PlayAnimationRequest {
            entity,
            animation,
            broadcast_to_all: false,
        });
    }

    /// Play an animation for an entity, visible to all players
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to animate
    /// * `animation` - The animation type to play
    pub fn play_animation_global(&mut self, entity: Entity, animation: AnimationType) {
        self.animation_events.write(PlayAnimationRequest {
            entity,
            animation,
            broadcast_to_all: true,
        });
    }

    /// Set an entity's pose, visible to nearby players
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
            broadcast_to_all: false,
        });
    }

    /// Set an entity's pose, visible to all players
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity whose pose to change
    /// * `entity_id` - The entity's network ID
    /// * `pose` - The new pose
    pub fn set_pose_global(&mut self, entity: Entity, entity_id: VarInt, pose: EntityPose) {
        self.pose_events.write(SetEntityPoseRequest {
            entity,
            entity_id,
            pose,
            broadcast_to_all: true,
        });
    }
}
