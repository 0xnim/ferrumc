//! Safe world queries for plugins
//!
//! Provides READ-ONLY access to world data.
//! Does NOT allow modifying blocks, chunks, or database.

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_state::GlobalStateResource;
use ferrumc_world::block_state_id::BlockStateId;

use crate::world::WorldExt;

/// Read-only world queries for plugins
///
/// Provides safe, read-only access to world data:
/// - Block queries (get block, check solid, etc.)
/// - Chunk queries (is loaded, get biome, etc.)
///
/// Does NOT allow:
/// - Setting blocks (use BlockAPI instead)
/// - Direct chunk modification (use BlockAPI instead)
/// - Database access (not allowed in plugins)
///
/// # Example
///
/// ```rust,no_run
/// fn my_system(world: WorldQueries) {
///     // Check what block is at a position
///     if let Some(block) = world.get_block(10, 64, 20) {
///         println!("Block: {:?}", block);
///     }
///     
///     // Check if chunk is loaded
///     if world.chunk_loaded(0, 0) {
///         println!("Spawn chunk is loaded");
///     }
/// }
/// ```
#[derive(SystemParam)]
pub struct WorldQueries<'w> {
    state: Res<'w, GlobalStateResource>,
}

impl<'w> WorldQueries<'w> {
    /// Get block at position (read-only)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if let Some(block) = world.get_block(10, 64, 20) {
    ///     println!("Block at 10,64,20: {:?}", block);
    /// }
    /// ```
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<BlockStateId> {
        let (cx, cz) = Self::to_chunk_coords(x, z);
        let chunk = self.state.0.get_chunk(cx, cz, "overworld").ok()?;
        chunk.get_block(x, y, z).ok()
    }
    
    /// Check if chunk is loaded
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if world.chunk_loaded(0, 0) {
    ///     println!("Spawn chunk is loaded");
    /// }
    /// ```
    pub fn chunk_loaded(&self, chunk_x: i32, chunk_z: i32) -> bool {
        self.state.0.chunk_exists(chunk_x, chunk_z, "overworld")
    }
    
    /// Check if a block position is in a loaded chunk
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if world.is_loaded(100, 64, 200) {
    ///     // Can safely query blocks at this position
    /// }
    /// ```
    pub fn is_loaded(&self, x: i32, _y: i32, z: i32) -> bool {
        let (cx, cz) = Self::to_chunk_coords(x, z);
        self.chunk_loaded(cx, cz)
    }
    
    /// Convert block coordinates to chunk coordinates
    fn to_chunk_coords(x: i32, z: i32) -> (i32, i32) {
        (x >> 4, z >> 4)
    }
    
    /// Convert chunk coordinates to block coordinates (chunk origin)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let (block_x, block_z) = WorldQueries::chunk_to_block_coords(0, 0);
    /// // Returns (0, 0) - the origin block of chunk 0,0
    /// ```
    pub fn chunk_to_block_coords(chunk_x: i32, chunk_z: i32) -> (i32, i32) {
        (chunk_x << 4, chunk_z << 4)
    }
    
    /// Get the highest non-air block at x, z coordinates
    ///
    /// Useful for finding ground level.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if let Some(y) = world.get_highest_block(10, 20) {
    ///     println!("Ground level at 10,20 is y={}", y);
    /// }
    /// ```
    pub fn get_highest_block(&self, x: i32, z: i32) -> Option<i32> {
        let (cx, cz) = Self::to_chunk_coords(x, z);
        let chunk = self.state.0.get_chunk(cx, cz, "overworld").ok()?;
        
        // Search from top down for first non-air block
        for y in (0..=255).rev() {
            if let Ok(block) = chunk.get_block(x, y, z) {
                if block.0 != 0 {
                    // Not air
                    return Some(y);
                }
            }
        }
        
        None
    }
}
