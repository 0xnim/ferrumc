//! Context types for API methods.
//!
//! These provide information about the current state when
//! behavior methods or event handlers are called.

use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;

/// Context for world access operations.
pub struct WorldContext<'w> {
    world: &'w mut World,
}

impl<'w> WorldContext<'w> {
    pub fn new(world: &'w mut World) -> Self {
        Self { world }
    }

    /// Get read-only access to a component on an entity.
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.world.get::<T>(entity)
    }

    /// Get mutable access to a component on an entity.
    pub fn get_mut<T: Component<Mutability = Mutable>>(
        &mut self,
        entity: Entity,
    ) -> Option<Mut<'_, T>> {
        self.world.get_mut::<T>(entity)
    }

    /// Check if an entity has a component.
    pub fn has<T: Component>(&self, entity: Entity) -> bool {
        self.world.get::<T>(entity).is_some()
    }

    /// Get the underlying Bevy world for advanced operations.
    pub fn world(&self) -> &World {
        self.world
    }

    /// Get mutable access to the underlying Bevy world.
    pub fn world_mut(&mut self) -> &mut World {
        self.world
    }
}
