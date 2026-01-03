//! Respawn handler system.
//!
//! Handles respawn requests from dead players:
//! - Resets health and hunger
//! - Teleports to spawn point
//! - Removes DeadPlayer marker
//! - Sends respawn packet
//! - Triggers chunk reload

use bevy_ecs::prelude::*;
use ferrumc_components::health::Health;
use ferrumc_components::player::dead::DeadPlayer;
use ferrumc_components::player::gamemode::GameModeComponent;
use ferrumc_components::player::hunger::Hunger;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::transform::position::Position;
use ferrumc_messages::chunk_calc::ChunkCalc;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::game_event::GameEventPacket;
use ferrumc_net::packets::outgoing::respawn::RespawnPacket;
use ferrumc_net::packets::outgoing::set_default_spawn_position::DEFAULT_SPAWN_POSITION;
use ferrumc_net::packets::outgoing::set_health::SetHealth;
use ferrumc_net::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket;
use ferrumc_net::ClientCommandReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error, info};

/// System that handles client respawn requests.
///
/// When a dead player presses the respawn button:
/// 1. Reset health to 20, hunger to 20, saturation to 5
/// 2. Teleport to spawn point
/// 3. Send Respawn packet
/// 4. Send position sync
/// 5. Send health update
/// 6. Remove DeadPlayer marker
/// 7. Trigger chunk reload
#[expect(clippy::type_complexity)]
pub fn handle_respawn_request(
    receiver: Res<ClientCommandReceiver>,
    state: Res<GlobalStateResource>,
    mut commands: Commands,
    mut chunk_calc: MessageWriter<ChunkCalc>,
    mut query: Query<(
        &mut Health,
        &mut Hunger,
        &mut Position,
        &mut ChunkReceiver,
        &GameModeComponent,
        Option<&DeadPlayer>,
        Option<&StreamWriter>,
    )>,
) {
    for (packet, entity) in receiver.0.try_iter() {
        // Only handle respawn requests (action = 0)
        if !packet.is_respawn_request() {
            continue;
        }

        // Check if player is connected
        if !state.0.players.is_connected(entity) {
            continue;
        }

        // Get player components
        let Ok((mut health, mut hunger, mut position, mut chunk_receiver, gamemode, dead_marker, writer)) =
            query.get_mut(entity)
        else {
            continue;
        };

        // Only respawn if actually dead
        if dead_marker.is_none() {
            debug!("Respawn request from non-dead player {:?}, ignoring", entity);
            continue;
        }

        info!("Player {:?} is respawning", entity);

        // --- 1. Reset health ---
        health.current = health.max;

        // --- 2. Reset hunger ---
        hunger.level = 20;
        hunger.saturation = 5.0;
        hunger.exhaustion = 0.0;

        // --- 3. Get spawn position ---
        // TODO: Use player's spawn point (bed) if available
        let spawn_pos = Position::new(
            f64::from(DEFAULT_SPAWN_POSITION.x),
            f64::from(DEFAULT_SPAWN_POSITION.y),
            f64::from(DEFAULT_SPAWN_POSITION.z),
        );

        *position = spawn_pos;

        // --- 4. Send packets to client ---
        if let Some(writer) = writer {
            // Send Respawn packet (resets client state)
            let respawn_packet = RespawnPacket::same_dimension(gamemode.0 as u8);
            if let Err(e) = writer.send_packet_ref(&respawn_packet) {
                error!("Failed to send Respawn packet: {:?}", e);
            }

            // Tell client to start waiting for level chunks
            // This is required after respawn in the same dimension
            let game_event = GameEventPacket::start_waiting_for_level_chunks();
            if let Err(e) = writer.send_packet_ref(&game_event) {
                error!("Failed to send GameEvent packet: {:?}", e);
            }

            // Send position sync
            let pos_packet = SynchronizePlayerPositionPacket::new(
                (spawn_pos.x, spawn_pos.y, spawn_pos.z),
                (0.0, 0.0, 0.0), // velocity
                0.0,            // yaw
                0.0,            // pitch
                0,              // flags (absolute positioning)
                VarInt::new(0), // teleport ID
            );
            if let Err(e) = writer.send_packet_ref(&pos_packet) {
                error!("Failed to send position sync: {:?}", e);
            }

            // Send health update
            let health_packet = SetHealth {
                health: health.current,
                food: hunger.level.into(),
                saturation: hunger.saturation,
            };
            if let Err(e) = writer.send_packet_ref(&health_packet) {
                error!("Failed to send health update: {:?}", e);
            }
        }

        // --- 5. Remove DeadPlayer marker ---
        commands.entity(entity).remove::<DeadPlayer>();

        // --- 6. Reset chunk receiver and trigger chunk reload ---
        // After respawn, the client no longer has any chunks loaded
        chunk_receiver.loaded.clear();
        chunk_receiver.loading.clear();
        chunk_receiver.unloading.clear();
        chunk_receiver.dirty.clear();

        // Trigger chunk recalculation to send chunks around spawn
        chunk_calc.write(ChunkCalc(entity));

        debug!(
            "Player {:?} respawned at ({}, {}, {})",
            entity, spawn_pos.x, spawn_pos.y, spawn_pos.z
        );
    }
}
