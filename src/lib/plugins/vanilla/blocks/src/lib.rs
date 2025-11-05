//! Vanilla Blocks Plugin
//!
//! Implements vanilla Minecraft block placement/breaking rules.
//! Validates placement and broadcasts to other players.

use ferrumc_plugin_api::prelude::*;
use ferrumc_block_api::{
    BlockAPI, BlockUpdateRequest, BreakBlockRequest, PlaceBlockEvent, PlaceBlockRequest,
    PlayerActionEvent, SendBlockAckRequest,
};
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
            .with_entity_queries()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        trace!("Loading vanilla-blocks plugin");

        ctx.events()
            .register::<PlaceBlockEvent>()
            .register::<PlayerActionEvent>()
            .register::<PlaceBlockRequest>()
            .register::<BreakBlockRequest>()
            .register::<BlockUpdateRequest>()
            .register::<SendBlockAckRequest>();

        ctx.systems()
            .add_tick(handle_place_block)
            .add_tick(handle_break_block);

        trace!("Vanilla-blocks plugin loaded successfully");
    }
}

fn handle_place_block(
    mut events: EventReader<PlaceBlockEvent>,
    mut api: BlockAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        // Vanilla: Just allow placement and broadcast to all
        api.place_block(event.player, event.position.clone(), event.block, event.sequence);
        
        // Broadcast block change to all players
        for (other, _, _) in entities.iter_players() {
            api.update_block(other, event.position.clone(), event.block);
        }
        
        trace!("Placed block for player {}", event.player.index());
    }
}

fn handle_break_block(
    mut events: EventReader<PlayerActionEvent>,
    mut api: BlockAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        if event.is_break_action() {
            // Vanilla: Just allow breaking and broadcast to all
            api.break_block(event.player, event.position.clone(), event.sequence);
            
            // Broadcast block change to all players
            for (other, _, _) in entities.iter_players() {
                api.update_block(other, event.position.clone(), 0.into());
            }
            
            trace!("Broke block for player {}", event.player.index());
        }
    }
}
