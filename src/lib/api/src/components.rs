//! Shared marker components for cross-mod functionality.
//!
//! These components are defined in the API crate so multiple mods
//! can interact without depending on each other.

use bevy_ecs::prelude::*;

/// Marker component indicating an entity is immune to damage.
///
/// Any mod can add this component to prevent damage from being applied.
/// The survival mod's damage handler will skip entities with this component.
///
/// # Examples
///
/// - Creative mode: Adds `DamageImmune` when player enters creative
/// - God mode command: Adds `DamageImmune` while active
/// - Invincibility powerup: Adds `DamageImmune` with a timer
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct DamageImmune;
