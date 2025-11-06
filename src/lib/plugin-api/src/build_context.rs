//! Plugin build context - capability-restricted API
//!
//! This module provides the new PluginBuildContext which enforces
//! capability boundaries at compile time.

use bevy_ecs::prelude::*;
use bevy_ecs::system::IntoSystem;
use std::time::Duration;

use crate::capabilities::PluginCapabilities;
use crate::context::PluginConfig;

/// Context provided to plugins during build phase
///
/// Unlike the old PluginContext, this does NOT provide direct access
/// to World or State. Plugins must use domain APIs and query systems.
///
/// # Example
///
/// ```rust,no_run
/// fn build(&self, mut ctx: PluginBuildContext<'_>) {
///     ctx.events()
///         .register::<MyEvent>();
///     
///     ctx.systems()
///         .add_tick(my_system);
/// }
/// ```
pub struct PluginBuildContext<'a> {
    /// Declared capabilities for this plugin
    capabilities: PluginCapabilities,
    
    /// System registration
    systems: SystemRegistry<'a>,
    
    /// Event registration
    events: EventRegistry<'a>,
    
    /// Plugin configuration
    config: PluginConfig,
}

impl<'a> PluginBuildContext<'a> {
    /// Create a new plugin build context
    ///
    /// This is called internally by the plugin registry.
    pub fn new(
        capabilities: PluginCapabilities,
        world: &'a mut World,
        tick_schedule: &'a mut Schedule,
        config: PluginConfig,
    ) -> Self {
        Self {
            capabilities,
            systems: SystemRegistry::new(tick_schedule),
            events: EventRegistry::new(world),
            config,
        }
    }
    
    /// Access to system registration
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// ctx.systems()
    ///     .add_tick(my_system)
    ///     .add_tick(another_system);
    /// ```
    pub fn systems(&mut self) -> &mut SystemRegistry<'a> {
        &mut self.systems
    }
    
    /// Access to event registration
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// ctx.events()
    ///     .register::<MyEvent>()
    ///     .register::<AnotherEvent>();
    /// ```
    pub fn events(&mut self) -> &mut EventRegistry<'a> {
        &mut self.events
    }
    
    /// Access to plugin configuration
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let enabled: bool = ctx.config().get_bool("enabled").unwrap_or(true);
    /// ```
    pub fn config(&self) -> &PluginConfig {
        &self.config
    }
    
    /// Get the declared capabilities for this plugin
    pub fn capabilities(&self) -> &PluginCapabilities {
        &self.capabilities
    }
    
    /// Insert a resource into the world
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// ctx.insert_resource(MyResource::default());
    /// ```
    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.events.world.insert_resource(resource);
    }
}

/// System registration API
///
/// Controls what systems can be added. Future versions will validate
/// system parameters against declared capabilities.
pub struct SystemRegistry<'a> {
    tick_schedule: &'a mut Schedule,
}

impl<'a> SystemRegistry<'a> {
    /// Create a new system registry
    fn new(tick_schedule: &'a mut Schedule) -> Self {
        Self { tick_schedule }
    }
    
    /// Add a system that runs every tick
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// registry.add_tick(my_system);
    /// ```
    pub fn add_tick<Marker>(
        &mut self,
        system: impl IntoSystem<(), (), Marker> + 'static,
    ) -> &mut Self {
        self.tick_schedule.add_systems(system);
        self
    }
    

    
    /// Add a timed system that runs periodically
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// registry.add_timed(
    ///     "autosave",
    ///     Duration::from_secs(300),
    ///     autosave_system
    /// );
    /// ```
    pub fn add_timed<Marker>(
        &mut self,
        name: impl Into<String>,
        period: Duration,
        system: impl IntoSystem<(), (), Marker> + Send + 'static,
    ) -> &mut Self
    where
        Marker: Send + 'static,
    {
        let mut system_option = Some(system);
        ferrumc_scheduler::register_schedule(name, period, move |schedule| {
            if let Some(sys) = system_option.take() {
                schedule.add_systems(sys);
            }
        });
        self
    }
}

/// Event registration API
pub struct EventRegistry<'a> {
    world: &'a mut World,
}

impl<'a> EventRegistry<'a> {
    /// Create a new event registry
    fn new(world: &'a mut World) -> Self {
        Self { world }
    }
    
    /// Register an event type
    ///
    /// Events allow systems to communicate asynchronously.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// registry.register::<MyEvent>();
    /// ```
    pub fn register<E: Event>(&mut self) -> &mut Self {
        self.world.init_resource::<Events<E>>();
        self
    }
}
