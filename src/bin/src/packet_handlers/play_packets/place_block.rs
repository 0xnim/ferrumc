//! Block placement handler.
//!
//! This handler processes block placement requests.
//! Item consumption for survival mode is handled via events by ferrumc-survival.

use bevy_ecs::prelude::{Entity, MessageWriter, Query, Res};
use ferrumc_components::player::dimension::PlayerDimension;
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::transform::position::Position;
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_messages::BlockPlacedEvent;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::block_change_ack::BlockChangeAck;
use ferrumc_net::packets::outgoing::block_update::BlockUpdate;
use ferrumc_net::PlaceBlockReceiver;
use ferrumc_net_codec::net_types::network_position::NetworkPosition;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::pos::BlockPos;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::{debug, error, trace};

const ITEM_TO_BLOCK_MAPPING_FILE: &str =
    include_str!("../../../../../assets/data/item_to_block_mapping.json");
static ITEM_TO_BLOCK_MAPPING: Lazy<HashMap<i32, BlockStateId>> = Lazy::new(|| {
    let str_form: HashMap<String, String> = serde_json::from_str(ITEM_TO_BLOCK_MAPPING_FILE)
        .expect("Failed to parse item_to_block_mapping.json");
    str_form
        .into_iter()
        .map(|(k, v)| {
            (
                i32::from_str(&k).unwrap(),
                BlockStateId::new(u32::from_str(&v).unwrap()),
            )
        })
        .collect()
});

pub fn handle(
    receiver: Res<PlaceBlockReceiver>,
    state: Res<GlobalStateResource>,
    mut query: Query<(Entity, &StreamWriter, &Inventory, &Hotbar, &PlayerDimension)>,
    pos_q: Query<(&Position, &CollisionBounds)>,
    mut block_placed_events: MessageWriter<BlockPlacedEvent>,
) {
    'ev_loop: for (event, eid) in receiver.0.try_iter() {
        let Ok((entity, conn, inventory, hotbar, dimension)) = query.get_mut(eid) else {
            debug!("Could not get connection for entity {:?}", eid);
            continue;
        };
        if !state.0.players.is_connected(entity) {
            trace!("Entity {:?} is not connected", entity);
            continue;
        }
        match event.hand.0 {
            0 => {
                let Ok(slot) = hotbar.get_selected_item(inventory) else {
                    error!("Could not fetch {:?}", eid);
                    continue 'ev_loop;
                };
                if let Some(selected_item) = slot {
                    let Some(item_id) = selected_item.item_id else {
                        error!("Selected item has no item ID");
                        continue 'ev_loop;
                    };
                    let Some(mapped_block_state_id) = ITEM_TO_BLOCK_MAPPING.get(&item_id.0 .0)
                    else {
                        error!("No block mapping found for item ID: {}", item_id.0);
                        continue 'ev_loop;
                    };
                    debug!(
                        "Placing block with item ID: {}, mapped to block state ID: {}",
                        item_id.0, mapped_block_state_id
                    );
                    let pos: BlockPos = event.position.into();
                    let Ok(mut chunk) = ferrumc_utils::world::load_or_generate_mut(
                        &state.0,
                        pos.chunk(),
                        dimension.as_str(),
                    ) else {
                        error!("Failed to load or generate chunk");
                        continue 'ev_loop;
                    };
                    let Ok(block_clicked) = chunk.get_block(pos.chunk_block_pos()) else {
                        debug!("Failed to get block at position: {}", pos);
                        continue 'ev_loop;
                    };
                    trace!("Block clicked: {:?}", block_clicked);

                    // Use the face to determine the offset of the block to place
                    let offset_pos = pos
                        + match event.face.0 {
                            0 => (0, -1, 0),
                            1 => (0, 1, 0),
                            2 => (0, 0, -1),
                            3 => (0, 0, 1),
                            4 => (-1, 0, 0),
                            5 => (1, 0, 0),
                            _ => (0, 0, 0),
                        };

                    // Check if the block collides with any entities
                    let does_collide = {
                        pos_q.into_iter().any(|(pos, bounds)| {
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
                                (
                                    offset_pos.pos.x as f64,
                                    offset_pos.pos.y as f64,
                                    offset_pos.pos.z as f64,
                                ),
                            )
                        })
                    };
                    if does_collide {
                        trace!("Block placement collided with entity");
                        continue 'ev_loop;
                    }

                    // Send initial ACK
                    let packet = BlockChangeAck {
                        sequence: event.sequence,
                    };
                    if let Err(err) = conn.send_packet_ref(&packet) {
                        error!("Failed to send block change ack packet: {:?}", err);
                        continue 'ev_loop;
                    }

                    // Place the block in the world
                    if let Err(err) = chunk.set_block(pos.chunk_block_pos(), *mapped_block_state_id)
                    {
                        error!("Failed to set block: {:?}", err);
                        continue 'ev_loop;
                    }

                    // Fire event for survival mod to consume the item
                    let slot_index = hotbar.get_selected_inventory_index();
                    block_placed_events.write(BlockPlacedEvent {
                        player: entity,
                        position: offset_pos,
                        slot_index,
                    });

                    // Send block update to client
                    let chunk_packet = BlockUpdate {
                        location: NetworkPosition {
                            x: offset_pos.pos.x,
                            y: offset_pos.pos.y as i16,
                            z: offset_pos.pos.z,
                        },
                        block_state_id: VarInt::from(*mapped_block_state_id),
                    };
                    if let Err(err) = conn.send_packet_ref(&chunk_packet) {
                        error!("Failed to send block update packet: {:?}", err);
                        continue 'ev_loop;
                    }

                    // Send final ACK
                    let ack_packet = BlockChangeAck {
                        sequence: event.sequence,
                    };
                    if let Err(err) = conn.send_packet_ref(&ack_packet) {
                        error!("Failed to send block change ack packet: {:?}", err);
                        continue 'ev_loop;
                    }
                }
            }
            1 => {
                trace!("Offhand block placement not implemented");
            }
            _ => {
                debug!("Invalid hand");
            }
        }
    }
}
