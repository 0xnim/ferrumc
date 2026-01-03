//! World Accessor
//!
//! Provides read/write access to the game world for mods.

use std::sync::Arc;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::*;
use bevy_math::DVec3;
use ferrumc_core::transform::position::Position;
use ferrumc_state::GlobalState;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::errors::WorldError;
use ferrumc_world::pos::{BlockPos, ChunkPos};

/// Provides read/write access to the game world.
///
/// This is the primary interface for mods to interact with blocks,
/// chunks, and perform spatial queries.
///
/// # Example
///
/// ```ignore
/// fn start_server_side(&self, api: &mut dyn ServerApi) {
///     let world = api.world();
///
///     // Read a block
///     let block = world.get_block_and_fetch(BlockPos::new(0, 64, 0), "overworld")?;
///
///     // Set a block
///     world.set_block_and_fetch(BlockPos::new(0, 65, 0), "overworld", BlockStateId::new(1))?;
/// }
/// ```
pub struct WorldAccessor {
    state: GlobalState,
}

impl WorldAccessor {
    /// Create a new WorldAccessor wrapping the given GlobalState.
    pub fn new(state: GlobalState) -> Self {
        Self { state }
    }

    // =========================================================================
    // Block Access
    // =========================================================================

    /// Get block at position, loading chunk if needed.
    ///
    /// This is the most common way to read a block. It will automatically
    /// load the chunk from disk if it's not already in memory.
    ///
    /// # Arguments
    ///
    /// * `pos` - The block position
    /// * `dimension` - The dimension (e.g., "overworld", "nether", "end")
    ///
    /// # Returns
    ///
    /// The block state ID at the given position.
    pub fn get_block_and_fetch(
        &self,
        pos: BlockPos,
        dimension: &str,
    ) -> Result<BlockStateId, WorldError> {
        self.state.world.get_block_and_fetch(pos, dimension)
    }

    /// Set block at position, loading chunk if needed.
    ///
    /// This is the most common way to set a block. It will automatically
    /// load the chunk from disk if it's not already in memory.
    ///
    /// # Arguments
    ///
    /// * `pos` - The block position
    /// * `dimension` - The dimension (e.g., "overworld", "nether", "end")
    /// * `block` - The block state ID to set
    pub fn set_block_and_fetch(
        &self,
        pos: BlockPos,
        dimension: &str,
        block: BlockStateId,
    ) -> Result<(), WorldError> {
        self.state.world.set_block_and_fetch(pos, dimension, block)
    }

    // =========================================================================
    // Chunk Access
    // =========================================================================

    /// Check if a chunk exists (either in memory or on disk).
    ///
    /// Returns `true` if the chunk has been generated and saved.
    pub fn chunk_exists(&self, pos: ChunkPos, dimension: &str) -> Result<bool, WorldError> {
        self.state.world.chunk_exists(pos, dimension)
    }

    /// Check if a chunk is currently loaded in the memory cache.
    ///
    /// Returns `true` only if the chunk is actively cached, not just
    /// if it exists on disk.
    pub fn is_chunk_cached(&self, pos: ChunkPos, dimension: &str) -> bool {
        self.state
            .world
            .get_cache()
            .contains_key(&(pos, dimension.to_string()))
    }

    // =========================================================================
    // Terrain Generation
    // =========================================================================

    /// Generate a chunk at the given position if it doesn't exist.
    ///
    /// This will use the server's terrain generator to create a new chunk.
    /// If the chunk already exists, this is a no-op.
    ///
    /// # Arguments
    ///
    /// * `pos` - The chunk position to generate
    /// * `dimension` - The dimension (currently only "overworld" supports generation)
    pub fn load_or_generate(&self, pos: ChunkPos, dimension: &str) -> Result<(), WorldError> {
        if !self.state.world.chunk_exists(pos, dimension)? {
            let chunk = self.state.terrain_generator.generate_chunk(pos)?;
            self.state.world.insert_chunk(pos, dimension, chunk)?;
        }
        Ok(())
    }

    // =========================================================================
    // Entity Queries
    // =========================================================================

    /// Get all entities within a radius of a position.
    ///
    /// This queries the Bevy ECS world for all entities with a Position
    /// component that are within the specified distance from the center point.
    ///
    /// # Arguments
    ///
    /// * `bevy_world` - The Bevy ECS world (from tick system or behavior context)
    /// * `center` - The center point to search from
    /// * `radius` - The search radius in blocks
    ///
    /// # Returns
    ///
    /// A vector of entity IDs that are within the radius.
    pub fn get_entities_in_radius(
        &self,
        bevy_world: &World,
        center: DVec3,
        radius: f64,
    ) -> Vec<Entity> {
        let mut result = Vec::new();
        let mut query = bevy_world.query::<(Entity, &Position)>();

        for (entity, pos) in query.iter(bevy_world) {
            let distance = pos.coords.distance(center);
            if distance <= radius {
                result.push(entity);
            }
        }
        result
    }

    /// Get all entities within a bounding box.
    ///
    /// This queries the Bevy ECS world for all entities with a Position
    /// component that are within the specified axis-aligned bounding box.
    ///
    /// # Arguments
    ///
    /// * `bevy_world` - The Bevy ECS world
    /// * `min` - The minimum corner of the bounding box
    /// * `max` - The maximum corner of the bounding box
    ///
    /// # Returns
    ///
    /// A vector of entity IDs that are within the bounding box.
    pub fn get_entities_in_box(
        &self,
        bevy_world: &World,
        min: DVec3,
        max: DVec3,
    ) -> Vec<Entity> {
        let mut result = Vec::new();
        let mut query = bevy_world.query::<(Entity, &Position)>();

        for (entity, pos) in query.iter(bevy_world) {
            let p = pos.coords;
            if p.x >= min.x
                && p.x <= max.x
                && p.y >= min.y
                && p.y <= max.y
                && p.z >= min.z
                && p.z <= max.z
            {
                result.push(entity);
            }
        }
        result
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Get direct access to the underlying GlobalState.
    ///
    /// This is an escape hatch for advanced use cases that need
    /// direct access to server internals.
    pub fn state(&self) -> &GlobalState {
        &self.state
    }
}

/// Bevy resource that provides world access to ECS systems.
///
/// This is inserted as a resource during server startup and can be
/// accessed via `Res<WorldAccessorResource>` in system parameters.
#[derive(Resource, Clone)]
pub struct WorldAccessorResource(pub Arc<WorldAccessor>);

impl WorldAccessorResource {
    /// Create a new WorldAccessorResource from GlobalState.
    pub fn new(state: GlobalState) -> Self {
        Self(Arc::new(WorldAccessor::new(state)))
    }
}
