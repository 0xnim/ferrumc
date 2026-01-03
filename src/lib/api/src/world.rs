//! World Access API
//!
//! Defines the trait for accessing the game world from mods.

use ferrumc_world::block_state_id::BlockStateId;
use ferrumc_world::errors::WorldError;
use ferrumc_world::pos::{BlockPos, ChunkPos};

/// Game dimension/world type.
///
/// Use this instead of string literals to avoid typos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Dimension {
    /// The main overworld dimension
    #[default]
    Overworld,
    /// The nether dimension
    Nether,
    /// The end dimension
    TheEnd,
}

impl Dimension {
    /// Get the string identifier for this dimension.
    pub fn as_str(&self) -> &'static str {
        match self {
            Dimension::Overworld => "overworld",
            Dimension::Nether => "the_nether",
            Dimension::TheEnd => "the_end",
        }
    }
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Trait for accessing the game world.
///
/// This is the primary interface for mods to read and modify blocks,
/// check chunk status, and generate terrain.
///
/// # Example
///
/// ```ignore
/// fn start_server_side(&self, api: &mut dyn ServerApi) {
///     let world = api.world();
///
///     // Read a block
///     let block = world.get_block(BlockPos::of(0, 64, 0), Dimension::Overworld)?;
///
///     // Set a block
///     world.set_block(BlockPos::of(0, 65, 0), Dimension::Overworld, BlockStateId::AIR)?;
/// }
/// ```
pub trait WorldAccess: Send + Sync {
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
    /// * `dimension` - The dimension
    ///
    /// # Returns
    ///
    /// The block state ID at the given position.
    fn get_block(&self, pos: BlockPos, dimension: Dimension) -> Result<BlockStateId, WorldError>;

    /// Set block at position, loading chunk if needed.
    ///
    /// This is the most common way to set a block. It will automatically
    /// load the chunk from disk if it's not already in memory.
    ///
    /// # Arguments
    ///
    /// * `pos` - The block position
    /// * `dimension` - The dimension
    /// * `block` - The block state ID to set
    fn set_block(
        &self,
        pos: BlockPos,
        dimension: Dimension,
        block: BlockStateId,
    ) -> Result<(), WorldError>;

    // =========================================================================
    // Chunk Access
    // =========================================================================

    /// Check if a chunk exists (either in memory or on disk).
    ///
    /// Returns `true` if the chunk has been generated and saved.
    fn chunk_exists(&self, pos: ChunkPos, dimension: Dimension) -> Result<bool, WorldError>;

    /// Check if a chunk is currently loaded in the memory cache.
    ///
    /// Returns `true` only if the chunk is actively cached, not just
    /// if it exists on disk.
    fn is_chunk_cached(&self, pos: ChunkPos, dimension: Dimension) -> bool;

    // =========================================================================
    // Terrain Generation
    // =========================================================================

    /// Load or generate a chunk at the given position.
    ///
    /// This will use the server's terrain generator to create a new chunk
    /// if it doesn't already exist.
    ///
    /// # Arguments
    ///
    /// * `pos` - The chunk position
    /// * `dimension` - The dimension
    fn load_or_generate(&self, pos: ChunkPos, dimension: Dimension) -> Result<(), WorldError>;
}
