use bevy_ecs::prelude::*;
use ferrumc_net_codec::net_types::var_int::VarInt;

use crate::types::{AnimationType, EntityPose, Hand, PlayerCommand};

/// High-level event: Player swung their arm
///
/// Emitted by core when a SwingArmPacket is received from the network.
/// Plugins listen to this event and decide whether to trigger an animation.
#[derive(Event, Clone, Debug)]
pub struct PlayerSwingArmEvent {
    /// The player entity that swung their arm
    pub player: Entity,
    /// Which hand was swung
    pub hand: Hand,
}

/// High-level event: Player executed a command (sneak, sprint, etc.)
///
/// Emitted by core when a PlayerCommandPacket is received.
/// Plugins listen to this and decide how to handle the command.
#[derive(Event, Clone, Debug)]
pub struct PlayerCommandEvent {
    /// The player entity
    pub player: Entity,
    /// The command action
    pub command: PlayerCommand,
    /// Entity ID from the packet (for network purposes)
    pub entity_id: VarInt,
    /// Jump boost value
    pub jump_boost: VarInt,
}

/// Request to play an animation for an entity
///
/// Emitted by plugins when they want to trigger an animation.
/// Core systems listen to this and broadcast the appropriate packets.
#[derive(Event, Clone, Debug)]
pub struct PlayAnimationRequest {
    /// The entity to animate
    pub entity: Entity,
    /// The animation to play
    pub animation: AnimationType,
    /// Optionally exclude a player from receiving the broadcast (typically the triggering player)
    pub exclude_player: Option<Entity>,
}

/// Request to set an entity's pose/stance
///
/// Emitted by plugins when they want to change an entity's pose.
/// Core systems listen to this and broadcast EntityMetadata packets.
#[derive(Event, Clone, Debug)]
pub struct SetEntityPoseRequest {
    /// The entity whose pose should change
    pub entity: Entity,
    /// Entity ID for the packet
    pub entity_id: VarInt,
    /// The new pose
    pub pose: EntityPose,
    /// Optionally exclude a player from receiving the broadcast (typically the triggering player)
    pub exclude_player: Option<Entity>,
}
