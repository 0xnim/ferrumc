use bevy_ecs::prelude::{Query, Res};
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::chunks::chunk_receiver::PlayerRenderDistance;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_render_distance::SetRenderDistance;
use ferrumc_net::ClientInformationPlayReceiver;
use ferrumc_state::GlobalStateResource;
use std::collections::HashSet;
use tracing::{debug, error, trace};
use crate::systems::send_chunks::send_chunks;

pub fn handle_client_information(
    events: Res<ClientInformationPlayReceiver>,
    mut query: Query<(&mut StreamWriter, &mut PlayerRenderDistance, &Position)>,
    state: Res<GlobalStateResource>,
) {
    let config = get_global_config();
    
    for (packet, entity) in events.0.try_iter() {
        let new_effective_distance = config.get_effective_render_distance(packet.view_distance);
        
        let Ok((mut conn, mut current_render_distance, position)) = query.get_mut(entity) else {
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

        // Calculate chunk differences and send/unload as needed
        if new_effective_distance > old_distance {
            // Render distance increased - calculate and send the new chunks immediately
            let additional_chunks = get_additional_chunk_coords(
                current_chunk,
                old_distance as i32,
                new_effective_distance as i32,
            );
            debug!(
                "Render distance increased from {} to {} - sending {} additional chunks immediately",
                old_distance, new_effective_distance, additional_chunks.len()
            );
            
            // Send the additional chunks immediately
            if !additional_chunks.is_empty() {
                if let Err(e) = send_chunks(
                    state.0.clone(),
                    additional_chunks,
                    &mut conn,
                    current_chunk,
                ) {
                    error!("Failed to send additional chunks: {}", e);
                }
            }
        } else {
            // Render distance decreased - client should automatically unload the extra chunks
            debug!(
                "Render distance decreased from {} to {} - client will unload extra chunks",
                old_distance, new_effective_distance
            );
        }
    }
}

fn get_additional_chunk_coords(
    center_chunk: (i32, i32),
    old_radius: i32,
    new_radius: i32,
) -> Vec<(i32, i32, String)> {
    let mut old_chunk_seen = HashSet::new();
    let mut additional_chunks = Vec::new();

    // Calculate old visible chunks
    for x in center_chunk.0 - old_radius..=center_chunk.0 + old_radius {
        for z in center_chunk.1 - old_radius..=center_chunk.1 + old_radius {
            old_chunk_seen.insert((x, z));
        }
    }

    // Find new chunks that need to be sent (in new but not in old)
    for x in center_chunk.0 - new_radius..=center_chunk.0 + new_radius {
        for z in center_chunk.1 - new_radius..=center_chunk.1 + new_radius {
            if !old_chunk_seen.contains(&(x, z)) {
                additional_chunks.push((x, z, "overworld".to_string()));
            }
        }
    }

    additional_chunks
}
