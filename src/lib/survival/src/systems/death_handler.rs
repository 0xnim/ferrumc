//! Death handler system.
//!
//! Handles player death events:
//! - Adds DeadPlayer marker component
//! - Sends death screen to client
//! - Broadcasts death message

use bevy_ecs::prelude::*;
use ferrumc_components::health::Health;
use ferrumc_components::player::dead::DeadPlayer;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_messages::{DamageSource, PlayerDied};
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_health::SetHealth;
use ferrumc_net::packets::outgoing::system_message::SystemMessagePacket;
use ferrumc_nbt::NBT;
use ferrumc_text::{ComponentBuilder, NamedColor};
use tracing::{error, info};

/// System that handles PlayerDied events.
///
/// When a player dies:
/// 1. Adds DeadPlayer marker component
/// 2. Sends health=0 packet to show death screen
/// 3. Broadcasts death message to all players
pub fn handle_death(
    mut commands: Commands,
    mut events: MessageReader<PlayerDied>,
    mut query: Query<(
        &mut Health,
        Option<&PlayerIdentity>,
        Option<&StreamWriter>,
    )>,
    broadcast_query: Query<(&PlayerIdentity, &StreamWriter)>,
) {
    for event in events.read() {
        let entity = event.player;

        // Get player data
        let Ok((mut health, identity, writer)) = query.get_mut(entity) else {
            continue;
        };

        let player_name = identity
            .map(|id| id.username.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        info!("{} has died from {:?}", player_name, event.source);

        // Ensure health is at 0
        health.current = 0.0;

        // Capture source before event is consumed
        let death_source = event.source;
        let killer = event.killer;

        // Add DeadPlayer marker
        commands.entity(entity).insert(DeadPlayer { killer });

        // Send death screen to the player (health=0 triggers death screen)
        if let Some(writer) = writer {
            let packet = SetHealth::dead();
            if let Err(e) = writer.send_packet_ref(&packet) {
                error!("Failed to send death screen packet: {:?}", e);
            }
        }

        // Generate and broadcast death message
        let death_message = generate_death_message(&player_name, death_source, killer);
        broadcast_death_message(&broadcast_query, &death_message);
    }
}

/// Generate a death message based on the damage source.
fn generate_death_message(
    player_name: &str,
    source: DamageSource,
    _killer: Option<Entity>,
) -> String {
    match source {
        DamageSource::Fall { distance } => {
            if distance > 20.0 {
                format!("{} fell from a high place", player_name)
            } else {
                format!("{} hit the ground too hard", player_name)
            }
        }
        DamageSource::Starvation => {
            format!("{} starved to death", player_name)
        }
        DamageSource::Void => {
            format!("{} fell out of the world", player_name)
        }
        DamageSource::EntityAttack { .. } => {
            // TODO: Get killer name from entity
            format!("{} was slain", player_name)
        }
        DamageSource::Fire => {
            format!("{} burned to death", player_name)
        }
        DamageSource::Drowning => {
            format!("{} drowned", player_name)
        }
        DamageSource::Suffocation => {
            format!("{} suffocated in a wall", player_name)
        }
        DamageSource::Explosion { .. } => {
            format!("{} blew up", player_name)
        }
        DamageSource::Cactus => {
            format!("{} was pricked to death", player_name)
        }
        DamageSource::Magic => {
            format!("{} was killed by magic", player_name)
        }
        DamageSource::Generic => {
            format!("{} died", player_name)
        }
    }
}

/// Broadcast death message to all connected players.
fn broadcast_death_message(query: &Query<(&PlayerIdentity, &StreamWriter)>, message: &str) {
    // Build death message component (yellow text)
    let text_component = ComponentBuilder::text(message)
        .color(NamedColor::Yellow)
        .build();

    let packet = SystemMessagePacket {
        message: NBT::new(text_component),
        overlay: false,
    };

    for (_identity, writer) in query.iter() {
        if let Err(e) = writer.send_packet_ref(&packet) {
            error!("Failed to broadcast death message: {:?}", e);
        }
    }
}
