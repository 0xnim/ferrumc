//! Damage handler system that processes damage events.
//!
//! This system reads `PlayerDamaged` events, checks for damage immunity,
//! applies the damage, and emits `PlayerDied` events when health reaches zero.

use bevy_ecs::prelude::*;
use ferrumc_api::components::DamageImmune;
use ferrumc_components::health::Health;
use ferrumc_components::player::hunger::Hunger;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_messages::{DamageSource, PlayerDamaged, PlayerDied};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_health::SetHealth;
use tracing::{debug, error, info};

/// System that handles damage events.
///
/// Flow:
/// 1. Reads `PlayerDamaged` events
/// 2. Skips entities with `DamageImmune` component
/// 3. Applies damage to Health component
/// 4. Sends `SetHealth` packet to update client UI
/// 5. Emits `PlayerDied` event if health reaches zero
#[expect(clippy::type_complexity)]
pub fn handle_damage(
    mut events: MessageReader<PlayerDamaged>,
    mut query: Query<(
        &mut Health,
        &Hunger,
        Option<&PlayerIdentity>,
        Option<&StreamWriter>,
        Option<&DamageImmune>,
    )>,
    mut death_events: MessageWriter<PlayerDied>,
) {
    for event in events.read() {
        let entity = event.player;
        let damage = event.amount;
        let source = event.source;

        let Ok((mut health, hunger, identity, writer, immune)) = query.get_mut(entity) else {
            continue;
        };

        let name = identity
            .map(|id| id.username.as_str())
            .unwrap_or("entity");

        // Skip entities with damage immunity
        if immune.is_some() {
            debug!("Damage prevented for {} (immune)", name);
            continue;
        }

        // Skip if already dead
        if health.current <= 0.0 {
            continue;
        }

        // Apply damage
        let old_health = health.current;
        health.current = (health.current - damage).max(0.0);

        info!(
            "{} took {:.1} damage from {:?}, health: {:.1} -> {:.1}",
            name, damage, source, old_health, health.current
        );

        // Send SetHealth packet with current hunger values
        if let Some(writer) = writer {
            let packet = SetHealth {
                health: health.current,
                food: hunger.level.into(),
                saturation: hunger.saturation,
            };
            if let Err(e) = writer.send_packet_ref(&packet) {
                error!("Failed to send SetHealth packet: {:?}", e);
            }
        }

        // Check for death
        if health.current <= 0.0 {
            info!("{} died from {:?}", name, source);

            // Extract killer entity if applicable
            let killer = match source {
                DamageSource::EntityAttack { attacker } => Some(attacker),
                DamageSource::Explosion { source } => source,
                _ => None,
            };

            death_events.write(PlayerDied {
                player: entity,
                source,
                killer,
            });
        }
    }
}
