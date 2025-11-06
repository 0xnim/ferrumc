//! Vanilla Spawn Safety Plugin
//!
//! Ensures players don't spawn inside blocks by checking their spawn position
//! and teleporting them to a safe location if needed.

use ferrumc_plugin_api::prelude::*;
use ferrumc_join_leave_api::PlayerJoinEvent;
use ferrumc_movement_api::MovementAPI;
use ferrumc_core::transform::position::Position;
use ferrumc_world::block_state_id::BlockStateId;
use tracing::{info, warn};

#[derive(Default)]
pub struct VanillaSpawnSafetyPlugin;

impl Plugin for VanillaSpawnSafetyPlugin {
    fn name(&self) -> &'static str {
        "vanilla-spawn-safety"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Ensures players spawn in safe locations (not inside blocks)"
    }

    fn priority(&self) -> i32 {
        100 // Run after join-leave messages (priority 20)
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_entity_queries()
            .with_world_queries()
            .with_movement_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        ctx.systems().add_tick(check_spawn_safety);
    }
}

fn check_spawn_safety(
    mut events: EventReader<PlayerJoinEvent>,
    entities: EntityQueries,
    world: WorldQueries,
    mut movement: MovementAPI,
) {
    for event in events.read() {
        let Some(pos) = entities.position(event.joining_player) else {
            warn!("Player position not found for entity {:?}", event.joining_player);
            continue;
        };

        // Check the block at the player's head position (where they spawned)
        let head_block = world.get_block(
            pos.x as i32,
            pos.y as i32,
            pos.z as i32,
        );

        match head_block {
            Some(block_id) if block_id == BlockStateId(0) => {
                // Air block - safe spawn
                info!(
                    "Player {} loaded at safe position: ({}, {}, {})",
                    event.identity.username,
                    pos.x,
                    pos.y,
                    pos.z
                );
            }
            Some(block_id) => {
                // Not air - player spawned in a block!
                info!(
                    "Player {} loaded at position: ({}, {}, {}) with head block: {:?}",
                    event.identity.username,
                    pos.x,
                    pos.y,
                    pos.z,
                    block_id
                );
                
                // Teleport the player to the world center (spawn) using MovementAPI
                let spawn_position = Position::new(0.0, 64.0, 0.0);
                movement.teleport(event.joining_player, spawn_position, None, None);
                
                info!(
                    "Teleported player {} to spawn position",
                    event.identity.username
                );
            }
            None => {
                warn!(
                    "Failed to fetch head block for player {} at position: ({}, {}, {})",
                    event.identity.username, pos.x, pos.y, pos.z
                );
            }
        }
    }
}
