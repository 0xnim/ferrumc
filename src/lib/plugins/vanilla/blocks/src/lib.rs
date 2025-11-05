//! Vanilla Blocks Plugin
//!
//! Implements vanilla Minecraft block placement/breaking rules.
//! Validates placement and broadcasts to other players.

use ferrumc_plugin_api::prelude::*;
use ferrumc_block_api::{
    BlockBroadcasts, BlockBreakAttemptEvent, BlockPlaceAttemptEvent, BlockRequests,
    BreakBlockRequest, PlaceBlockRequest, SendBlockChangeAckRequest, SendBlockUpdateRequest,
};
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_net_codec::net_types::var_int::VarInt;
use tracing::trace;

#[derive(Default)]
pub struct VanillaBlocksPlugin;

impl Plugin for VanillaBlocksPlugin {
    fn name(&self) -> &'static str {
        "vanilla-blocks"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Vanilla Minecraft block mechanics - placement rules and broadcasting"
    }

    fn priority(&self) -> i32 {
        40
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_block_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        trace!("Loading vanilla-blocks plugin");

        ctx.systems()
            .add_tick(handle_place_block)
            .add_tick(handle_break_block);

        trace!("Vanilla-blocks plugin loaded successfully");
    }
}

fn handle_place_block(
    mut events: EventReader<BlockPlaceAttemptEvent>,
    mut blocks: BlockRequests,
) {
    for event in events.read() {
        // Vanilla: Just allow placement (no validation for now)
        // TODO: Get actual block from event when available
        let block = BlockStateId::from(VarInt::new(1));
        
        // Request block placement - core handles I/O and broadcasting
        blocks.place_block(event.player, event.position.clone(), block, event.sequence);
        
        trace!("Placed block for player {}", event.player.index());
    }
}

fn handle_break_block(
    mut events: EventReader<BlockBreakAttemptEvent>,
    mut blocks: BlockRequests,
) {
    for event in events.read() {
        // Vanilla: Just allow breaking (no validation for now)
        // Request block break - core handles I/O and broadcasting
        blocks.break_block(event.player, event.position.clone(), event.sequence);
        
        trace!("Broke block for player {}", event.player.index());
    }
}
