//! Hunger tick system.
//!
//! Implements Minecraft's hunger mechanics:
//! - Exhaustion drains saturation, then hunger level
//! - Health regeneration when hunger is high
//! - Starvation damage when hunger is empty

use bevy_ecs::prelude::*;
use ferrumc_components::{health::Health, player::hunger::Hunger};
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_messages::PlayerDamaged;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_health::SetHealth;
use tracing::debug;

/// Timer for hunger tick system (runs every 80 ticks = 4 seconds).
#[derive(Resource, Default)]
pub struct HungerTickTimer {
    pub ticks: u32,
}

/// System that processes hunger mechanics every 80 ticks (4 seconds).
///
/// Minecraft hunger mechanics:
/// 1. Exhaustion drains saturation (1 saturation per 4 exhaustion)
/// 2. When saturation is 0, hunger level drains instead
/// 3. When hunger >= 18, regenerate 1 HP (costs 6 exhaustion)
/// 4. When hunger == 0, deal starvation damage
#[expect(clippy::type_complexity)]
pub fn tick(
    mut timer: ResMut<HungerTickTimer>,
    mut query: Query<(
        Entity,
        &mut Health,
        &mut Hunger,
        Option<&PlayerIdentity>,
        Option<&StreamWriter>,
    )>,
    mut damage_events: MessageWriter<PlayerDamaged>,
) {
    timer.ticks += 1;

    // Hunger tick runs every 80 ticks (4 seconds at 20 TPS)
    if timer.ticks < 80 {
        return;
    }
    timer.ticks = 0;

    for (entity, mut health, mut hunger, identity, writer) in query.iter_mut() {
        // Skip dead entities
        if health.current <= 0.0 {
            continue;
        }

        let name = identity
            .map(|id| id.username.as_str())
            .unwrap_or("entity");

        let mut health_changed = false;

        // --- Process exhaustion ---
        // When exhaustion >= 4.0, drain saturation or hunger
        while hunger.exhaustion >= 4.0 {
            hunger.exhaustion -= 4.0;

            if hunger.saturation > 0.0 {
                // Drain saturation first
                hunger.saturation = (hunger.saturation - 1.0).max(0.0);
            } else if hunger.level > 0 {
                // Then drain hunger level
                hunger.level = hunger.level.saturating_sub(1);
            }
        }

        // --- Health regeneration ---
        // When hunger >= 18 and not at max health, regenerate
        if hunger.level >= 18 && health.current < health.max {
            health.current = (health.current + 1.0).min(health.max);
            hunger.exhaustion += 6.0; // Regen costs exhaustion
            health_changed = true;

            debug!(
                "{}: Regenerated 1 HP (hunger={}, health={:.1})",
                name, hunger.level, health.current
            );
        }

        // --- Starvation damage ---
        // When hunger == 0, deal starvation damage
        // In Normal mode, starvation stops at 1 HP
        // In Hard mode, starvation can kill
        if hunger.level == 0 && health.current > 1.0 {
            debug!(
                "{}: Starvation damage (health was {:.1})",
                name, health.current
            );

            // Emit starvation damage event
            damage_events.write(PlayerDamaged::starvation(entity));
            // Note: The actual damage is applied by damage_handler
            // We don't modify health here to keep the damage flow consistent
        }

        // --- Sync to client ---
        if health_changed {
            if let Some(writer) = writer {
                let packet = SetHealth {
                    health: health.current,
                    food: hunger.level.into(),
                    saturation: hunger.saturation,
                };
                if let Err(e) = writer.send_packet_ref(&packet) {
                    tracing::error!("Failed to send SetHealth packet: {:?}", e);
                }
            }
        }
    }
}

/// Add exhaustion to a player's hunger.
///
/// Call this when the player performs actions that consume hunger:
/// - Walking: 0.0 (none)
/// - Sprinting: 0.1 per meter
/// - Swimming: 0.01 per meter
/// - Breaking block: 0.005
/// - Jumping: 0.05 (0.2 while sprinting)
/// - Attacking: 0.1
/// - Taking damage: 0.1
/// - Hunger effect: 0.1 per second per level
pub fn add_exhaustion(hunger: &mut Hunger, amount: f32) {
    hunger.exhaustion = (hunger.exhaustion + amount).min(40.0);
}
