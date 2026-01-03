//! Synchronizes the CreativeMode and DamageImmune components with player gamemode changes.

use bevy_ecs::prelude::*;
use ferrumc_api::components::DamageImmune;
use ferrumc_components::player::gamemode::GameMode;
use ferrumc_messages::PlayerGameModeChanged;

use crate::CreativeMode;

/// System that syncs the CreativeMode and DamageImmune components when a player's gamemode changes.
///
/// - Adds CreativeMode and DamageImmune when switching TO creative
/// - Removes CreativeMode and DamageImmune when switching FROM creative
pub fn handle(mut commands: Commands, mut events: MessageReader<PlayerGameModeChanged>) {
    for event in events.read() {
        match event.new_mode {
            GameMode::Creative => {
                // Switching to creative - add creative mode and damage immunity
                commands
                    .entity(event.player)
                    .insert((CreativeMode, DamageImmune));
            }
            _ => {
                // Switching away from creative - remove both components
                commands
                    .entity(event.player)
                    .remove::<CreativeMode>()
                    .remove::<DamageImmune>();
            }
        }
    }
}
