//! Core systems for handling block placement and breaking
//!
//! These systems handle the I/O operations (chunk loading/saving)
//! that plugins should not directly access.

use bevy_ecs::prelude::*;
use ferrumc_block_api::{
    BreakBlockRequest, PlaceBlockRequest, SendBlockChangeAckRequest, SendBlockUpdateRequest,
};
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use std::sync::Arc;
use tracing::{debug, error};

/// Core system: Handle block placement requests from plugins
///
/// This system performs the actual I/O:
/// - Loads the chunk from database
/// - Modifies the block
/// - Saves the chunk back to database
/// - Broadcasts the update to players
///
/// Plugins should never do this directly!
pub fn handle_place_block_requests(
    mut events: EventReader<PlaceBlockRequest>,
    mut ack_events: EventWriter<SendBlockChangeAckRequest>,
    mut update_events: EventWriter<SendBlockUpdateRequest>,
    state: Res<GlobalStateResource>,
) {
    for request in events.read() {
        // Calculate the position offset based on the request
        let (x, y, z) = (
            request.position.x,
            request.position.y,
            request.position.z,
        );

        // Load the chunk (I/O operation - belongs in core)
        let mut chunk = match state
            .0
            .world
            .load_chunk_owned(x >> 4, z >> 4, "overworld")
        {
            Ok(chunk) => chunk,
            Err(e) => {
                error!("Failed to load chunk for block placement: {:?}", e);
                continue;
            }
        };

        // Set the block in the chunk
        if let Err(err) = chunk.set_block(x & 0xF, y as i32, z & 0xF, request.block) {
            error!("Failed to set block: {:?}", err);
            continue;
        }

        // Save the chunk (I/O operation - belongs in core)
        if let Err(err) = state.0.world.save_chunk(Arc::from(chunk)) {
            error!("Failed to save chunk after block placement: {:?}", err);
        } else {
            debug!("Block placed at ({}, {}, {})", x, y, z);
        }

        // Send acknowledgment to the player
        ack_events.write(SendBlockChangeAckRequest {
            player: request.player,
            sequence: request.sequence,
        });

        // Broadcast block update to all players
        update_events.write(SendBlockUpdateRequest {
            position: request.position.clone(),
            block: request.block,
            exclude_player: None,
        });
    }
}

/// Core system: Handle block breaking requests from plugins
///
/// This system performs the actual I/O:
/// - Loads the chunk from database
/// - Sets block to air
/// - Saves the chunk back to database
/// - Broadcasts the update to players
///
/// Plugins should never do this directly!
pub fn handle_break_block_requests(
    mut events: EventReader<BreakBlockRequest>,
    mut ack_events: EventWriter<SendBlockChangeAckRequest>,
    mut update_events: EventWriter<SendBlockUpdateRequest>,
    state: Res<GlobalStateResource>,
) {
    for request in events.read() {
        let (x, y, z) = (
            request.position.x,
            request.position.y,
            request.position.z,
        );

        // Load or generate the chunk (I/O operation - belongs in core)
        let mut chunk = match state
            .0
            .world
            .load_chunk_owned(x >> 4, z >> 4, "overworld")
        {
            Ok(chunk) => chunk,
            Err(_) => {
                // Try to generate chunk if it doesn't exist
                match state.0.terrain_generator.generate_chunk(x >> 4, z >> 4) {
                    Ok(chunk) => chunk,
                    Err(e) => {
                        error!("Failed to generate chunk: {:?}", e);
                        continue;
                    }
                }
            }
        };

        // Set block to air (default block state)
        if let Err(err) = chunk.set_block(x & 0xF, y as i32, z & 0xF, BlockStateId::default()) {
            error!("Failed to break block: {:?}", err);
            continue;
        }

        // Save the chunk (I/O operation - belongs in core)
        if let Err(err) = state.0.world.save_chunk(Arc::new(chunk)) {
            error!("Failed to save chunk after block break: {:?}", err);
        } else {
            debug!("Block broken at ({}, {}, {})", x, y, z);
        }

        // Send acknowledgment to the player
        ack_events.write(SendBlockChangeAckRequest {
            player: request.player,
            sequence: request.sequence,
        });

        // Broadcast block update to all players (set to air)
        update_events.write(SendBlockUpdateRequest {
            position: request.position.clone(),
            block: BlockStateId::default(),
            exclude_player: None,
        });
    }
}
