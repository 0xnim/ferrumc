use bevy_ecs::prelude::{Commands, Entity, Query, Res};
use ferrumc_core::chunks::initial_chunks_marker::NeedsInitialChunks;
use ferrumc_core::chunks::chunk_loading_rings::ChunkLoadPriority;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use tracing::{debug};

use crate::systems::chunk_loading_rings::add_missing_chunks_to_player;

/// System that processes newly connected players and sends remaining chunks
/// beyond the initial login chunks, in batches to avoid overwhelming the connection.
pub fn process_initial_chunk_loading(
    mut commands: Commands,
    mut query: Query<(Entity, &Position, &NeedsInitialChunks, &StreamWriter)>,
    _state: Res<GlobalStateResource>,
) {
    const INITIAL_CHUNK_RADIUS: i32 = 2; // Must match the radius used in login.rs
    
    for (entity, position, needs_chunks, _conn) in query.iter_mut() {
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

        debug!(
            "Using ring system to load remaining chunks after initial login: expanding from radius {} to {}",
            INITIAL_CHUNK_RADIUS, target_radius
        );

        // Use ring system for gradual loading of remaining chunks
        add_missing_chunks_to_player(
            &mut commands,
            entity,
            current_chunk,
            INITIAL_CHUNK_RADIUS as u32,
            target_radius as u32,
            ChunkLoadPriority::Medium, // Initial chunk expansion is medium priority
        );

        // Remove the marker since we've scheduled the chunk loading
        commands.entity(entity).remove::<NeedsInitialChunks>();
    }
}
