//! Plugin capability system
//!
//! Capabilities define what a plugin is allowed to access at compile time.
//! Plugins must declare their required capabilities upfront.

use std::any::TypeId;

/// Capabilities that a plugin can request
///
/// Plugins declare capabilities in their `capabilities()` method.
/// The type system enforces these boundaries.
///
/// # Example
///
/// ```rust
/// fn capabilities(&self) -> PluginCapabilities {
///     PluginCapabilities::builder()
///         .with_animation_api()
///         .with_entity_queries()
///         .build()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PluginCapabilities {
    /// Access to animation API
    pub animation_api: bool,
    
    /// Access to block API
    pub block_api: bool,
    
    /// Access to chat API
    pub chat_api: bool,
    
    /// Access to inventory API
    pub inventory_api: bool,
    
    /// Access to join/leave API
    pub join_leave_api: bool,
    
    /// Access to entity API (future)
    pub entity_api: bool,
    
    /// Access to movement API
    pub movement_api: bool,
    
    /// Ability to query entities and their safe components
    pub entity_queries: bool,
    
    /// Ability to query world data (read-only)
    pub world_queries: bool,
    
    /// Resources the plugin can access
    pub resources: Vec<ResourceCapability>,
}

impl Default for PluginCapabilities {
    fn default() -> Self {
        Self::none()
    }
}

impl PluginCapabilities {
    /// No capabilities (most restrictive)
    pub fn none() -> Self {
        Self {
            animation_api: false,
            block_api: false,
            chat_api: false,
            inventory_api: false,
            join_leave_api: false,
            entity_api: false,
            movement_api: false,
            entity_queries: false,
            world_queries: false,
            resources: Vec::new(),
        }
    }
    
    /// All capabilities (for backward compatibility during migration)
    ///
    /// **DEPRECATED:** Will be removed after migration is complete.
    /// Plugins should declare specific capabilities instead.
    #[deprecated(note = "Declare specific capabilities instead of using all()")]
    pub fn all() -> Self {
        Self {
            animation_api: true,
            block_api: true,
            chat_api: true,
            inventory_api: true,
            join_leave_api: true,
            entity_api: true,
            movement_api: true,
            entity_queries: true,
            world_queries: true,
            resources: Vec::new(),
        }
    }
    
    /// Create a capabilities builder
    pub fn builder() -> PluginCapabilitiesBuilder {
        PluginCapabilitiesBuilder::new()
    }
}

/// Builder for plugin capabilities
///
/// Provides a fluent API for declaring capabilities.
///
/// # Example
///
/// ```rust
/// let caps = PluginCapabilities::builder()
///     .with_animation_api()
///     .with_entity_queries()
///     .with_resource::<MyResource>()
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct PluginCapabilitiesBuilder {
    caps: PluginCapabilities,
}

impl PluginCapabilitiesBuilder {
    /// Create a new capabilities builder
    pub fn new() -> Self {
        Self {
            caps: PluginCapabilities::none(),
        }
    }
    
    /// Request access to animation API
    pub fn with_animation_api(mut self) -> Self {
        self.caps.animation_api = true;
        self
    }
    
    /// Request access to block API
    pub fn with_block_api(mut self) -> Self {
        self.caps.block_api = true;
        self
    }
    
    /// Request access to chat API
    pub fn with_chat_api(mut self) -> Self {
        self.caps.chat_api = true;
        self
    }
    
    /// Request access to inventory API
    pub fn with_inventory_api(mut self) -> Self {
        self.caps.inventory_api = true;
        self
    }
    
    /// Request access to join/leave API
    pub fn with_join_leave_api(mut self) -> Self {
        self.caps.join_leave_api = true;
        self
    }
    
    /// Request access to entity API (future)
    pub fn with_entity_api(mut self) -> Self {
        self.caps.entity_api = true;
        self
    }
    
    /// Request access to movement API
    pub fn with_movement_api(mut self) -> Self {
        self.caps.movement_api = true;
        self
    }
    
    /// Request ability to query entities
    pub fn with_entity_queries(mut self) -> Self {
        self.caps.entity_queries = true;
        self
    }
    
    /// Request ability to query world data (read-only)
    pub fn with_world_queries(mut self) -> Self {
        self.caps.world_queries = true;
        self
    }
    
    /// Request access to a specific resource
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.with_resource::<MyConfig>()
    /// ```
    pub fn with_resource<R: bevy_ecs::prelude::Resource>(mut self) -> Self {
        self.caps.resources.push(ResourceCapability::of::<R>());
        self
    }
    
    /// Request read-only access to a specific resource
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.with_resource_readonly::<ServerConfig>()
    /// ```
    pub fn with_resource_readonly<R: bevy_ecs::prelude::Resource>(mut self) -> Self {
        self.caps.resources.push(ResourceCapability::read_only::<R>());
        self
    }
    
    /// Build the capabilities
    pub fn build(self) -> PluginCapabilities {
        self.caps
    }
}

impl Default for PluginCapabilitiesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Capability to access a specific resource
#[derive(Debug, Clone)]
pub struct ResourceCapability {
    /// Type ID of the resource
    pub type_id: TypeId,
    
    /// Type name for debugging
    pub type_name: &'static str,
    
    /// Whether access is read-only
    pub read_only: bool,
}

impl ResourceCapability {
    /// Create a resource capability with read-write access
    pub fn of<R: bevy_ecs::prelude::Resource>() -> Self {
        Self {
            type_id: TypeId::of::<R>(),
            type_name: std::any::type_name::<R>(),
            read_only: false,
        }
    }
    
    /// Create a resource capability with read-only access
    pub fn read_only<R: bevy_ecs::prelude::Resource>() -> Self {
        Self {
            type_id: TypeId::of::<R>(),
            type_name: std::any::type_name::<R>(),
            read_only: true,
        }
    }
}
