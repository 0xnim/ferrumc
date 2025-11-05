//! Vanilla Movement Plugin
//!
//! **Purpose:** Implements vanilla Minecraft movement behavior.
//!
//! This plugin:
//! - Validates movement (detect teleports, speed hacks)
//! - Decides who can see movement updates
//! - Broadcasts position/rotation changes to other players
//! - All the game logic that entity-sync doesn't do

use ferrumc_plugin_api::prelude::*;
use ferrumc_movement_api::{
    BroadcastHeadRotationRequest, BroadcastMovementRequest, HeadRotationEvent,
    PlayerMoveAndRotateEvent, PlayerMoveEvent, PlayerRotateEvent,
};
use tracing::trace;

mod broadcasting;
mod validation;

#[derive(Default)]
pub struct VanillaMovementPlugin;

impl Plugin for VanillaMovementPlugin {
    fn name(&self) -> &'static str {
        "vanilla-movement"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Vanilla Minecraft movement behavior - validation and broadcasting"
    }

    fn priority(&self) -> i32 {
        45 // After entity-sync (1000), before most other plugins
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_movement_api()
            .with_entity_queries()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        trace!("Loading vanilla-movement plugin");

        // Register vanilla behavior systems
        ctx.systems()
            .add_tick(broadcasting::broadcast_position_updates)
            .add_tick(broadcasting::broadcast_rotation_updates)
            .add_tick(broadcasting::broadcast_combined_updates)
            .add_tick(broadcasting::broadcast_head_rotation);

        trace!("Vanilla-movement plugin loaded successfully");
    }
}
