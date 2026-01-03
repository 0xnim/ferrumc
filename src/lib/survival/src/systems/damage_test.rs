//! Test system that applies 1 damage to all entities every second.
//!
//! This is a DEBUG/TEST system to verify damage mechanics work.
//! Should be removed or disabled in production.

use bevy_ecs::prelude::*;
use ferrumc_components::health::Health;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_messages::PlayerDamaged;
use tracing::info;

/// Tracks ticks for the damage test system.
#[derive(Resource, Default)]
pub struct DamageTestTimer {
    pub ticks: u32,
}

/// Test system that emits damage events for all entities with Health every second (20 ticks).
///
/// This system:
/// 1. Counts ticks until 20 (1 second at 20 TPS)
/// 2. Finds all entities with Health component
/// 3. Emits a `PlayerDamaged` event for 1.0 damage
///
/// The actual damage application is handled by `damage_handler::handle_damage`,
/// which checks entity behaviors (like creative mode invulnerability) before applying.
pub fn tick(
    mut timer: ResMut<DamageTestTimer>,
    query: Query<(Entity, &Health, Option<&PlayerIdentity>)>,
    mut damage_events: MessageWriter<PlayerDamaged>,
) {
    timer.ticks += 1;

    // Only run every 20 ticks (1 second at 20 TPS)
    if timer.ticks < 20 {
        return;
    }
    timer.ticks = 0;

    for (entity, health, identity) in query.iter() {
        // Skip if already dead
        if health.current <= 0.0 {
            continue;
        }

        let name = identity
            .map(|id| id.username.as_str())
            .unwrap_or("entity");

        info!(
            "Damage test: emitting damage event for {} ({:?})",
            name, entity
        );

        // Emit damage event - the damage_handler will process it
        damage_events.write(PlayerDamaged {
            player: entity,
            amount: 1.0,
        });
    }
}
