//! Ring-based chunk loading component for gradual chunk delivery.
//! 
//! This component tracks the state of chunk loading for each player,
//! implementing a vanilla-like spiral/ring loading pattern where chunks
//! are loaded from the center outward in concentric rings.

use bevy_ecs::prelude::Component;
use std::collections::VecDeque;

/// Priority level for chunk loading operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkLoadPriority {
    /// Immediate loading needed (player movement)
    High,
    /// Render distance change or respawn
    Medium,
    /// Background pre-loading
    Low,
}

/// A single chunk loading request with its priority and ring information
#[derive(Debug, Clone)]
pub struct ChunkLoadRequest {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub dimension: String,
    pub priority: ChunkLoadPriority,
    pub ring: u32,
}

/// Component that manages gradual chunk loading in rings around a player.
/// 
/// This implements vanilla-like chunk loading behavior where chunks are
/// sent in concentric rings from the player's center position outward.
#[derive(Component)]
pub struct ChunkLoadingRings {
    /// Center position of the chunk loading (player's chunk position)
    pub center: (i32, i32),
    /// Maximum ring to load (based on render distance)
    pub max_ring: u32,
    /// Queue of chunks to be loaded, ordered by priority and ring
    pub pending_chunks: VecDeque<ChunkLoadRequest>,
    /// Current priority being processed
    pub current_priority: ChunkLoadPriority,
    /// Maximum chunks to process per tick
    pub chunks_per_tick: u32,
    /// Whether this loading operation is active
    pub active: bool,
}

impl ChunkLoadingRings {
    /// Create a new chunk loading rings component
    pub fn new(center: (i32, i32), render_distance: u32) -> Self {
        Self {
            center,
            max_ring: render_distance,
            pending_chunks: VecDeque::new(),
            current_priority: ChunkLoadPriority::Medium,
            chunks_per_tick: 16, // Default rate
            active: false,
        }
    }

    /// Add chunks to be loaded in a ring pattern around the center
    pub fn add_ring_chunks(&mut self, center: (i32, i32), radius: u32, priority: ChunkLoadPriority, dimension: String) {
        self.center = center;
        
        if radius == 0 {
            // Special case: just the center chunk
            self.pending_chunks.push_back(ChunkLoadRequest {
                chunk_x: center.0,
                chunk_z: center.1,
                dimension,
                priority,
                ring: 0,
            });
            return;
        }

        // Generate chunks in rings from center outward
        for ring in 0..=radius {
            let ring_chunks = generate_ring_chunks(center, ring, &dimension, priority);
            
            // Insert chunks based on priority (high priority goes to front)
            match priority {
                ChunkLoadPriority::High => {
                    // Add to front of queue
                    for chunk in ring_chunks.into_iter().rev() {
                        self.pending_chunks.push_front(chunk);
                    }
                }
                _ => {
                    // Add to back of queue
                    for chunk in ring_chunks {
                        self.pending_chunks.push_back(chunk);
                    }
                }
            }
        }
        
        self.max_ring = radius;
        self.active = true;
    }

    /// Add chunks that are missing between two render distances
    pub fn add_missing_chunks(&mut self, center: (i32, i32), old_radius: u32, new_radius: u32, priority: ChunkLoadPriority, dimension: String) {
        self.center = center;
        
        // Only add chunks that are in the new radius but not in the old radius
        for ring in old_radius + 1..=new_radius {
            let ring_chunks = generate_ring_chunks(center, ring, &dimension, priority);
            
            match priority {
                ChunkLoadPriority::High => {
                    for chunk in ring_chunks.into_iter().rev() {
                        self.pending_chunks.push_front(chunk);
                    }
                }
                _ => {
                    for chunk in ring_chunks {
                        self.pending_chunks.push_back(chunk);
                    }
                }
            }
        }
        
        self.max_ring = new_radius;
        self.active = true;
    }

    /// Get the next batch of chunks to process (up to chunks_per_tick)
    pub fn get_next_batch(&mut self) -> Vec<ChunkLoadRequest> {
        let mut batch = Vec::new();
        
        for _ in 0..self.chunks_per_tick {
            if let Some(chunk_request) = self.pending_chunks.pop_front() {
                batch.push(chunk_request);
            } else {
                break;
            }
        }

        // If no more chunks, mark as inactive
        if self.pending_chunks.is_empty() {
            self.active = false;
        }

        batch
    }

    /// Check if there are chunks pending
    pub fn has_pending_chunks(&self) -> bool {
        !self.pending_chunks.is_empty()
    }

    /// Clear all pending chunks
    pub fn clear(&mut self) {
        self.pending_chunks.clear();
        self.active = false;
    }
}

/// Generate chunks for a specific ring around a center point
fn generate_ring_chunks(center: (i32, i32), ring: u32, dimension: &str, priority: ChunkLoadPriority) -> Vec<ChunkLoadRequest> {
    let mut chunks = Vec::new();
    let ring = ring as i32;
    
    if ring == 0 {
        // Center chunk
        chunks.push(ChunkLoadRequest {
            chunk_x: center.0,
            chunk_z: center.1,
            dimension: dimension.to_string(),
            priority,
            ring: 0,
        });
        return chunks;
    }

    // Generate the square ring
    let (cx, cz) = center;
    
    // Top and bottom rows
    for x in (cx - ring)..=(cx + ring) {
        // Top row
        chunks.push(ChunkLoadRequest {
            chunk_x: x,
            chunk_z: cz - ring,
            dimension: dimension.to_string(),
            priority,
            ring: ring as u32,
        });
        
        // Bottom row
        chunks.push(ChunkLoadRequest {
            chunk_x: x,
            chunk_z: cz + ring,
            dimension: dimension.to_string(),
            priority,
            ring: ring as u32,
        });
    }
    
    // Left and right columns (excluding corners already done)
    for z in (cz - ring + 1)..=(cz + ring - 1) {
        // Left column
        chunks.push(ChunkLoadRequest {
            chunk_x: cx - ring,
            chunk_z: z,
            dimension: dimension.to_string(),
            priority,
            ring: ring as u32,
        });
        
        // Right column
        chunks.push(ChunkLoadRequest {
            chunk_x: cx + ring,
            chunk_z: z,
            dimension: dimension.to_string(),
            priority,
            ring: ring as u32,
        });
    }

    // Sort by distance from center for better loading order
    chunks.sort_by(|a, b| {
        let dist_a = ((a.chunk_x - cx).pow(2) + (a.chunk_z - cz).pow(2)) as f64;
        let dist_b = ((b.chunk_x - cx).pow(2) + (b.chunk_z - cz).pow(2)) as f64;
        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    chunks
}
