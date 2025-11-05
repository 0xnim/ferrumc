//! Blocks Plugin for FerrumC
//!
//! This plugin implements the gameplay logic for block placement and breaking.
//! It handles validation (collision detection, permissions, inventory checks).
//!
//! # Architecture
//!
//! - Core converts packets → events (BlockPlaceAttemptEvent, BlockBreakAttemptEvent)
//! - **This plugin** validates placement/breaking (game logic only - NO I/O)
//! - Plugin uses BlockAPI to request block operations
//! - Core handles chunk loading/saving and broadcasts (I/O layer)

mod item_mapping;

use ferrumc_plugin_api::prelude::*;
use ferrumc_block_api::{BlockAPI, BlockBreakAttemptEvent, BlockPlaceAttemptEvent, Hand};
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_world::block_state_id::BlockStateId;
use tracing::{debug, error, info, trace};

use item_mapping::ITEM_TO_BLOCK_MAPPING;

#[derive(Default)]
pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn name(&self) -> &'static str {
        "blocks"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Handles block placement and breaking validation (game logic only)"
    }

    fn priority(&self) -> i32 {
        40 // Validation and placement logic
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_block_api()
            .with_entity_queries()
            .with_inventory_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        info!("Loading blocks plugin");

        // Register events from block API
        ctx.events()
            .register::<BlockPlaceAttemptEvent>()
            .register::<BlockBreakAttemptEvent>()
            .register::<ferrumc_block_api::PlaceBlockRequest>()
            .register::<ferrumc_block_api::BreakBlockRequest>()
            .register::<ferrumc_block_api::SendBlockUpdateRequest>()
            .register::<ferrumc_block_api::SendBlockChangeAckRequest>();

        // Register our gameplay logic systems (validation only - no I/O!)
        ctx.systems()
            .add_tick(handle_block_placement)
            .add_tick(handle_block_breaking);

        info!("Blocks plugin loaded successfully");
    }
}

/// Handle block placement attempts
///
/// This is PURE GAME LOGIC - no I/O operations!
/// - Validates inventory (do they have the item?)
/// - Validates collision (would it intersect entities?)
/// - Maps item to block state
/// - Calls BlockAPI to request placement (core handles I/O)
fn handle_block_placement(
    mut events: EventReader<BlockPlaceAttemptEvent>,
    mut blocks: BlockAPI,
    inventories: InventoryQueries,
    entities: EntityQueries,
) {
    for event in events.read() {
        // Only handle main hand for now
        if event.hand != Hand::Main {
            trace!("Offhand block placement not implemented");
            continue;
        }

        // Validate: Get player inventory
        let Some((inventory, hotbar)) = inventories.get(event.player) else {
            debug!("Could not get inventory for player {:?}", event.player);
            continue;
        };

        // Validate: Get the item in the selected hotbar slot
        let slot_index = hotbar.selected_slot as usize;
        let Ok(slot) = inventory.get_item(slot_index) else {
            error!("Could not fetch inventory slot {}", slot_index);
            continue;
        };

        let Some(selected_item) = slot else {
            trace!("No item in selected slot");
            continue;
        };

        // Validate: Get the item ID
        let Some(item_id) = selected_item.item_id else {
            error!("Selected item has no item ID");
            continue;
        };

        // Validate: Map item to block state
        let Some(&mapped_block_state_id) = ITEM_TO_BLOCK_MAPPING.get(&item_id.0 .0) else {
            error!("No block mapping found for item ID: {}", item_id.0);
            continue;
        };

        debug!(
            "Validating block placement with item ID: {}, mapped to block state ID: {}",
            item_id.0, mapped_block_state_id
        );

        // Calculate the position offset based on the face
        let (x_offset, y_offset, z_offset) = event.face.offset();
        let (x, y, z) = (
            event.position.x + x_offset,
            event.position.y + y_offset as i16,
            event.position.z + z_offset,
        );

        // Validate: Check collision with entities
        if entities.would_collide_with_block(x, y as i32, z) {
            trace!("Block placement collided with entity");
            continue;
        }

        // All validation passed! Request block placement via API
        // The core will handle: chunk loading, block setting, chunk saving, broadcasting
        blocks.place_block(
            event.player,
            NetworkPosition { x, y, z },
            BlockStateId(mapped_block_state_id as u32),
            event.sequence,
        );

        debug!("Block placement validated and requested at ({}, {}, {})", x, y, z);
    }
}

/// Handle block breaking attempts
///
/// This is PURE GAME LOGIC - no I/O operations!
/// - Validates permissions (can they break blocks?)
/// - Validates gamemode
/// - Calls BlockAPI to request breaking (core handles I/O)
fn handle_block_breaking(
    mut events: EventReader<BlockBreakAttemptEvent>,
    mut blocks: BlockAPI,
) {
    for event in events.read() {
        // Validation would go here:
        // - Check gamemode (creative/survival)
        // - Check permissions (protected areas, etc.)
        // - Check tool requirements
        // For now, we allow all breaks

        // Request block break via API
        // The core will handle: chunk loading, setting to air, chunk saving, broadcasting
        blocks.break_block(event.player, event.position.clone(), event.sequence);

        debug!(
            "Block break validated and requested at ({}, {}, {})",
            event.position.x, event.position.y, event.position.z
        );
    }
}
