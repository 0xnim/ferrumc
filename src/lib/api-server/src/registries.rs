//! Behavior and component registries.
//!
//! These registries store behaviors and component providers
//! registered by mods, and are inserted as Bevy resources.

use std::sync::Arc;

use bevy_ecs::prelude::*;
use dashmap::DashMap;
use parking_lot::RwLock;

use ferrumc_api::behavior::{BlockBehavior, CollectibleBehavior, EntityBehavior, ItemStack};
use ferrumc_api::behavior::BlockContext;
use ferrumc_api::provider::ComponentProvider;

/// Registry for block behaviors.
///
/// Stores behaviors keyed by block ID. Use "*" for behaviors
/// that apply to all blocks.
#[derive(Resource, Default)]
pub struct BlockBehaviorRegistry {
    behaviors: DashMap<String, Vec<Arc<dyn BlockBehavior>>>,
    /// Behaviors that apply to all blocks
    global_behaviors: RwLock<Vec<Arc<dyn BlockBehavior>>>,
}

impl BlockBehaviorRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a behavior for a block ID.
    ///
    /// Use "*" to register a behavior that applies to all blocks.
    pub fn register(&self, block_id: &str, behavior: Arc<dyn BlockBehavior>) {
        if block_id == "*" {
            self.global_behaviors.write().push(behavior);
        } else {
            self.behaviors
                .entry(block_id.to_string())
                .or_default()
                .push(behavior);
        }
    }

    /// Get all behaviors for a block (including global behaviors).
    pub fn get_behaviors(&self, block_id: &str) -> Vec<Arc<dyn BlockBehavior>> {
        let mut result = self.global_behaviors.read().clone();

        if let Some(specific) = self.behaviors.get(block_id) {
            result.extend(specific.iter().cloned());
        }

        result
    }

    /// Check if any behaviors are registered for a block.
    pub fn has_behaviors(&self, block_id: &str) -> bool {
        !self.global_behaviors.read().is_empty() || self.behaviors.contains_key(block_id)
    }

    /// Get hardness override from behaviors.
    ///
    /// Returns the first non-None hardness value from behaviors,
    /// checking global behaviors first, then block-specific behaviors.
    pub fn get_hardness(&self, block_id: &str, ctx: &BlockContext<'_>) -> Option<f32> {
        // Check global behaviors first
        for behavior in self.global_behaviors.read().iter() {
            if let Some(hardness) = behavior.get_hardness(ctx) {
                return Some(hardness);
            }
        }

        // Then check block-specific behaviors
        if let Some(behaviors) = self.behaviors.get(block_id) {
            for behavior in behaviors.iter() {
                if let Some(hardness) = behavior.get_hardness(ctx) {
                    return Some(hardness);
                }
            }
        }

        None
    }

    /// Get drops override from behaviors.
    ///
    /// Returns the first non-None drops value from behaviors.
    pub fn get_drops(&self, block_id: &str, ctx: &BlockContext<'_>) -> Option<Vec<ItemStack>> {
        // Check global behaviors first
        for behavior in self.global_behaviors.read().iter() {
            if let Some(drops) = behavior.get_drops(ctx) {
                return Some(drops);
            }
        }

        // Then check block-specific behaviors
        if let Some(behaviors) = self.behaviors.get(block_id) {
            for behavior in behaviors.iter() {
                if let Some(drops) = behavior.get_drops(ctx) {
                    return Some(drops);
                }
            }
        }

        None
    }
}

/// Registry for entity behaviors.
///
/// Stores behaviors keyed by entity type (e.g., "player", "pig").
#[derive(Resource, Default)]
pub struct EntityBehaviorRegistry {
    behaviors: DashMap<String, Vec<Arc<dyn EntityBehavior>>>,
    /// Behaviors that apply to all entities
    global_behaviors: RwLock<Vec<Arc<dyn EntityBehavior>>>,
}

impl EntityBehaviorRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a behavior for an entity type.
    ///
    /// Use "*" to register a behavior that applies to all entities.
    /// Use "player" for player-specific behaviors.
    pub fn register(&self, entity_type: &str, behavior: Arc<dyn EntityBehavior>) {
        if entity_type == "*" {
            self.global_behaviors.write().push(behavior);
        } else {
            self.behaviors
                .entry(entity_type.to_string())
                .or_default()
                .push(behavior);
        }
    }

    /// Get all behaviors for an entity type (including global behaviors).
    pub fn get_behaviors(&self, entity_type: &str) -> Vec<Arc<dyn EntityBehavior>> {
        let mut result = self.global_behaviors.read().clone();

        if let Some(specific) = self.behaviors.get(entity_type) {
            result.extend(specific.iter().cloned());
        }

        result
    }

    /// Check if any behaviors are registered for an entity type.
    pub fn has_behaviors(&self, entity_type: &str) -> bool {
        !self.global_behaviors.read().is_empty() || self.behaviors.contains_key(entity_type)
    }
}

/// Registry for item/collectible behaviors.
///
/// Stores behaviors keyed by item ID.
#[derive(Resource, Default)]
pub struct ItemBehaviorRegistry {
    behaviors: DashMap<String, Vec<Arc<dyn CollectibleBehavior>>>,
    /// Behaviors that apply to all items
    global_behaviors: RwLock<Vec<Arc<dyn CollectibleBehavior>>>,
}

impl ItemBehaviorRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a behavior for an item ID.
    ///
    /// Use "*" to register a behavior that applies to all items.
    pub fn register(&self, item_id: &str, behavior: Arc<dyn CollectibleBehavior>) {
        if item_id == "*" {
            self.global_behaviors.write().push(behavior);
        } else {
            self.behaviors
                .entry(item_id.to_string())
                .or_default()
                .push(behavior);
        }
    }

    /// Get all behaviors for an item (including global behaviors).
    pub fn get_behaviors(&self, item_id: &str) -> Vec<Arc<dyn CollectibleBehavior>> {
        let mut result = self.global_behaviors.read().clone();

        if let Some(specific) = self.behaviors.get(item_id) {
            result.extend(specific.iter().cloned());
        }

        result
    }

    /// Check if any behaviors are registered for an item.
    pub fn has_behaviors(&self, item_id: &str) -> bool {
        !self.global_behaviors.read().is_empty() || self.behaviors.contains_key(item_id)
    }
}

/// Registry for component providers.
///
/// Stores providers that add components to entities during creation.
#[derive(Resource, Default)]
pub struct ComponentProviderRegistry {
    /// Providers for player entities
    player_providers: RwLock<Vec<Arc<dyn ComponentProvider>>>,
    /// Providers for specific entity types
    entity_providers: DashMap<String, Vec<Arc<dyn ComponentProvider>>>,
}

impl ComponentProviderRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a provider for player entities.
    pub fn register_player_provider(&self, provider: Arc<dyn ComponentProvider>) {
        self.player_providers.write().push(provider);
    }

    /// Register a provider for a specific entity type.
    pub fn register_entity_provider(&self, entity_type: &str, provider: Arc<dyn ComponentProvider>) {
        self.entity_providers
            .entry(entity_type.to_string())
            .or_default()
            .push(provider);
    }

    /// Get all providers for player entities.
    pub fn get_player_providers(&self) -> Vec<Arc<dyn ComponentProvider>> {
        self.player_providers.read().clone()
    }

    /// Get all providers for an entity type.
    pub fn get_entity_providers(&self, entity_type: &str) -> Vec<Arc<dyn ComponentProvider>> {
        self.entity_providers
            .get(entity_type)
            .map(|v| v.clone())
            .unwrap_or_default()
    }
}
