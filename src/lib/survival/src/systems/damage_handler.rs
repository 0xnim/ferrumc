//! Damage handler system that processes damage events.
//!
//! This system reads `PlayerDamaged` events, checks for damage immunity,
//! and applies the damage if allowed.

use bevy_ecs::prelude::*;
use ferrumc_api::components::DamageImmune;
use ferrumc_components::health::Health;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_messages::PlayerDamaged;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_health::SetHealth;
use tracing::{error, info};

/// System that handles damage events.
///
/// Flow:
/// 1. Reads `PlayerDamaged` events
/// 2. Skips entities with `DamageImmune` component
/// 3. Applies damage to Health component
/// 4. Sends `SetHealth` packet to update client UI
#[expect(clippy::type_complexity)]
pub fn handle_damage(
    mut events: MessageReader<PlayerDamaged>,
    mut query: Query<(
        &mut Health,
        Option<&PlayerIdentity>,
        Option<&StreamWriter>,
        Option<&DamageImmune>,
    )>,
) {
    for event in events.read() {
        let entity = event.player;
        let damage = event.amount;

        let Ok((mut health, identity, writer, immune)) = query.get_mut(entity) else {
            continue;
        };

        // Skip entities with damage immunity
        if immune.is_some() {
            let name = identity
                .map(|id| id.username.as_str())
                .unwrap_or("entity");
            info!("Damage prevented for {} (immune)", name);
            continue;
        }

        // Skip if already dead
        if health.current <= 0.0 {
            continue;
        }

        // Apply damage
        health.current = (health.current - damage).max(0.0);

        let name = identity
            .map(|id| id.username.as_str())
            .unwrap_or("entity");

        info!(
            "Damage applied: {} took {} damage, health: {}/{}",
            name, damage, health.current, health.max
        );

        // Send SetHealth packet
        if let Some(writer) = writer {
            let packet = SetHealth::new(health.current, 20, 5.0);
            if let Err(e) = writer.send_packet_ref(&packet) {
                error!("Failed to send SetHealth packet: {:?}", e);
            }
        }
    }
}
