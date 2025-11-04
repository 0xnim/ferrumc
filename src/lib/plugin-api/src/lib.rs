//! FerrumC Plugin API
//!
//! This crate provides the foundation for building compiled-in plugins for FerrumC.
//! Plugins are Rust crates that implement the `Plugin` trait and are statically
//! linked into the server binary.
//!
//! # Example
//!
//! ```rust
//! use ferrumc_plugin_api::*;
//!
//! #[derive(Default)]
//! pub struct MyPlugin;
//!
//! impl Plugin for MyPlugin {
//!     fn name(&self) -> &'static str { "my_plugin" }
//!     fn version(&self) -> &'static str { "1.0.0" }
//!     
//!     fn build(&self, ctx: &mut PluginContext) {
//!         // Register systems, commands, events, etc.
//!         ctx.add_tick_system(my_system);
//!     }
//! }
//!
//! fn my_system() {
//!     // Your system logic
//! }
//! ```

pub mod context;
pub mod entity;
pub mod events;
pub mod world;

#[cfg(test)]
mod tests;

pub use context::{PluginConfig, PluginContext};
pub use entity::EntityExt;
pub use world::WorldExt;

/// Trait that all plugins must implement.
///
/// Plugins are registered at compile time and initialized during server startup.
/// The `build` method is called once during initialization, where the plugin
/// can register systems, commands, events, and resources.
pub trait Plugin: Send + Sync + 'static {
    /// Unique plugin identifier (used for dependencies and configuration).
    ///
    /// Must be unique across all plugins. Use lowercase with underscores.
    /// Example: "entity_tracking", "combat_system"
    fn name(&self) -> &'static str;

    /// Plugin version in semantic versioning format.
    ///
    /// Example: "1.0.0", "0.2.1-beta"
    fn version(&self) -> &'static str;

    /// Plugin author(s).
    ///
    /// Can be a single name or comma-separated list.
    fn author(&self) -> &'static str {
        "Unknown"
    }

    /// Brief description of what the plugin does.
    fn description(&self) -> &'static str {
        ""
    }

    /// Names of plugins this plugin depends on.
    ///
    /// Dependencies are loaded before this plugin. Circular dependencies
    /// are not allowed and will cause a panic at startup.
    ///
    /// Example:
    /// ```rust,no_run
    /// fn dependencies(&self) -> Vec<&'static str> {
    ///     vec!["entity_tracking", "inventory"]
    /// }
    /// ```
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }

    /// Build the plugin - called during server initialization.
    ///
    /// This is where you register:
    /// - ECS systems (tick systems, timed systems)
    /// - Commands
    /// - Event types
    /// - Components
    /// - Resources
    /// - Packet handlers
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// fn build(&self, ctx: &mut PluginContext) {
    ///     // Register a system that runs every tick
    ///     ctx.add_tick_system(my_tick_system);
    ///     
    ///     // Register a system that runs every 5 seconds
    ///     ctx.add_timed_system(
    ///         "my_periodic_system",
    ///         Duration::from_secs(5),
    ///         my_periodic_system
    ///     );
    ///     
    ///     // Register a command
    ///     ctx.register_command(my_command);
    ///     
    ///     // Register an event type
    ///     ctx.register_event::<MyEvent>();
    ///     
    ///     // Insert a resource
    ///     ctx.insert_resource(MyResource::default());
    /// }
    /// ```
    fn build(&self, ctx: &mut PluginContext<'_>);
}

/// Helper trait for plugin implementations that need default construction.
pub trait PluginDefault: Plugin + Default {}

impl<T: Plugin + Default> PluginDefault for T {}
