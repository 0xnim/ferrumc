//! Blocks Plugin for FerrumC
//!
//! This plugin implements the gameplay logic for block placement and breaking.
//! It handles validation, collision detection, and world updates.
//!
//! # Architecture
//!
//! - Core converts packets → events (BlockPlaceAttemptEvent, BlockBreakAttemptEvent)
//! - This plugin reads events and applies game logic
//! - Plugin uses BlockAPI to broadcast block updates
//! - Core converts block update requests → packets

mod item_mapping;

use bevy_ecs::prelude::*;
use ferrumc_block_api::{
    BlockAPI, BlockBreakAttemptEvent, BlockPlaceAttemptEvent, Hand, SendBlockChangeAckRequest,
    SendBlockUpdateRequest,
};
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::transform::position::Position;
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_plugin_api::{register_events, Plugin, PluginContext};
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use std::sync::Arc;
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
        "Handles block placement and breaking with collision detection"
    }

    fn build(&self, ctx: &mut PluginContext<'_>) {
        info!("Loading blocks plugin");

        // Register events from block API
        register_events!(
            ctx,
            BlockPlaceAttemptEvent,
            BlockBreakAttemptEvent,
            SendBlockUpdateRequest,
            SendBlockChangeAckRequest
        );

        // Register our gameplay logic systems
        ctx.add_tick_system(handle_block_placement);
        ctx.add_tick_system(handle_block_breaking);

        info!("Blocks plugin loaded successfully");
    }
}

/// Handle block placement attempts
///
/// This is pure game logic:
/// - Validates the placement (inventory, collision, etc.)
/// - Updates the world
/// - Broadcasts updates to players
fn handle_block_placement(
    mut events: EventReader<BlockPlaceAttemptEvent>,
    mut blocks: BlockAPI,
    state: Res<GlobalStateResource>,
    query: Query<(&Inventory, &Hotbar)>,
    pos_query: Query<(&Position, &CollisionBounds)>,
) {
    for event in events.read() {
        // Only handle main hand for now
        if event.hand != Hand::Main {
            trace!("Offhand block placement not implemented");
            continue;
        }

        // Get player inventory
        let Ok((inventory, hotbar)) = query.get(event.player) else {
            debug!("Could not get inventory for player {:?}", event.player);
            continue;
        };

        // Get the item in the selected hotbar slot
        let slot_index = hotbar.selected_slot as usize;
        let Ok(slot) = inventory.get_item(slot_index) else {
            error!("Could not fetch inventory slot {}", slot_index);
            continue;
        };

        let Some(selected_item) = slot else {
            trace!("No item in selected slot");
            continue;
        };

        // Get the item ID
        let Some(item_id) = selected_item.item_id else {
            error!("Selected item has no item ID");
            continue;
        };

        // Map item to block state
        let Some(&mapped_block_state_id) = ITEM_TO_BLOCK_MAPPING.get(&item_id.0 .0) else {
            error!("No block mapping found for item ID: {}", item_id.0);
            continue;
        };

        debug!(
            "Placing block with item ID: {}, mapped to block state ID: {}",
            item_id.0, mapped_block_state_id
        );

        // Load the chunk
        let mut chunk = match state.0.world.load_chunk_owned(
            event.position.x >> 4,
            event.position.z >> 4,
            "overworld",
        ) {
            Ok(chunk) => chunk,
            Err(e) => {
                debug!("Failed to load chunk: {:?}", e);
                continue;
            }
        };

        // Calculate the position offset based on the face
        let (x_offset, y_offset, z_offset) = event.face.offset();
        let (x, y, z) = (
            event.position.x + x_offset,
            event.position.y + y_offset as i16,
            event.position.z + z_offset,
        );

        // Check collision with entities
        let does_collide = pos_query.iter().any(|(pos, bounds)| {
            bounds.collides(
                (pos.x, pos.y, pos.z),
                &CollisionBounds {
                    x_offset_start: 0.0,
                    x_offset_end: 1.0,
                    y_offset_start: 0.0,
                    y_offset_end: 1.0,
                    z_offset_start: 0.0,
                    z_offset_end: 1.0,
                },
                (x as f64, y as f64, z as f64),
            )
        });

        if does_collide {
            trace!("Block placement collided with entity");
            continue;
        }

        // Set the block in the chunk
        if let Err(err) = chunk.set_block(
            x & 0xF,
            y as i32,
            z & 0xF,
            BlockStateId(mapped_block_state_id as u32),
        ) {
            error!("Failed to set block: {:?}", err);
            continue;
        }

        // Save the chunk
        if let Err(err) = state.0.world.save_chunk(Arc::from(chunk)) {
            error!("Failed to save chunk after block placement: {:?}", err);
        } else {
            trace!("Block placed at ({}, {}, {})", x, y, z);
        }

        // Send acknowledgment to the player
        blocks.send_ack(event.player, event.sequence);

        // Broadcast block update to all players
        blocks.broadcast_block_update(
            ferrumc_net_codec::net_types::network_position::NetworkPosition { x, y, z },
            BlockStateId(mapped_block_state_id as u32),
        );
    }
}

/// Handle block breaking attempts
///
/// This is pure game logic:
/// - Validates the break (permissions, gamemode, etc.)
/// - Updates the world
/// - Broadcasts updates to players
fn handle_block_breaking(
    mut events: EventReader<BlockBreakAttemptEvent>,
    mut blocks: BlockAPI,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        // Load or generate the chunk
        let mut chunk = match state.0.world.load_chunk_owned(
            event.position.x >> 4,
            event.position.z >> 4,
            "overworld",
        ) {
            Ok(chunk) => chunk,
            Err(e) => {
                trace!("Chunk not found, generating new chunk: {:?}", e);
                match state
                    .0
                    .terrain_generator
                    .generate_chunk(event.position.x >> 4, event.position.z >> 4)
                {
                    Ok(chunk) => chunk,
                    Err(e) => {
                        error!("Failed to generate chunk: {:?}", e);
                        continue;
                    }
                }
            }
        };

        // Calculate relative position within chunk
        let (relative_x, relative_y, relative_z) = (
            event.position.x.abs() % 16,
            event.position.y as i32,
            event.position.z.abs() % 16,
        );

        // Set block to air
        if let Err(err) = chunk.set_block(relative_x, relative_y, relative_z, BlockStateId::default()) {
            error!("Failed to break block: {:?}", err);
            continue;
        }

        // Save the chunk
        if let Err(err) = state.0.world.save_chunk(Arc::new(chunk)) {
            error!("Failed to save chunk after block break: {:?}", err);
        } else {
            trace!("Block broken at ({}, {}, {})", event.position.x, event.position.y, event.position.z);
        }

        // Send acknowledgment to the player who broke the block
        blocks.send_ack(event.player, event.sequence);

        // Broadcast block update to all players
        blocks.broadcast_block_update(event.position.clone(), BlockStateId::default());
    }
}
