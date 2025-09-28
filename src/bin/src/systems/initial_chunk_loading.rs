use bevy_ecs::prelude::{Commands, Entity, Query, Res};
use ferrumc_core::chunks::initial_chunks_marker::NeedsInitialChunks;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use std::collections::HashSet;
use tracing::{debug, error};

use crate::systems::send_chunks::send_chunks;

/// System that processes newly connected players and sends remaining chunks
/// beyond the initial login chunks, in batches to avoid overwhelming the connection.
pub fn process_initial_chunk_loading(
    mut commands: Commands,
    mut query: Query<(Entity, &Position, &NeedsInitialChunks, &mut StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    const INITIAL_CHUNK_RADIUS: i32 = 2; // Must match the radius used in login.rs
    
    for (entity, position, needs_chunks, mut conn) in query.iter_mut() {
        let current_chunk = ((position.x as i32) >> 4, (position.z as i32) >> 4);
        let target_radius = needs_chunks.target_radius as i32;
        
        debug!(
            "Loading remaining chunks for player at chunk ({}, {}) - expanding from radius {} to {}",
            current_chunk.0, current_chunk.1, INITIAL_CHUNK_RADIUS, target_radius
        );

        if target_radius <= INITIAL_CHUNK_RADIUS {
            // No additional chunks needed
            commands.entity(entity).remove::<NeedsInitialChunks>();
            continue;
        }

        // Calculate which chunks are already sent (initial chunks)
        let mut initial_chunks = HashSet::new();
        for x in current_chunk.0 - INITIAL_CHUNK_RADIUS..=current_chunk.0 + INITIAL_CHUNK_RADIUS {
            for z in current_chunk.1 - INITIAL_CHUNK_RADIUS..=current_chunk.1 + INITIAL_CHUNK_RADIUS {
                initial_chunks.insert((x, z));
            }
        }

        // Calculate which chunks need to be sent (full radius minus initial chunks)
        let mut needed_chunks = Vec::new();
        for x in current_chunk.0 - target_radius..=current_chunk.0 + target_radius {
            for z in current_chunk.1 - target_radius..=current_chunk.1 + target_radius {
                if !initial_chunks.contains(&(x, z)) {
                    needed_chunks.push((x, z, "overworld".to_string()));
                }
            }
        }

        if !needed_chunks.is_empty() {
            debug!("Sending {} additional chunks after initial login", needed_chunks.len());
            
            if let Err(e) = send_chunks(state.0.clone(), needed_chunks, &mut conn, current_chunk) {
                error!("Failed to send additional chunks for player: {}", e);
            }
        }

        // Remove the marker component as we've processed this player
        commands.entity(entity).remove::<NeedsInitialChunks>();
    }
}
