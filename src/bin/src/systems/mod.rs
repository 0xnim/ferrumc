pub mod connection_killer;
mod cross_chunk_boundary;
mod initial_chunk_loading;
pub mod keep_alive_system;
mod mq;
pub mod new_connections;
pub mod player_count_update;
pub mod send_chunks;
pub mod shutdown_systems;
pub mod world_sync;

pub fn register_game_systems(schedule: &mut bevy_ecs::schedule::Schedule) {
    // Tick-bound systems only (run every game tick)
    schedule.add_systems(new_connections::accept_new_connections);
    schedule.add_systems(initial_chunk_loading::process_initial_chunk_loading);
    schedule.add_systems(cross_chunk_boundary::cross_chunk_boundary);
    schedule.add_systems(mq::process);

    // Should always be last
    schedule.add_systems(connection_killer::connection_killer);
}
