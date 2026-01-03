//! # FerrumC Mod API
//!
//! This crate defines the trait-based API for creating mods that extend FerrumC.
//!
//! ## Architecture
//!
//! The API is structured similar to Vintage Story's VSApi:
//! - [`ModSystem`] - Base trait for all mods with lifecycle methods
//! - [`CoreApi`] - Common API for registering content (blocks, entities, items)
//! - [`ServerApi`] - Server-specific API (world access, events, tick systems)
//!
//! ## Example
//!
//! ```ignore
//! use ferrumc_api::prelude::*;
//!
//! pub struct MyMod;
//!
//! impl ModSystem for MyMod {
//!     fn mod_id(&self) -> &'static str { "my_mod" }
//!
//!     fn start(&self, api: &mut dyn CoreApi) {
//!         // Register behaviors, commands, etc.
//!     }
//!
//!     fn start_server_side(&self, api: &mut dyn ServerApi) {
//!         // Register server-specific systems
//!     }
//! }
//! ```

pub mod behavior;
pub mod components;
pub mod context;
pub mod event;
pub mod provider;
pub mod world;

use std::sync::Arc;

use world::WorldAccess;

use behavior::{BlockBehavior, CollectibleBehavior, EntityBehavior};
use ferrumc_commands::BuiltCommand;
use provider::ComponentProvider;

/// Re-exports for convenient imports
pub mod prelude {
    pub use crate::behavior::{
        BlockBehavior, BlockContext, CollectibleBehavior, EntityBehavior, Handling,
    };
    pub use crate::components::DamageImmune;
    pub use crate::context::*;
    pub use crate::event::{Event, EventHandler, EventPriority};
    pub use crate::provider::{ComponentProvider, EntitySetupContext, PlayerSetupContext};
    pub use crate::world::{Dimension, WorldAccess};
    pub use crate::{CoreApi, ModSystem, ResourceBuilder, ScheduleBuilder, ServerApi};

    // Command API re-exports
    pub use ferrumc_commands::arg::builders::{
        BoolArg, FloatArg, GameModeArg, GreedyStringArg, IntArg, LongArg, StringArg, WordArg,
    };
    pub use ferrumc_commands::{BuiltCommand, CommandBuilder, CommandContext, CommandResult};

    // World types re-exports
    pub use ferrumc_world::block_state_id::BlockStateId;
    pub use ferrumc_world::pos::{BlockPos, ChunkPos};
}

/// Core trait for all mods.
///
/// Implement this trait to create a mod that extends FerrumC.
/// The lifecycle methods are called in order during server startup:
///
/// 1. `start()` - Register behaviors, commands, content types
/// 2. `start_server_side()` - Register server systems, event listeners
/// 3. `assets_loaded()` - Called after all mods have registered content
/// 4. `assets_finalize()` - Final chance to modify before server accepts connections
pub trait ModSystem: Send + Sync + 'static {
    /// Unique identifier for this mod (e.g., "ferrumc:survival")
    fn mod_id(&self) -> &'static str;

    /// Mod version string
    fn version(&self) -> &'static str {
        "1.0.0"
    }

    /// List of mod IDs this mod depends on
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    /// Called first during startup. Use this to register:
    /// - Block behaviors
    /// - Entity behaviors
    /// - Item behaviors
    /// - Commands
    fn start(&self, _api: &mut dyn CoreApi) {}

    /// Called after world is loaded. Use this to register:
    /// - Tick systems
    /// - Event listeners
    /// - Component providers
    fn start_server_side(&self, _api: &mut dyn ServerApi) {}

    /// Called after all mods have registered their content.
    /// Use this to resolve cross-mod dependencies.
    fn assets_loaded(&self, _api: &mut dyn ServerApi) {}

    /// Final initialization before server accepts connections.
    /// Last chance to modify registered content.
    fn assets_finalize(&self, _api: &mut dyn ServerApi) {}

    /// Called when the server is shutting down.
    ///
    /// TODO: Add shutdown context parameter when shutdown sequence is implemented.
    fn dispose(&self) {}
}

/// Common API available to all mods.
///
/// Use this to register content types like behaviors and commands.
pub trait CoreApi: Send + Sync {
    /// Register a block behavior for a specific block ID.
    ///
    /// Multiple behaviors can be registered for the same block.
    /// Use "*" as block_id to apply to all blocks.
    fn register_block_behavior(&mut self, block_id: &str, behavior: Arc<dyn BlockBehavior>);

    /// Register an entity behavior for a specific entity type.
    ///
    /// Multiple behaviors can be registered for the same entity type.
    /// Use "player" for player entities.
    fn register_entity_behavior(&mut self, entity_type: &str, behavior: Arc<dyn EntityBehavior>);

    /// Register an item behavior for a specific item ID.
    fn register_item_behavior(&mut self, item_id: &str, behavior: Arc<dyn CollectibleBehavior>);

    /// Register a command via the builder API.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ferrumc_commands::{CommandBuilder, arg::builders::FloatArg};
    ///
    /// api.register_command(
    ///     CommandBuilder::new("teleport")
    ///         .description("Teleport to coordinates")
    ///         .permission("server.teleport")
    ///         .requires_player()
    ///         .arg(FloatArg::new("x"))
    ///         .arg(FloatArg::new("y"))
    ///         .arg(FloatArg::new("z"))
    ///         .handler(|ctx| {
    ///             // Handle command
    ///             Ok(())
    ///         })
    ///         .build()
    /// );
    /// ```
    fn register_command(&mut self, command: BuiltCommand);
}

/// Server-specific API extending CoreApi.
///
/// Provides access to world, events, and server systems.
pub trait ServerApi: CoreApi {
    /// Get access to the game world for block/chunk operations.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn start_server_side(&self, api: &mut dyn ServerApi) {
    ///     let world = api.world();
    ///
    ///     // Read a block
    ///     let block = world.get_block(BlockPos::of(0, 64, 0), Dimension::Overworld)?;
    ///
    ///     // Set a block
    ///     world.set_block(BlockPos::of(0, 65, 0), Dimension::Overworld, BlockStateId::AIR)?;
    /// }
    /// ```
    fn world(&self) -> &dyn WorldAccess;

    /// Register a function to be called every game tick.
    fn register_tick_system(&mut self, system: TickSystem);

    /// Register a component provider for player entities.
    ///
    /// The provider will be called when new players join to add
    /// mod-specific components to the player entity.
    fn register_player_component_provider(&mut self, provider: Arc<dyn ComponentProvider>);

    /// Register a component provider for non-player entities.
    fn register_entity_component_provider(
        &mut self,
        entity_type: &str,
        provider: Arc<dyn ComponentProvider>,
    );

    /// Register gameplay systems that run every game tick.
    ///
    /// Use this to add Bevy ECS systems with full parameter injection support.
    /// Systems are added to the main tick schedule.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn start_server_side(&self, api: &mut dyn ServerApi) {
    ///     api.register_gameplay_systems(Box::new(|schedule| {
    ///         schedule.add_systems(my_damage_handler);
    ///         schedule.add_systems(my_hunger_system);
    ///     }));
    /// }
    /// ```
    fn register_gameplay_systems(&mut self, builder: ScheduleBuilder);

    /// Register ECS resources needed by mod systems.
    ///
    /// Use this to insert resources (timers, config, state) into the
    /// ECS world that your systems depend on.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn start_server_side(&self, api: &mut dyn ServerApi) {
    ///     api.register_resources(Box::new(|world| {
    ///         world.insert_resource(MyTimer::default());
    ///     }));
    /// }
    /// ```
    fn register_resources(&mut self, builder: ResourceBuilder);
}

/// A system that runs every game tick.
pub type TickSystem = Box<dyn Fn(&mut bevy_ecs::world::World) + Send + Sync>;

/// A builder function that registers systems to a Bevy schedule.
///
/// This allows mods to register their Bevy ECS systems using the full
/// power of Bevy's system API (parameter injection, system sets, etc.).
///
/// # Example
///
/// ```ignore
/// fn start_server_side(&self, api: &mut dyn ServerApi) {
///     api.register_gameplay_systems(Box::new(|schedule| {
///         schedule.add_systems(my_system);
///         schedule.add_systems((system_a, system_b));
///     }));
/// }
/// ```
pub type ScheduleBuilder = Box<dyn FnOnce(&mut bevy_ecs::schedule::Schedule) + Send>;

/// A builder function that registers ECS resources to the world.
///
/// This allows mods to insert their resources (timers, config, etc.)
/// into the ECS world during initialization.
pub type ResourceBuilder = Box<dyn FnOnce(&mut bevy_ecs::world::World) + Send>;
