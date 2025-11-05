//! Plugin loading and initialization

use bevy_ecs::prelude::*;
use ferrumc_plugin_api::{Plugin, PluginConfig, PluginContext};
use ferrumc_state::GlobalState;
use std::collections::HashMap;
use tracing::{error, info, warn};

/// Registry of all compiled-in plugins
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
    config: HashMap<String, PluginConfig>,
}

impl PluginRegistry {
    /// Create a new empty plugin registry
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            config: HashMap::new(),
        }
    }

    /// Register a plugin
    pub fn register<P: Plugin + Default + 'static>(&mut self) {
        self.plugins.push(Box::new(P::default()));
    }

    /// Load configuration from plugins.toml
    pub fn load_config(&mut self, config_path: &str) -> Result<(), PluginError> {
        use std::fs;

        // Try to read plugins.toml
        let config_str = match fs::read_to_string(config_path) {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                warn!("plugins.toml not found, using default configuration");
                return Ok(());
            }
            Err(e) => {
                return Err(PluginError::ConfigLoadFailed(e.to_string()));
            }
        };

        // Parse TOML
        let config: toml::Table = toml::from_str(&config_str)
            .map_err(|e| PluginError::ConfigParseFailed(e.to_string()))?;

        // Extract plugin-specific configs
        for (key, value) in config {
            if let Some(table) = value.as_table() {
                self.config
                    .insert(key, PluginConfig::from_table(table.clone()));
            }
        }

        Ok(())
    }

    /// Sort plugins by dependencies, then by priority
    fn sort_by_dependencies(&mut self) -> Result<(), PluginError> {
        // Create dependency graph
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut in_degree: HashMap<&str, usize> = HashMap::new();

        for plugin in &self.plugins {
            let name = plugin.name();
            graph.insert(name, plugin.dependencies());
            in_degree.insert(name, 0);
        }

        // Calculate in-degrees
        for plugin in &self.plugins {
            for dep in plugin.dependencies() {
                *in_degree.get_mut(dep).unwrap_or(&mut 0) += 1;
            }
        }

        // Topological sort (Kahn's algorithm)
        let mut sorted = Vec::new();
        let mut queue: Vec<&str> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&name, _)| name)
            .collect();

        while let Some(name) = queue.pop() {
            sorted.push(name);

            if let Some(deps) = graph.get(name) {
                for &dep in deps {
                    if let Some(deg) = in_degree.get_mut(dep) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push(dep);
                        }
                    }
                }
            }
        }

        // Check for cycles
        if sorted.len() != self.plugins.len() {
            return Err(PluginError::CircularDependency);
        }

        // Reorder plugins based on sorted order
        let name_to_index: HashMap<&str, usize> = sorted
            .iter()
            .enumerate()
            .map(|(i, &name)| (name, i))
            .collect();

        self.plugins
            .sort_by_key(|p| name_to_index.get(p.name()).copied().unwrap_or(usize::MAX));

        // Within dependency groups, sort by priority (higher priority first)
        // This ensures systems from higher priority plugins register first
        self.plugins.sort_by(|a, b| {
            let dep_order_a = name_to_index.get(a.name()).copied().unwrap_or(usize::MAX);
            let dep_order_b = name_to_index.get(b.name()).copied().unwrap_or(usize::MAX);
            
            // First sort by dependency order
            match dep_order_a.cmp(&dep_order_b) {
                std::cmp::Ordering::Equal => {
                    // If same dependency level, sort by priority (higher first)
                    b.priority().cmp(&a.priority())
                }
                other => other,
            }
        });

        Ok(())
    }

    /// Build all plugins
    pub fn build_all(
        self,
        world: &mut World,
        state: GlobalState,
        tick_schedule: &mut Schedule,
    ) -> Result<(), PluginError> {
        info!("Initializing {} plugin(s)", self.plugins.len());

        // Check all dependencies first
        for plugin in &self.plugins {
            for dep in plugin.dependencies() {
                if !self.plugins.iter().any(|p| p.name() == dep) {
                    error!(
                        "Plugin {} requires dependency {} which is not loaded",
                        plugin.name(),
                        dep
                    );
                    return Err(PluginError::MissingDependency {
                        plugin: plugin.name().to_string(),
                        dependency: dep.to_string(),
                    });
                }
            }
        }

        // Build each plugin
        for plugin in self.plugins {
            info!(
                "Loading plugin: {} v{} by {}",
                plugin.name(),
                plugin.version(),
                plugin.author()
            );

            if !plugin.description().is_empty() {
                info!("  Description: {}", plugin.description());
            }

            // Get plugin config
            let config = self.config.get(plugin.name()).cloned().unwrap_or_default();

            // Create context
            let mut ctx = PluginContext::new(world, state.clone(), config, tick_schedule);

            // Build plugin
            plugin.build(&mut ctx);

            info!("  ✓ Plugin {} loaded successfully", plugin.name());
        }

        info!("All plugins loaded successfully");
        Ok(())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur during plugin loading
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Failed to load plugin configuration: {0}")]
    ConfigLoadFailed(String),

    #[error("Failed to parse plugin configuration: {0}")]
    ConfigParseFailed(String),

    #[error("Plugin {plugin} requires dependency {dependency} which is not loaded")]
    MissingDependency { plugin: String, dependency: String },

    #[error("Circular dependency detected in plugin dependencies")]
    CircularDependency,
}

/// Create and configure the plugin registry with all compiled-in plugins
pub fn create_plugin_registry() -> Result<PluginRegistry, PluginError> {
    let mut registry = PluginRegistry::new();

    // Load configuration
    registry.load_config("plugins.toml")?;

    // Register plugins
    // Core gameplay plugins
    registry.register::<ferrumc_plugin_animations::AnimationsPlugin>();
    registry.register::<ferrumc_plugin_blocks::BlocksPlugin>();
    registry.register::<ferrumc_plugin_chat::ChatPlugin>();
    registry.register::<ferrumc_plugin_default_commands::DefaultCommandsPlugin>();
    registry.register::<ferrumc_plugin_inventory::InventoryPlugin>();
    
    // Example plugins
    registry.register::<ferrumc_plugin_hello::HelloPlugin>();

    // TODO: Add more plugins here as they are created
    // registry.register::<ferrumc_plugin_entity_tracking::EntityTrackingPlugin>();
    // registry.register::<ferrumc_plugin_combat::CombatPlugin>();

    // Sort by dependencies
    registry.sort_by_dependencies()?;

    Ok(registry)
}
