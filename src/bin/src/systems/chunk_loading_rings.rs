use bevy_ecs::prelude::{Commands, Entity, Query, Res};
use ferrumc_core::chunks::chunk_loading_rings::{ChunkLoadingRings, ChunkLoadRequest};
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error, trace};

use crate::systems::send_chunks::send_chunks;

/// System that processes chunk loading rings for all players.
/// 
/// This system runs every tick and processes a limited number of chunks per player
/// to ensure gradual chunk loading that doesn't overwhelm the network or client.
pub fn process_chunk_loading_rings(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ChunkLoadingRings, &mut StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for (entity, mut rings, mut conn) in query.iter_mut() {
        // Skip inactive loading operations
        if !rings.active || !rings.has_pending_chunks() {
            // Remove the component if no more chunks to load
            if !rings.has_pending_chunks() {
                commands.entity(entity).remove::<ChunkLoadingRings>();
            }
            continue;
        }

        // Get the next batch of chunks to process
        let chunk_batch = rings.get_next_batch();
        
        if chunk_batch.is_empty() {
            continue;
        }

        trace!(
            "Processing {} chunks for player entity {:?} (priority: {:?}, {} remaining)",
            chunk_batch.len(),
            entity,
            chunk_batch.first().map(|c| c.priority),
            rings.pending_chunks.len()
        );

        // Convert chunk requests to the format expected by send_chunks
        let chunk_coords: Vec<(i32, i32, String)> = chunk_batch
            .into_iter()
            .map(|request| (request.chunk_x, request.chunk_z, request.dimension))
            .collect();

        // Send the batch of chunks
        if let Err(e) = send_chunks(
            state.0.clone(),
            chunk_coords,
            &mut conn,
            rings.center,
        ) {
            error!("Failed to send chunk batch for player entity {:?}: {}", entity, e);
            // Don't remove the component on error - we'll retry next tick
        }

        // If this was the last batch, the component will be removed next tick
        if !rings.has_pending_chunks() {
            debug!("Completed chunk loading for player entity {:?}", entity);
        }
    }
}

/// Helper function to add chunk loading rings to a player
pub fn add_chunk_loading_to_player(
    commands: &mut Commands,
    entity: Entity,
    center_chunk: (i32, i32),
    render_distance: u32,
    priority: ferrumc_core::chunks::chunk_loading_rings::ChunkLoadPriority,
) {
    let mut rings = ChunkLoadingRings::new(center_chunk, render_distance);
    rings.add_ring_chunks(
        center_chunk,
        render_distance,
        priority,
        "overworld".to_string(),
    );

    commands.entity(entity).insert(rings);
    
    debug!(
        "Added chunk loading rings for entity {:?}: center=({}, {}), radius={}, priority={:?}",
        entity, center_chunk.0, center_chunk.1, render_distance, priority
    );
}

/// Helper function to add missing chunks between two render distances
pub fn add_missing_chunks_to_player(
    commands: &mut Commands,
    entity: Entity,
    center_chunk: (i32, i32),
    old_radius: u32,
    new_radius: u32,
    priority: ferrumc_core::chunks::chunk_loading_rings::ChunkLoadPriority,
) {
    // Check if player already has a ChunkLoadingRings component
    let mut rings = ChunkLoadingRings::new(center_chunk, new_radius);
    rings.add_missing_chunks(
        center_chunk,
        old_radius,
        new_radius,
        priority,
        "overworld".to_string(),
    );

    commands.entity(entity).insert(rings);
    
    debug!(
        "Added missing chunks for entity {:?}: center=({}, {}), old_radius={}, new_radius={}, priority={:?}",
        entity, center_chunk.0, center_chunk.1, old_radius, new_radius, priority
    );
}

/// Add chunks needed for cross-chunk boundary movement
pub fn add_boundary_chunks_to_player(
    commands: &mut Commands,
    entity: Entity,
    old_chunk: (i32, i32),
    new_chunk: (i32, i32),
    render_distance: u32,
    priority: ferrumc_core::chunks::chunk_loading_rings::ChunkLoadPriority,
) {
    let mut rings = ChunkLoadingRings::new(new_chunk, render_distance);
    
    // Calculate which chunks are needed for the new position but weren't needed for the old
    use std::collections::HashSet;
    
    let mut old_chunks = HashSet::new();
    let radius = render_distance as i32;
    
    // Calculate old visible chunks
    for x in old_chunk.0 - radius..=old_chunk.0 + radius {
        for z in old_chunk.1 - radius..=old_chunk.1 + radius {
            old_chunks.insert((x, z));
        }
    }
    
    // Find new chunks that need to be sent
    let mut needed_chunks = Vec::new();
    for x in new_chunk.0 - radius..=new_chunk.0 + radius {
        for z in new_chunk.1 - radius..=new_chunk.1 + radius {
            if !old_chunks.contains(&(x, z)) {
                needed_chunks.push((x, z));
            }
        }
    }
    
    // Add chunks to the ring loader
    for (x, z) in needed_chunks {
        rings.pending_chunks.push_back(ChunkLoadRequest {
            chunk_x: x,
            chunk_z: z,
            dimension: "overworld".to_string(),
            priority,
            ring: 0, // Not important for boundary chunks
        });
    }
    
    if rings.has_pending_chunks() {
        let num_chunks = rings.pending_chunks.len();
        rings.active = true;
        commands.entity(entity).insert(rings);
        
        debug!(
            "Added boundary chunks for entity {:?}: old=({}, {}), new=({}, {}), {} chunks needed",
            entity, old_chunk.0, old_chunk.1, new_chunk.0, new_chunk.1, num_chunks
        );
    }
}
