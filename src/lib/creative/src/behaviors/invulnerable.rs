//! Invulnerability behavior for creative mode.

use bevy_ecs::prelude::*;
use ferrumc_api::behavior::{DamageContext, EntityBehavior, Handling};

use crate::CreativeMode;

/// Behavior that makes entities invulnerable in creative mode.
///
/// When attached to player entities, this behavior cancels all damage
/// if the player has the `CreativeMode` component.
pub struct InvulnerableBehavior;

impl EntityBehavior for InvulnerableBehavior {
    fn on_damage(&self, ctx: &mut DamageContext, world: &mut World) -> Handling {
        // Check if the target entity has the CreativeMode component
        if world.get::<CreativeMode>(ctx.target).is_some() {
            // Cancel all damage for creative mode players
            ctx.cancelled = true;
            ctx.damage = 0.0;
            Handling::Prevent // Stop the damage chain entirely
        } else {
            Handling::Pass // Let other behaviors handle it
        }
    }
}
