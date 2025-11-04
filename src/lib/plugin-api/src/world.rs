//! World manipulation helpers for plugins

use ferrumc_state::GlobalState;
use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::chunk_format::Chunk;
use ferrumc_world::errors::WorldError as WorldCrateError;
use ferrumc_world_gen::errors::WorldGenError;
use std::sync::Arc;

/// Helper trait providing convenient world manipulation methods for plugins.
///
/// This trait extends the world access with high-level operations that
/// automatically handle chunk loading, saving, and error handling.
pub trait WorldExt {
    /// Get a chunk, loading from database if necessary.
    ///
    /// # Arguments
    ///
    /// * `chunk_x` - Chunk X coordinate (block X / 16)
    /// * `chunk_z` - Chunk Z coordinate (block Z / 16)
    /// * `dimension` - Dimension name (e.g., "overworld", "nether", "the_end")
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let chunk = state.world.get_chunk(0, 0, "overworld")?;
    /// ```
    fn get_chunk(
        &self,
        chunk_x: i32,
        chunk_z: i32,
        dimension: &str,
    ) -> Result<Arc<Chunk>, WorldError>;

    /// Set a block at world coordinates, automatically loading/saving chunks.
    ///
    /// # Arguments
    ///
    /// * `x` - Block X coordinate
    /// * `y` - Block Y coordinate
    /// * `z` - Block Z coordinate
    /// * `block` - Block state ID to set
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// state.set_block_at(100, 64, 200, BlockStateId::STONE)?;
    /// ```
    fn set_block_at(&self, x: i32, y: i32, z: i32, block: BlockStateId) -> Result<(), WorldError>;

    /// Get a block at world coordinates, automatically loading chunk if needed.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let block = state.get_block_at(100, 64, 200)?;
    /// ```
    fn get_block_at(&self, x: i32, y: i32, z: i32) -> Result<BlockStateId, WorldError>;

    /// Check if a chunk exists in the database.
    fn chunk_exists(&self, chunk_x: i32, chunk_z: i32, dimension: &str) -> bool;
}

impl WorldExt for GlobalState {
    fn get_chunk(
        &self,
        chunk_x: i32,
        chunk_z: i32,
        dimension: &str,
    ) -> Result<Arc<Chunk>, WorldError> {
        self.world
            .load_chunk(chunk_x, chunk_z, dimension)
            .map_err(|e: WorldCrateError| WorldError::ChunkLoadFailed {
                x: chunk_x,
                z: chunk_z,
                dimension: dimension.to_string(),
                source: e.to_string(),
            })
    }

    fn set_block_at(&self, x: i32, y: i32, z: i32, block: BlockStateId) -> Result<(), WorldError> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let relative_x = x & 0xF;
        let relative_z = z & 0xF;

        let mut chunk = self
            .world
            .load_chunk_owned(chunk_x, chunk_z, "overworld")
            .or_else(|_: WorldCrateError| {
                // If chunk doesn't exist, generate it
                self.terrain_generator.generate_chunk(chunk_x, chunk_z)
            })
            .map_err(|e: WorldGenError| WorldError::ChunkLoadFailed {
                x: chunk_x,
                z: chunk_z,
                dimension: "overworld".to_string(),
                source: e.to_string(),
            })?;

        chunk
            .set_block(relative_x, y, relative_z, block)
            .map_err(|e: WorldCrateError| WorldError::BlockUpdateFailed {
                x,
                y,
                z,
                source: e.to_string(),
            })?;

        self.world
            .save_chunk(Arc::new(chunk))
            .map_err(|e: WorldCrateError| WorldError::ChunkSaveFailed {
                x: chunk_x,
                z: chunk_z,
                source: e.to_string(),
            })?;

        Ok(())
    }

    fn get_block_at(&self, x: i32, y: i32, z: i32) -> Result<BlockStateId, WorldError> {
        let chunk_x = x >> 4;
        let chunk_z = z >> 4;
        let relative_x = x & 0xF;
        let relative_z = z & 0xF;

        let chunk = self
            .world
            .load_chunk(chunk_x, chunk_z, "overworld")
            .map_err(|e| WorldError::ChunkLoadFailed {
                x: chunk_x,
                z: chunk_z,
                dimension: "overworld".to_string(),
                source: e.to_string(),
            })?;

        chunk
            .get_block(relative_x, y, relative_z)
            .map_err(|e: WorldCrateError| WorldError::BlockReadFailed {
                x,
                y,
                z,
                source: e.to_string(),
            })
    }

    fn chunk_exists(&self, chunk_x: i32, chunk_z: i32, dimension: &str) -> bool {
        self.world
            .chunk_exists(chunk_x, chunk_z, dimension)
            .unwrap_or(false)
    }
}

/// Errors that can occur during world operations
#[derive(Debug)]
pub enum WorldError {
    ChunkLoadFailed {
        x: i32,
        z: i32,
        dimension: String,
        source: String,
    },
    ChunkSaveFailed {
        x: i32,
        z: i32,
        source: String,
    },
    BlockUpdateFailed {
        x: i32,
        y: i32,
        z: i32,
        source: String,
    },
    BlockReadFailed {
        x: i32,
        y: i32,
        z: i32,
        source: String,
    },
}

impl std::fmt::Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldError::ChunkLoadFailed {
                x,
                z,
                dimension,
                source,
            } => {
                write!(
                    f,
                    "Failed to load chunk ({}, {}) in {}: {}",
                    x, z, dimension, source
                )
            }
            WorldError::ChunkSaveFailed { x, z, source } => {
                write!(f, "Failed to save chunk ({}, {}): {}", x, z, source)
            }
            WorldError::BlockUpdateFailed { x, y, z, source } => {
                write!(
                    f,
                    "Failed to update block at ({}, {}, {}): {}",
                    x, y, z, source
                )
            }
            WorldError::BlockReadFailed { x, y, z, source } => {
                write!(
                    f,
                    "Failed to read block at ({}, {}, {}): {}",
                    x, y, z, source
                )
            }
        }
    }
}

impl std::error::Error for WorldError {}
