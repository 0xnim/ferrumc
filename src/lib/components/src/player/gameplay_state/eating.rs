//! Eating state component.
//!
//! Tracks when a player is in the process of eating food.

use bevy_ecs::prelude::Component;
use ferrumc_inventories::item::ItemID;

/// Component added to players while they are eating.
///
/// This component is added when a player starts using a food item
/// and removed when eating completes or is cancelled.
#[derive(Component, Debug, Clone)]
pub struct EatingState {
    /// The item being eaten
    pub item_id: ItemID,
    /// Ticks elapsed since eating started
    pub ticks_elapsed: u32,
    /// Total ticks required to finish eating (32 ticks = 1.6s for most food)
    pub consume_duration_ticks: u32,
    /// Food value to restore (hunger points, 0-20)
    pub nutrition: u8,
    /// Saturation value to restore
    pub saturation: f32,
    /// The hand being used (0 = main, 1 = off)
    pub hand: u8,
}

impl EatingState {
    /// Create a new eating state.
    ///
    /// # Arguments
    /// * `item_id` - The food item being eaten
    /// * `consume_seconds` - How long it takes to eat (usually 1.6 seconds)
    /// * `nutrition` - Food points to restore
    /// * `saturation` - Saturation to restore
    /// * `hand` - Which hand is being used
    pub fn new(
        item_id: ItemID,
        consume_seconds: f32,
        nutrition: u8,
        saturation: f32,
        hand: u8,
    ) -> Self {
        // Convert seconds to ticks (20 TPS)
        let consume_duration_ticks = (consume_seconds * 20.0) as u32;

        Self {
            item_id,
            ticks_elapsed: 0,
            consume_duration_ticks,
            nutrition,
            saturation,
            hand,
        }
    }

    /// Advance the eating timer by one tick.
    ///
    /// Returns `true` if eating is complete.
    pub fn tick(&mut self) -> bool {
        self.ticks_elapsed += 1;
        self.ticks_elapsed >= self.consume_duration_ticks
    }

    /// Get progress as a fraction (0.0 to 1.0).
    pub fn progress(&self) -> f32 {
        if self.consume_duration_ticks == 0 {
            1.0
        } else {
            self.ticks_elapsed as f32 / self.consume_duration_ticks as f32
        }
    }

    /// Check if eating is finished.
    pub fn is_complete(&self) -> bool {
        self.ticks_elapsed >= self.consume_duration_ticks
    }
}
