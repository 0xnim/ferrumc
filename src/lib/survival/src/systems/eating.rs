//! Eating system.
//!
//! Handles food consumption mechanics:
//! - Starting to eat when using a food item
//! - Ticking the eating progress
//! - Completing eating and restoring hunger

use bevy_ecs::prelude::*;
use ferrumc_components::player::gameplay_state::eating::EatingState;
use ferrumc_components::player::hunger::Hunger;
use ferrumc_data::generated::items::{ConsumableImpl, DataComponent, FoodImpl, Item};
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net::packets::outgoing::set_health::SetHealth;
use ferrumc_net::UseItemReceiver;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_state::GlobalStateResource;
use tracing::{debug, error, trace};

/// System that handles UseItem packets to start eating.
///
/// When a player right-clicks with food in hand, this starts the eating process.
pub fn handle_use_item(
    receiver: Res<UseItemReceiver>,
    state: Res<GlobalStateResource>,
    mut commands: Commands,
    query: Query<(&Hotbar, &Inventory, &Hunger, Option<&EatingState>)>,
) {
    for (packet, entity) in receiver.0.try_iter() {
        // Skip if player not connected
        if !state.0.players.is_connected(entity) {
            continue;
        }

        // Get player components
        let Ok((hotbar, inventory, hunger, eating_state)) = query.get(entity) else {
            continue;
        };

        // Skip if already eating
        if eating_state.is_some() {
            debug!("Player {} is already eating, ignoring UseItem", entity);
            continue;
        }

        // Get the item in the used hand
        let item_slot = if packet.hand.0 == 0 {
            // Main hand
            hotbar.get_selected_item(inventory).ok().flatten()
        } else {
            // Off hand - slot 45 in inventory
            inventory.get_item(45).ok().flatten()
        };

        let Some(slot) = item_slot else {
            trace!("Player {} used empty hand", entity);
            continue;
        };

        let Some(item_id) = slot.item_id else {
            trace!("Player {} used slot with no item ID", entity);
            continue;
        };

        // Look up the item to check if it's food
        let Some(item) = Item::from_id(item_id.0.0 as u16) else {
            debug!("Unknown item ID: {}", item_id.0.0);
            continue;
        };

        // Check for Food component
        let food_data = get_food_data(item);
        let consumable_data = get_consumable_data(item);

        let Some((nutrition, saturation, can_always_eat)) = food_data else {
            trace!("Item {} is not food", item.registry_key);
            continue;
        };

        // Check if player can eat (hunger < 20 or can_always_eat)
        if hunger.level >= 20 && !can_always_eat {
            debug!(
                "Player {} cannot eat {} (hunger full)",
                entity, item.registry_key
            );
            continue;
        }

        // Get consume duration (default 1.6 seconds)
        let consume_seconds = consumable_data.unwrap_or(1.6);

        debug!(
            "Player {} started eating {} (nutrition={}, saturation={}, duration={:.1}s)",
            entity, item.registry_key, nutrition, saturation, consume_seconds
        );

        // Add EatingState component
        commands.entity(entity).insert(EatingState::new(
            item_id,
            consume_seconds,
            nutrition,
            saturation,
            packet.hand.0 as u8,
        ));
    }
}

/// System that ticks eating progress and completes eating.
#[expect(clippy::type_complexity)]
pub fn tick_eating(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut EatingState,
        &mut Hunger,
        &Hotbar,
        &mut Inventory,
        Option<&StreamWriter>,
        Option<&ferrumc_components::health::Health>,
    )>,
) {
    for (entity, mut eating, mut hunger, hotbar, mut inventory, writer, health) in query.iter_mut()
    {
        // Tick the eating progress
        if eating.tick() {
            // Eating complete!
            debug!(
                "Player {} finished eating (nutrition={}, saturation={})",
                entity, eating.nutrition, eating.saturation
            );

            // Apply hunger restoration
            let old_level = hunger.level;
            hunger.level = hunger.level.saturating_add(eating.nutrition).min(20);
            hunger.saturation =
                (hunger.saturation + eating.saturation).min(f32::from(hunger.level));

            debug!(
                "Hunger restored: {} -> {} (saturation: {:.1})",
                old_level, hunger.level, hunger.saturation
            );

            // Consume the item
            let hand = eating.hand;
            let item_slot_index = if hand == 0 {
                hotbar.get_selected_inventory_index()
            } else {
                45 // Off hand
            };

            if let Ok(Some(slot)) = inventory.get_item(item_slot_index) {
                let new_count = slot.count.0 - 1;
                if new_count <= 0 {
                    // Remove the item entirely
                    let _ = inventory.remove_item_with_update(item_slot_index, entity);
                } else {
                    // Decrease count
                    let mut new_slot = slot.clone();
                    new_slot.count = VarInt(new_count);
                    let _ = inventory.set_item_with_update(item_slot_index, new_slot, entity);
                }
            }

            // Send SetHealth packet to update client
            if let Some(writer) = writer {
                let current_health = health.map(|h| h.current).unwrap_or(20.0);
                let packet = SetHealth {
                    health: current_health,
                    food: hunger.level.into(),
                    saturation: hunger.saturation,
                };
                if let Err(e) = writer.send_packet_ref(&packet) {
                    error!("Failed to send SetHealth packet: {:?}", e);
                }
            }

            // Remove EatingState component
            commands.entity(entity).remove::<EatingState>();
        }
    }
}

/// Get food data from an item (nutrition, saturation, can_always_eat).
fn get_food_data(item: &Item) -> Option<(u8, f32, bool)> {
    for (component, data) in item.components.iter() {
        if *component == DataComponent::Food {
            if let Some(food) = data.as_any().downcast_ref::<FoodImpl>() {
                return Some((food.nutrition, food.saturation, food.can_always_eat));
            }
        }
    }
    None
}

/// Get consumable data from an item (consume_seconds).
fn get_consumable_data(item: &Item) -> Option<f32> {
    for (component, data) in item.components.iter() {
        if *component == DataComponent::Consumable {
            if let Some(consumable) = data.as_any().downcast_ref::<ConsumableImpl>() {
                return Some(consumable.consume_seconds);
            }
        }
    }
    None
}
