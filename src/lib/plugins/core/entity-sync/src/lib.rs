//! Entity Sync Plugin (Core)
//!
//! **Purpose:** Updates server-side entity state from network packets.
//! **Does NOT broadcast** to other players - that's vanilla's job.
//!
//! This plugin:
//! - Listens to movement/rotation events from packet handlers
//! - Updates Position/Rotation/OnGround ECS components
//! - Fires cross-chunk events
//! - NO validation, NO broadcasting

use ferrumc_plugin_api::prelude::*;
use ferrumc_movement_api::{
    ApplyMovementRequest, PlayerMoveAndRotateEvent, PlayerMoveEvent, PlayerRotateEvent,
};
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use tracing::trace;

mod position;
mod rotation;

#[derive(Default)]
pub struct EntitySyncPlugin;

impl Plugin for EntitySyncPlugin {
    fn name(&self) -> &'static str {
        "entity-sync"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Core entity state synchronization - updates server state from packets (no broadcasting)"
    }

    fn priority(&self) -> i32 {
        1000 // Very high - update state before vanilla logic runs
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_movement_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        trace!("Loading entity-sync plugin");

        // Register events
        ctx.events()
            .register::<PlayerMoveEvent>()
            .register::<PlayerRotateEvent>()
            .register::<PlayerMoveAndRotateEvent>()
            .register::<ApplyMovementRequest>()
            .register::<CrossChunkBoundaryEvent>();

        // Register state update systems
        ctx.systems()
            .add_tick(position::update_position_from_move)
            .add_tick(position::update_position_and_rotation_from_move_rotate)
            .add_tick(rotation::update_rotation_from_rotate);

        trace!("Entity-sync plugin loaded successfully");
    }
}
