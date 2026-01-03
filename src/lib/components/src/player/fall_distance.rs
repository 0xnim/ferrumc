//! Fall distance tracking component for fall damage calculation.

use bevy_ecs::prelude::*;

/// Tracks fall distance for calculating fall damage.
///
/// The system tracks when a player leaves the ground and accumulates
/// the distance fallen. When the player lands, fall damage is calculated
/// based on the total distance (damage = distance - 3 blocks).
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct FallDistance {
    /// Whether the player was on the ground last tick
    pub was_on_ground: bool,
    /// The Y position when the player started falling
    pub fall_start_y: f64,
    /// Current accumulated fall distance in blocks
    pub distance: f32,
}

impl FallDistance {
    /// Create a new FallDistance tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start tracking a fall from the given Y position.
    pub fn start_fall(&mut self, y: f64) {
        self.fall_start_y = y;
        self.distance = 0.0;
    }

    /// Update the fall distance based on current Y position.
    /// Returns the accumulated distance.
    pub fn update(&mut self, current_y: f64) -> f32 {
        let fallen = (self.fall_start_y - current_y).max(0.0) as f32;
        self.distance = fallen;
        fallen
    }

    /// Reset the fall tracker (called when landing).
    pub fn reset(&mut self) {
        self.distance = 0.0;
        self.fall_start_y = 0.0;
    }

    /// Calculate the damage from the current fall distance.
    /// Minecraft formula: damage = (distance - 3) half-hearts, minimum 0.
    pub fn calculate_damage(&self) -> f32 {
        (self.distance - 3.0).max(0.0)
    }

    /// Check if this fall would cause damage (> 3 blocks).
    pub fn would_cause_damage(&self) -> bool {
        self.distance > 3.0
    }
}
