//! Plugin context and configuration

use bevy_ecs::prelude::*;
use ferrumc_state::GlobalState;
use std::collections::HashMap;
use std::time::Duration;

/// Context provided to plugins during initialization.
///
/// Plugins use this to register systems, commands, events, and access
/// server state and configuration.
pub struct PluginContext<'a> {
    /// ECS world for registering components, resources, and events
    pub world: &'a mut World,

    /// Global server state (world, terrain generator, etc.)
    pub state: GlobalState,

    /// Configuration specific to this plugin (from plugins.toml)
    pub config: PluginConfig,

    /// Internal: Schedule for tick systems
    tick_schedule: &'a mut Schedule,
}

impl<'a> PluginContext<'a> {
    /// Create a new plugin context.
    ///
    /// This is called internally by the plugin registry and should not
    /// be constructed manually.
    pub fn new(
        world: &'a mut World,
        state: GlobalState,
        config: PluginConfig,
        tick_schedule: &'a mut Schedule,
    ) -> Self {
        Self {
            world,
            state,
            config,
            tick_schedule,
        }
    }

    /// Register a system to run every tick.
    ///
    /// Tick systems run at the configured TPS (ticks per second).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// ctx.add_tick_system(my_system);
    /// ```
    pub fn add_tick_system<Marker>(&mut self, system: impl IntoSystem<(), (), Marker> + 'static) {
        self.tick_schedule.add_systems(system);
    }

    /// Register a system with a custom schedule that runs periodically.
    ///
    /// # Arguments
    ///
    /// * `name` - Unique name for this schedule
    /// * `period` - How often the system should run
    /// * `system` - The system to run
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::time::Duration;
    ///
    /// ctx.add_timed_system(
    ///     "autosave",
    ///     Duration::from_secs(300),
    ///     autosave_system
    /// );
    /// ```
    pub fn add_timed_system<Marker>(
        &mut self,
        name: impl Into<String>,
        _period: Duration,
        _system: impl IntoSystem<(), (), Marker> + 'static,
    ) {
        // TODO: This needs proper implementation
        // The scheduler's TimedSchedule::new takes a closure that builds the schedule
        // We can't easily pass the system through this API
        // For now, log a warning
        tracing::warn!(
            "add_timed_system('{}') called but not yet fully implemented",
            name.into()
        );

        // Proper implementation will require refactoring the scheduler
        // to allow adding systems after TimedSchedule creation
    }

    /// Register an event type.
    ///
    /// Events allow systems to communicate asynchronously. After registering
    /// an event, systems can use `EventWriter<T>` to send events and
    /// `EventReader<T>` to receive them.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// #[derive(Event)]
    /// struct MyEvent {
    ///     data: String,
    /// }
    ///
    /// ctx.register_event::<MyEvent>();
    /// ```
    ///
    /// # Note
    ///
    /// Currently this initializes the event storage. The event will be properly
    /// registered when Bevy App integration is added.
    pub fn register_event<T: Event>(&mut self) {
        // Initialize event resources manually since we don't have App
        self.world.init_resource::<Events<T>>();
    }

    /// Register a component type.
    ///
    /// This is optional - components are automatically registered when first used.
    /// However, explicit registration can be useful for documentation and
    /// ensuring the component is available.
    pub fn register_component<T: Component>(&mut self) {
        self.world.register_component::<T>();
    }

    /// Insert a global resource.
    ///
    /// Resources are global singletons that can be accessed by systems
    /// using `Res<T>` and `ResMut<T>`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// #[derive(Resource, Default)]
    /// struct MyResource {
    ///     counter: u32,
    /// }
    ///
    /// ctx.insert_resource(MyResource::default());
    /// ```
    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.world.insert_resource(resource);
    }

    /// Register a command.
    ///
    /// Commands are automatically registered when using the `#[command]` macro,
    /// but this method allows manual registration if needed.
    ///
    /// Note: This is a placeholder and will be fully implemented when command
    /// system integration is complete.
    pub fn register_command_fn(&mut self, _name: &str) {
        // This will be implemented when we integrate with the command system
        tracing::warn!("Command registration not yet fully implemented");
    }

    /// Get a configuration value for this plugin.
    ///
    /// Configuration is loaded from `plugins.toml` under the section
    /// matching the plugin's name.
    ///
    /// # Example
    ///
    /// If `plugins.toml` contains:
    /// ```toml
    /// [my_plugin]
    /// interval_seconds = 300
    /// enabled = true
    /// ```
    ///
    /// Then you can read it with:
    /// ```rust,no_run
    /// let interval: u64 = ctx.get_config("interval_seconds").unwrap_or(60);
    /// let enabled: bool = ctx.get_config("enabled").unwrap_or(true);
    /// ```
    pub fn get_config<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.config.get(key)
    }

    /// Check if a configuration key exists.
    pub fn has_config(&self, key: &str) -> bool {
        self.config.has(key)
    }

    /// Get the raw configuration table for this plugin.
    pub fn get_config_table(&self) -> &HashMap<String, toml::Value> {
        &self.config.values
    }
}

/// Plugin-specific configuration loaded from plugins.toml
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub(crate) values: HashMap<String, toml::Value>,
}

impl PluginConfig {
    /// Create a new empty plugin config
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Create from a TOML table
    pub fn from_table(table: toml::Table) -> Self {
        Self {
            values: table.into_iter().collect(),
        }
    }

    /// Get a typed configuration value
    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.values.get(key).and_then(|v| v.clone().try_into().ok())
    }

    /// Check if a key exists
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    /// Get a value as a string
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key)
    }

    /// Get a value as an integer
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)
    }

    /// Get a value as a boolean
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)
    }

    /// Get a value as a float
    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.get(key)
    }
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self::new()
    }
}
