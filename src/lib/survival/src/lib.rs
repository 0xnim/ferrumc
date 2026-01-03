//! # FerrumC Survival Game Mode
//!
//! This mod implements vanilla Minecraft survival mechanics including:
//! - Health and damage
//! - Hunger and saturation
//! - Experience and leveling
//! - Fall damage
//! - Food consumption
//!
//! ## Usage
//!
//! Add `ferrumc-survival` as a dependency to automatically register
//! the survival mod. The mod will be loaded and initialized during
//! server startup.
//!
//! ```toml
//! [dependencies]
//! ferrumc-survival = { path = "../survival" }
//! ```

pub mod systems;

use std::sync::Arc;

use bevy_ecs::prelude::*;
use ferrumc_api::prelude::*;
use ferrumc_api::{CoreApi, ModSystem, ServerApi};
use ferrumc_api_server::register_mod;
use tracing::info;

/// The survival game mode mod.
pub struct SurvivalMod;

impl ModSystem for SurvivalMod {
    fn mod_id(&self) -> &'static str {
        "ferrumc:survival"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn start(&self, api: &mut dyn CoreApi) {
        info!("Survival mod starting...");

        // Register a test command to verify the builder API works
        api.register_command(
            CommandBuilder::new("survivaltest")
                .description("Test command from survival mod")
                .handler(|ctx| {
                    ctx.sender.send_message(
                        ferrumc_text::TextComponentBuilder::new("Hello from survival mod!")
                            .color(ferrumc_text::NamedColor::Green)
                            .build(),
                        false,
                    );
                    Ok(())
                })
                .build(),
        );

        // TODO: Register behaviors
        // api.register_entity_behavior("player", Arc::new(HungerBehavior));
        // api.register_entity_behavior("player", Arc::new(HealthBehavior));
        // api.register_entity_behavior("player", Arc::new(FallDamageBehavior));
    }

    fn start_server_side(&self, api: &mut dyn ServerApi) {
        info!("Survival mod registering server-side components...");

        // Register component provider to add survival components to players
        api.register_player_component_provider(Arc::new(SurvivalComponentProvider));
    }

    fn assets_loaded(&self, _api: &mut dyn ServerApi) {
        info!("Survival mod assets loaded");
    }

    fn assets_finalize(&self, _api: &mut dyn ServerApi) {
        info!("Survival mod initialized successfully");
    }
}

/// Component provider that adds survival components to player entities.
struct SurvivalComponentProvider;

impl ComponentProvider for SurvivalComponentProvider {
    fn provide_player_components(
        &self,
        _commands: &mut EntityCommands,
        _ctx: &PlayerSetupContext,
    ) {
        // TODO: When survival components are moved here, add them:
        // commands.insert((
        //     Health::new(20.0),
        //     Hunger::new(20, 5.0),
        //     Experience::default(),
        //     ActiveEffects::default(),
        // ));
        //
        // For now, components are still in the core PlayerBundle,
        // so this is a no-op placeholder.
    }

    fn provider_id(&self) -> &'static str {
        "ferrumc:survival"
    }
}

/// Register the survival mod during static initialization.
#[ctor::ctor]
fn register_survival_mod() {
    register_mod(Arc::new(SurvivalMod));
}
