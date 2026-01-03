//! Attack cooldown component.
//!
//! Tracks the attack cooldown state for combat calculations.

use bevy_ecs::prelude::Component;

/// Tracks attack cooldown for combat.
///
/// Minecraft's combat system uses attack cooldowns to calculate damage:
/// - Attacks shortly after the previous attack deal reduced damage
/// - Full damage is dealt when the cooldown is complete
/// - The cooldown rate depends on the weapon's attack speed attribute
#[derive(Component, Debug, Clone, Copy)]
pub struct AttackCooldown {
    /// Number of ticks since the last attack
    pub ticks_since_attack: u32,
    /// Attack speed modifier (default 4.0 for hand, varies by weapon)
    pub attack_speed: f32,
}

impl Default for AttackCooldown {
    fn default() -> Self {
        Self {
            ticks_since_attack: 100, // Start fully charged
            attack_speed: 4.0,       // Default hand attack speed
        }
    }
}

impl AttackCooldown {
    /// Get the cooldown progress (0.0 to 1.0).
    ///
    /// Returns 1.0 when fully charged, 0.0 immediately after attacking.
    pub fn progress(&self) -> f32 {
        // Full recovery time in ticks = 20 / attack_speed
        // (at 4.0 attack speed, it takes 5 ticks = 0.25s to fully recover)
        let recovery_ticks = 20.0 / self.attack_speed;
        (self.ticks_since_attack as f32 / recovery_ticks).min(1.0)
    }

    /// Calculate the damage multiplier based on cooldown progress.
    ///
    /// Returns a value between 0.2 and 1.0:
    /// - 0.2 at 0% progress (minimum damage)
    /// - 1.0 at 100% progress (full damage)
    pub fn damage_multiplier(&self) -> f32 {
        let progress = self.progress();
        // Minecraft formula: 0.2 + (progress * progress * 0.8)
        0.2 + (progress * progress * 0.8)
    }

    /// Check if the attack is fully charged.
    pub fn is_fully_charged(&self) -> bool {
        self.progress() >= 1.0
    }

    /// Reset the cooldown (call this after an attack).
    pub fn reset(&mut self) {
        self.ticks_since_attack = 0;
    }

    /// Tick the cooldown forward.
    pub fn tick(&mut self) {
        self.ticks_since_attack = self.ticks_since_attack.saturating_add(1);
    }

    /// Set attack speed (call this when weapon changes).
    ///
    /// Common weapon attack speeds:
    /// - Hand: 4.0
    /// - Sword: 1.6
    /// - Axe: 0.8-1.0
    /// - Pickaxe: 1.2
    /// - Hoe: varies
    pub fn set_attack_speed(&mut self, speed: f32) {
        self.attack_speed = speed.max(0.1); // Prevent division by zero
    }
}
