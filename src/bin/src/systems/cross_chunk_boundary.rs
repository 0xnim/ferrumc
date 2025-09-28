use crate::systems::chunk_loading_rings::add_boundary_chunks_to_player;
use bevy_ecs::prelude::{Commands, EventReader, Query, Res};
use ferrumc_core::chunks::chunk_receiver::PlayerRenderDistance;
use ferrumc_core::chunks::chunk_loading_rings::ChunkLoadPriority;
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use ferrumc_net::connection::StreamWriter;
use ferrumc_state::GlobalStateResource;

pub fn cross_chunk_boundary(
    mut commands: Commands,
    mut events: EventReader<CrossChunkBoundaryEvent>,
    mut query: Query<(&mut StreamWriter, &PlayerRenderDistance)>,
    state: Res<GlobalStateResource>,
) {
    if events.is_empty() {
        return;
    }
    for event in events.read() {
        if !state.0.players.is_connected(event.player) {
            continue; // Skip if the player is not connected
        }
        
        let (_conn, render_distance) = query.get_mut(event.player).expect("Player does not exist");
        
        // Use ring system for gradual boundary chunk loading
        add_boundary_chunks_to_player(
            &mut commands,
            event.player,
            event.old_chunk,
            event.new_chunk,
            render_distance.distance,
            ChunkLoadPriority::High, // Movement chunks are high priority
        );
    }
}
