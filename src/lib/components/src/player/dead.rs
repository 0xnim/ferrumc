//! Dead player marker component.
//!
//! Added to player entities when they die, removed when they respawn.

use bevy_ecs::prelude::{Component, Entity};

/// Marker component for dead players.
///
/// This component is added when a player's health reaches zero and
/// is removed when they respawn. While this component is present,
/// the player should not be able to take actions.
#[derive(Component, Debug, Clone, Default)]
pub struct DeadPlayer {
    /// Optional killer entity (for PvP death messages)
    pub killer: Option<Entity>,
}
