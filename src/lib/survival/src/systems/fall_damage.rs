//! Fall damage system.
//!
//! Tracks player fall distance and emits damage events when landing.

use bevy_ecs::prelude::*;
use ferrumc_components::player::fall_distance::FallDistance;
use ferrumc_core::transform::{grounded::OnGround, position::Position};
use ferrumc_messages::PlayerDamaged;
use tracing::debug;

/// System that tracks fall distance and emits damage events.
///
/// This system:
/// 1. Detects when a player transitions from airborne to grounded
/// 2. Calculates fall damage based on distance (damage = distance - 3)
/// 3. Emits `PlayerDamaged` event with `DamageSource::Fall`
/// 4. Resets the fall tracker
///
/// The damage is actually applied by the `damage_handler` system.
pub fn handle_fall_damage(
    mut query: Query<(Entity, &Position, &OnGround, &mut FallDistance)>,
    mut damage_events: MessageWriter<PlayerDamaged>,
) {
    for (entity, position, on_ground, mut fall) in query.iter_mut() {
        let currently_grounded = on_ground.0;
        let was_grounded = fall.was_on_ground;

        if !was_grounded && currently_grounded {
            // Just landed - check for fall damage
            let fall_distance = fall.distance;

            if fall.would_cause_damage() {
                debug!(
                    "Entity {:?} landed after falling {:.1} blocks, dealing {:.1} damage",
                    entity,
                    fall_distance,
                    fall.calculate_damage()
                );

                // Emit fall damage event
                damage_events.write(PlayerDamaged::fall(entity, fall_distance));
            }

            // Reset fall tracking
            fall.reset();
        } else if was_grounded && !currently_grounded {
            // Just started falling - record the starting Y position
            fall.start_fall(position.y);
        } else if !currently_grounded {
            // Still falling - update distance
            fall.update(position.y);
        }

        // Update grounded state for next tick
        fall.was_on_ground = currently_grounded;
    }
}
