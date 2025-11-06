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

pub mod build_context;
pub mod capabilities;
pub mod context;
pub mod entity;
pub mod events;
pub mod macros;
pub mod prelude;
pub mod queries;
pub mod world;

#[cfg(test)]
mod tests;

pub use build_context::{EventRegistry, PluginBuildContext, SystemRegistry};
pub use capabilities::{PluginCapabilities, PluginCapabilitiesBuilder, ResourceCapability};
pub use context::{PluginConfig, PluginContext};
pub use queries::{EntityQueries, InventoryQueries, InventoryQueriesMut, WorldQueries};

// IMPORTANT: EntityExt and WorldExt are NOT exported for plugins!
// They provide direct network/database access which violates separation of concerns.
// Use EntityAPI and domain-specific APIs instead.
// 
// If you're in core-systems and need these, import them explicitly:
// use ferrumc_plugin_api::entity::EntityExt;
// use ferrumc_plugin_api::world::WorldExt;

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

    /// Execution priority for system registration order.
    ///
    /// Higher priority plugins register systems first, meaning their systems
    /// will run earlier in the tick when reading the same events.
    ///
    /// Use this to control the order of event processing when multiple plugins
    /// read/modify the same events.
    ///
    /// **Priority Guidelines:**
    /// - Default: 0 (final processing, handlers)
    /// - Base Systems: 50+ (calculate base values, emit events)
    /// - Modifiers: 30-49 (modify events emitted by base systems)
    /// - Anti-cheat/Validation: 100+ (run before everything else)
    /// - Logging/Monitoring: -100 (run after everything else)
    ///
    /// Example:
    /// ```rust,no_run
    /// fn priority(&self) -> i32 {
    ///     50  // Base damage calculation runs early
    /// }
    /// ```
    fn priority(&self) -> i32 {
        0  // Default priority
    }
    
    /// Declare required capabilities.
    ///
    /// Plugins must declare what they need access to. The type system
    /// will enforce these boundaries at compile time.
    ///
    /// **Default:** For backward compatibility, returns all capabilities.
    /// This will be removed in a future version.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// fn capabilities(&self) -> PluginCapabilities {
    ///     PluginCapabilities::builder()
    ///         .with_animation_api()
    ///         .with_entity_queries()
    ///         .build()
    /// }
    /// ```
    #[allow(deprecated)]
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::all()  // Default: all for backward compat
    }

    /// Build the plugin - called during server initialization.
    ///
    /// This is where you register:
    /// - ECS systems (tick systems, timed systems)
    /// - Event types
    /// - Components
    /// - Resources
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// fn build(&self, mut ctx: PluginBuildContext) {
    ///     // Register events
    ///     ctx.events()
    ///         .register::<MyEvent>();
    ///     
    ///     // Register systems
    ///     ctx.systems()
    ///         .add_tick(my_tick_system)
    ///         .add_timed("periodic", Duration::from_secs(5), my_periodic_system);
    ///     
    ///     // Insert a resource
    ///     ctx.events().insert_resource(MyResource::default());
    /// }
    /// ```
    fn build(&self, ctx: PluginBuildContext<'_>);
}

/// Helper trait for plugin implementations that need default construction.
pub trait PluginDefault: Plugin + Default {}

impl<T: Plugin + Default> PluginDefault for T {}
