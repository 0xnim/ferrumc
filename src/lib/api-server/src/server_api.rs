//! Server API implementation.
//!
//! Provides the concrete implementation of CoreApi and ServerApi
//! traits used during mod initialization.

use std::sync::Arc;

use parking_lot::{Mutex, RwLock};

use ferrumc_api::behavior::{BlockBehavior, CollectibleBehavior, EntityBehavior};
use ferrumc_api::provider::ComponentProvider;
use ferrumc_api::world::WorldAccess;
use ferrumc_api::{CoreApi, ResourceBuilder, ScheduleBuilder, ServerApi, TickSystem};
use ferrumc_commands::BuiltCommand;

use crate::registries::{
    BlockBehaviorRegistry, ComponentProviderRegistry, EntityBehaviorRegistry, ItemBehaviorRegistry,
};
use crate::world_accessor::WorldAccessor;

/// Server-side API implementation.
///
/// This is created during server startup and passed to mods
/// during their lifecycle methods.
pub struct ServerApiImpl {
    /// Block behavior registry
    pub block_behaviors: BlockBehaviorRegistry,
    /// Entity behavior registry
    pub entity_behaviors: EntityBehaviorRegistry,
    /// Item behavior registry
    pub item_behaviors: ItemBehaviorRegistry,
    /// Component provider registry
    pub component_providers: ComponentProviderRegistry,
    /// Tick systems to register (legacy closure-based)
    pub tick_systems: RwLock<Vec<TickSystem>>,
    /// Commands registered via the builder API
    pub commands: RwLock<Vec<BuiltCommand>>,
    /// Gameplay system builders (for Bevy ECS systems)
    /// Uses Mutex because FnOnce is not Sync
    pub gameplay_system_builders: Mutex<Vec<ScheduleBuilder>>,
    /// Resource builders (for ECS resources)
    /// Uses Mutex because FnOnce is not Sync
    pub resource_builders: Mutex<Vec<ResourceBuilder>>,
    /// World accessor for block/chunk operations
    world_accessor: Option<Arc<WorldAccessor>>,
}

impl ServerApiImpl {
    /// Create a new server API instance.
    pub fn new() -> Self {
        Self {
            block_behaviors: BlockBehaviorRegistry::new(),
            entity_behaviors: EntityBehaviorRegistry::new(),
            item_behaviors: ItemBehaviorRegistry::new(),
            component_providers: ComponentProviderRegistry::new(),
            tick_systems: RwLock::new(Vec::new()),
            commands: RwLock::new(Vec::new()),
            gameplay_system_builders: Mutex::new(Vec::new()),
            resource_builders: Mutex::new(Vec::new()),
            world_accessor: None,
        }
    }

    /// Set the world accessor.
    ///
    /// This must be called before passing the API to mods that use world().
    pub fn set_world_accessor(&mut self, accessor: Arc<WorldAccessor>) {
        self.world_accessor = Some(accessor);
    }

    /// Take ownership of the registries for insertion into the ECS world.
    ///
    /// This consumes the API and returns the registries.
    pub fn into_registries(
        self,
    ) -> (
        BlockBehaviorRegistry,
        EntityBehaviorRegistry,
        ItemBehaviorRegistry,
        ComponentProviderRegistry,
    ) {
        (
            self.block_behaviors,
            self.entity_behaviors,
            self.item_behaviors,
            self.component_providers,
        )
    }

    /// Take the tick systems for registration.
    pub fn take_tick_systems(&self) -> Vec<TickSystem> {
        std::mem::take(&mut *self.tick_systems.write())
    }

    /// Take the commands for registration.
    pub fn take_commands(&self) -> Vec<BuiltCommand> {
        std::mem::take(&mut *self.commands.write())
    }

    /// Take the gameplay system builders for registration.
    pub fn take_gameplay_system_builders(&self) -> Vec<ScheduleBuilder> {
        std::mem::take(&mut *self.gameplay_system_builders.lock())
    }

    /// Take the resource builders for registration.
    pub fn take_resource_builders(&self) -> Vec<ResourceBuilder> {
        std::mem::take(&mut *self.resource_builders.lock())
    }
}

impl Default for ServerApiImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl CoreApi for ServerApiImpl {
    fn register_block_behavior(&mut self, block_id: &str, behavior: Arc<dyn BlockBehavior>) {
        self.block_behaviors.register(block_id, behavior);
    }

    fn register_entity_behavior(&mut self, entity_type: &str, behavior: Arc<dyn EntityBehavior>) {
        self.entity_behaviors.register(entity_type, behavior);
    }

    fn register_item_behavior(&mut self, item_id: &str, behavior: Arc<dyn CollectibleBehavior>) {
        self.item_behaviors.register(item_id, behavior);
    }

    fn register_command(&mut self, command: BuiltCommand) {
        self.commands.write().push(command);
    }
}

impl ServerApi for ServerApiImpl {
    fn world(&self) -> &dyn WorldAccess {
        self.world_accessor
            .as_ref()
            .expect("WorldAccessor not initialized - world() called before server startup")
            .as_ref()
    }

    fn register_tick_system(&mut self, system: TickSystem) {
        self.tick_systems.write().push(system);
    }

    fn register_player_component_provider(&mut self, provider: Arc<dyn ComponentProvider>) {
        self.component_providers.register_player_provider(provider);
    }

    fn register_entity_component_provider(
        &mut self,
        entity_type: &str,
        provider: Arc<dyn ComponentProvider>,
    ) {
        self.component_providers
            .register_entity_provider(entity_type, provider);
    }

    fn register_gameplay_systems(&mut self, builder: ScheduleBuilder) {
        self.gameplay_system_builders.lock().push(builder);
    }

    fn register_resources(&mut self, builder: ResourceBuilder) {
        self.resource_builders.lock().push(builder);
    }
}
