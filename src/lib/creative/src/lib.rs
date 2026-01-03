//! # FerrumC Creative Game Mode
//!
//! This mod implements creative mode mechanics including:
//! - Instant block breaking
//! - Invulnerability (no damage)
//! - Flight capability
//! - Access to all items
//!
//! ## Usage
//!
//! Add `ferrumc-creative` as a dependency to automatically register
//! the creative mod. The mod will be loaded and initialized during
//! server startup.
//!
//! ```toml
//! [dependencies]
//! ferrumc-creative = { path = "../creative" }
//! ```

mod behaviors;
pub mod commands;
pub mod systems;

use std::sync::Arc;

use bevy_ecs::prelude::*;
use ferrumc_api::prelude::*;
use ferrumc_api::{CoreApi, ModSystem, ServerApi};
use ferrumc_api_server::register_mod;
use tracing::info;

pub use behaviors::{InstantBreakBehavior, InvulnerableBehavior};

/// Marker component indicating an entity is in creative mode.
/// This component is added/removed when the player's gamemode changes.
#[derive(Component, Default)]
pub struct CreativeMode;

/// The creative game mode mod.
pub struct CreativeMod;

impl ModSystem for CreativeMod {
    fn mod_id(&self) -> &'static str {
        "ferrumc:creative"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn start(&self, api: &mut dyn CoreApi) {
        info!("Creative mod starting...");

        // Register global block behavior for instant breaking in creative mode
        api.register_block_behavior("*", Arc::new(InstantBreakBehavior));

        // Register entity behavior for invulnerability
        api.register_entity_behavior("player", Arc::new(InvulnerableBehavior));
    }

    fn start_server_side(&self, api: &mut dyn ServerApi) {
        info!("Creative mod registering server-side components...");

        // Register component provider to add creative components to players
        api.register_player_component_provider(Arc::new(CreativeComponentProvider));

        // Register creative gameplay systems
        api.register_gameplay_systems(Box::new(|schedule| {
            // Instant block breaking for creative mode
            schedule.add_systems(systems::instant_break::handle_instant_break);
            // Creative inventory slot handling
            schedule.add_systems(systems::creative_slot::handle);
            // Creative pick item (spawn item when not in inventory)
            schedule.add_systems(systems::pick_item::handle_pick_item);
        }));
    }

    fn assets_loaded(&self, _api: &mut dyn ServerApi) {
        info!("Creative mod assets loaded");
    }

    fn assets_finalize(&self, _api: &mut dyn ServerApi) {
        info!("Creative mod initialized successfully");
    }
}

/// Component provider that adds creative mode components to player entities.
struct CreativeComponentProvider;

impl ComponentProvider for CreativeComponentProvider {
    fn provide_player_components(
        &self,
        commands: &mut EntityCommands,
        ctx: &PlayerSetupContext,
    ) {
        use ferrumc_api::components::DamageImmune;

        // Check if player should be in creative mode
        let is_creative = ctx
            .cached_data
            .as_ref()
            .and_then(|data| data.game_mode)
            .map(|mode| mode == 1) // 1 = Creative
            .unwrap_or(false); // Default to not creative if no cached data

        if is_creative {
            // Add both CreativeMode and DamageImmune
            commands.insert((CreativeMode, DamageImmune));
        }
    }

    fn provider_id(&self) -> &'static str {
        "ferrumc:creative"
    }
}

/// Register the creative mod during static initialization.
#[ctor::ctor]
fn register_creative_mod() {
    register_mod(Arc::new(CreativeMod));
}
