//! Combat system.
//!
//! Handles player vs player/entity combat:
//! - Attack cooldown tracking
//! - Damage calculation with cooldown multiplier
//! - Critical hit detection
//! - Hurt animation broadcasting

use bevy_ecs::prelude::*;
use ferrumc_components::health::Health;
use ferrumc_components::player::attack_cooldown::AttackCooldown;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_messages::PlayerDamaged;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::hurt_animation::HurtAnimationPacket;
use ferrumc_net::InteractEntityReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error, trace};

/// Base damage for unarmed attacks (in half-hearts).
const BASE_HAND_DAMAGE: f32 = 1.0;

/// System that handles interact packets for combat.
///
/// When a player attacks another entity:
/// 1. Calculate damage based on cooldown progress
/// 2. Check for critical hit (falling + not on ground)
/// 3. Emit PlayerDamaged event
/// 4. Reset attacker cooldown
/// 5. Broadcast hurt animation
pub fn handle_combat(
    receiver: Res<InteractEntityReceiver>,
    state: Res<GlobalStateResource>,
    mut attacker_query: Query<(
        Entity,
        &mut AttackCooldown,
        &OnGround,
        &Rotation,
        Option<&PlayerIdentity>,
    )>,
    target_query: Query<(&Health, Option<&PlayerIdentity>)>,
    broadcast_query: Query<&StreamWriter>,
    mut damage_events: MessageWriter<PlayerDamaged>,
) {
    for (packet, attacker_entity) in receiver.0.try_iter() {
        // Only process attack interactions
        if !packet.is_attack() {
            trace!("Ignoring non-attack interaction from {:?}", attacker_entity);
            continue;
        }

        // Check if attacker is connected
        if !state.0.players.is_connected(attacker_entity) {
            continue;
        }

        // Get attacker data
        let Ok((attacker, mut cooldown, on_ground, rotation, attacker_identity)) =
            attacker_query.get_mut(attacker_entity)
        else {
            continue;
        };

        let attacker_name = attacker_identity
            .map(|id| id.username.as_str())
            .unwrap_or("entity");

        // Find the target entity by their network entity ID
        // The packet.entity_id is the Minecraft protocol entity ID (== entity.index())
        let target_entity_id = packet.entity_id.0 as u32;

        // Look up the bevy Entity from the protocol entity ID
        // We need to find an entity in the player list whose index matches
        let target_entity = state
            .0
            .players
            .player_list
            .iter()
            .find(|entry| entry.key().index() == target_entity_id)
            .map(|entry| *entry.key());

        let Some(target_entity) = target_entity else {
            debug!(
                "{} attacked unknown entity ID {}",
                attacker_name, target_entity_id
            );
            continue;
        };

        // Skip if attacking self
        if target_entity == attacker {
            trace!("{} tried to attack themselves", attacker_name);
            continue;
        }

        // Check if target exists and has health
        let Ok((target_health, target_identity)) = target_query.get(target_entity) else {
            debug!("{} attacked entity without health", attacker_name);
            continue;
        };

        // Skip if target is already dead
        if target_health.current <= 0.0 {
            trace!("{} attacked dead entity", attacker_name);
            continue;
        }

        let target_name = target_identity
            .map(|id| id.username.as_str())
            .unwrap_or("entity");

        // --- Calculate damage ---

        // Get base damage (TODO: get from held item)
        let base_damage = BASE_HAND_DAMAGE;

        // Apply cooldown multiplier
        let cooldown_multiplier = cooldown.damage_multiplier();
        let mut damage = base_damage * cooldown_multiplier;

        // Check for critical hit (falling and not on ground)
        // TODO: Check if player is actually falling (velocity.y < 0)
        let is_critical = !on_ground.0;

        if is_critical {
            damage *= 1.5;
            debug!("{} landed a critical hit on {}!", attacker_name, target_name);
        }

        debug!(
            "{} attacks {} for {:.1} damage (cooldown={:.0}%, crit={})",
            attacker_name,
            target_name,
            damage,
            cooldown_multiplier * 100.0,
            is_critical
        );

        // Emit damage event
        damage_events.write(PlayerDamaged::attack(target_entity, attacker, damage));

        // Reset attacker cooldown
        cooldown.reset();

        // Broadcast hurt animation to all nearby players
        let hurt_packet = HurtAnimationPacket::new(target_entity_id as i32, rotation.yaw);
        for writer in broadcast_query.iter() {
            if let Err(e) = writer.send_packet_ref(&hurt_packet) {
                error!("Failed to send hurt animation: {:?}", e);
            }
        }
    }
}

/// System that ticks attack cooldowns each game tick.
pub fn tick_cooldowns(mut query: Query<&mut AttackCooldown>) {
    for mut cooldown in query.iter_mut() {
        cooldown.tick();
    }
}
