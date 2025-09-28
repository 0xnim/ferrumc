use bevy_ecs::prelude::{Commands, Query, Res};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::chunk_receiver::PlayerRenderDistance;
use ferrumc_core::chunks::chunk_loading_rings::ChunkLoadPriority;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_render_distance::SetRenderDistance;
use ferrumc_net::ClientInformationPlayReceiver;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error, trace};
use crate::systems::chunk_loading_rings::add_missing_chunks_to_player;

pub fn handle_client_information(
    mut commands: Commands,
    events: Res<ClientInformationPlayReceiver>,
    mut query: Query<(&mut StreamWriter, &mut PlayerRenderDistance, &Position)>,
    _state: Res<GlobalStateResource>,
) {
    let config = get_global_config();
    
    for (packet, entity) in events.0.try_iter() {
        let new_effective_distance = config.get_effective_render_distance(packet.view_distance);
        
        let Ok((conn, mut current_render_distance, position)) = query.get_mut(entity) else {
            error!("Failed to get entity components for entity {:?}", entity);
            continue;
        };

        let old_distance = current_render_distance.distance;

        trace!(
            "Client information update: view_distance {} -> effective: {} -> {} (clamped between {} and {})",
            packet.view_distance,
            old_distance,
            new_effective_distance,
            config.min_chunk_render_distance,
            config.max_chunk_render_distance
        );

        // If the render distance hasn't changed, no need to update chunks
        if old_distance == new_effective_distance {
            continue;
        }

        // Update the stored render distance
        current_render_distance.distance = new_effective_distance;

        // Send the SetRenderDistance packet to notify the client
        let set_distance_packet = SetRenderDistance::new(new_effective_distance);
        if let Err(e) = conn.send_packet(set_distance_packet) {
            error!("Failed to send render distance packet: {}", e);
            continue;
        }

        let current_chunk = ((position.x as i32) >> 4, (position.z as i32) >> 4);

        // Calculate chunk differences and handle loading/unloading
        if new_effective_distance > old_distance {
            // Render distance increased - use ring system for gradual loading
            debug!(
                "Render distance increased from {} to {} - scheduling gradual chunk loading",
                old_distance, new_effective_distance
            );
            
            // Add missing chunks using the ring system for gradual loading
            add_missing_chunks_to_player(
                &mut commands,
                entity,
                current_chunk,
                old_distance,
                new_effective_distance,
                ChunkLoadPriority::Medium, // Render distance change is medium priority
            );
        } else {
            // Render distance decreased - client should automatically unload the extra chunks
            debug!(
                "Render distance decreased from {} to {} - client will unload extra chunks",
                old_distance, new_effective_distance
            );
        }
    }
}


