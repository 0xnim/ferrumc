//! Safe entity queries for plugins
//!
//! Provides access to safe entity components only.
//! Does NOT allow querying StreamWriter, database, or other sensitive components.

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_core::collisions::bounds::CollisionBounds;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;

/// Safe entity queries for plugins
///
/// ONLY allows querying safe components:
/// - Position
/// - Rotation
/// - PlayerIdentity
///
/// Does NOT allow:
/// - StreamWriter (network access)
/// - GlobalState (database access)
/// - Other sensitive components
///
/// # Example
///
/// ```rust,no_run
/// fn my_system(entities: EntityQueries) {
///     // Get position of an entity
///     if let Some(pos) = entities.position(player) {
///         println!("Player at {:?}", pos);
///     }
///     
///     // Find nearby players
///     let nearby = entities.players_in_range(&center_pos, 100.0);
/// }
/// ```
#[derive(SystemParam)]
pub struct EntityQueries<'w, 's> {
    /// Query for positions
    positions: Query<'w, 's, &'static Position>,
    
    /// Query for rotations
    rotations: Query<'w, 's, &'static Rotation>,
    
    /// Query for player identities
    identities: Query<'w, 's, &'static PlayerIdentity>,
    
    /// Combined query for all player data
    players: Query<'w, 's, (Entity, &'static Position, &'static PlayerIdentity)>,
    
    /// Combined query for position and rotation
    transforms: Query<'w, 's, (Entity, &'static Position, &'static Rotation)>,
    
    /// Query for collision bounds
    collision_bounds: Query<'w, 's, &'static CollisionBounds>,
    
    /// Combined query for position and collision bounds
    physics: Query<'w, 's, (Entity, &'static Position, &'static CollisionBounds)>,
}

impl<'w, 's> EntityQueries<'w, 's> {
    /// Get entity position
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if let Some(pos) = entities.position(player_entity) {
    ///     println!("Player at {}, {}, {}", pos.x, pos.y, pos.z);
    /// }
    /// ```
    pub fn position(&self, entity: Entity) -> Option<&Position> {
        self.positions.get(entity).ok()
    }
    
    /// Get entity rotation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if let Some(rot) = entities.rotation(player_entity) {
    ///     println!("Yaw: {}, Pitch: {}", rot.yaw, rot.pitch);
    /// }
    /// ```
    pub fn rotation(&self, entity: Entity) -> Option<&Rotation> {
        self.rotations.get(entity).ok()
    }
    
    /// Get player identity
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if let Some(identity) = entities.identity(player_entity) {
    ///     println!("Player: {}", identity.username);
    /// }
    /// ```
    pub fn identity(&self, entity: Entity) -> Option<&PlayerIdentity> {
        self.identities.get(entity).ok()
    }
    
    /// Iterate all players
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// for (entity, pos, identity) in entities.iter_players() {
    ///     println!("{} is at {:?}", identity.username, pos);
    /// }
    /// ```
    pub fn iter_players(&self) -> impl Iterator<Item = (Entity, &Position, &PlayerIdentity)> {
        self.players.iter()
    }
    
    /// Iterate all entities with transforms
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// for (entity, pos, rot) in entities.iter_transforms() {
    ///     // Process entities with position and rotation
    /// }
    /// ```
    pub fn iter_transforms(&self) -> impl Iterator<Item = (Entity, &Position, &Rotation)> {
        self.transforms.iter()
    }
    
    /// Find players within range of a position
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let center = Position { x: 0.0, y: 64.0, z: 0.0 };
    /// let nearby = entities.players_in_range(&center, 100.0);
    /// println!("Found {} players nearby", nearby.len());
    /// ```
    pub fn players_in_range(&self, center: &Position, range: f64) -> Vec<Entity> {
        let range_sq = range * range;
        
        self.players
            .iter()
            .filter(|(_, pos, _)| {
                let dx = pos.x - center.x;
                let dy = pos.y - center.y;
                let dz = pos.z - center.z;
                dx * dx + dy * dy + dz * dz <= range_sq
            })
            .map(|(entity, _, _)| entity)
            .collect()
    }
    
    /// Count total players
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let count = entities.player_count();
    /// println!("Server has {} players", count);
    /// ```
    pub fn player_count(&self) -> usize {
        self.players.iter().count()
    }
    
    /// Get distance between two entities
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if let Some(dist) = entities.distance_between(player1, player2) {
    ///     println!("Distance: {} blocks", dist);
    /// }
    /// ```
    pub fn distance_between(&self, entity1: Entity, entity2: Entity) -> Option<f64> {
        let pos1 = self.position(entity1)?;
        let pos2 = self.position(entity2)?;
        
        let dx = pos1.x - pos2.x;
        let dy = pos1.y - pos2.y;
        let dz = pos1.z - pos2.z;
        
        Some((dx * dx + dy * dy + dz * dz).sqrt())
    }
    
    /// Get collision bounds for entity
    pub fn collision_bounds(&self, entity: Entity) -> Option<&CollisionBounds> {
        self.collision_bounds.get(entity).ok()
    }
    
    /// Iterate all entities with physics (position + collision)
    pub fn iter_physics(&self) -> impl Iterator<Item = (Entity, &Position, &CollisionBounds)> {
        self.physics.iter()
    }
    
    /// Check if a block position would collide with any entity
    ///
    /// Useful for validating block placement.
    pub fn would_collide_with_block(&self, block_x: i32, block_y: i32, block_z: i32) -> bool {
        let block_bounds = CollisionBounds {
            x_offset_start: 0.0,
            x_offset_end: 1.0,
            y_offset_start: 0.0,
            y_offset_end: 1.0,
            z_offset_start: 0.0,
            z_offset_end: 1.0,
        };
        
        self.physics.iter().any(|(_, pos, entity_bounds)| {
            entity_bounds.collides(
                (pos.x, pos.y, pos.z),
                &block_bounds,
                (block_x as f64, block_y as f64, block_z as f64),
            )
        })
    }
    
    /// Find a player by username
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// if let Some((entity, identity)) = entities.find_player_by_name("Notch") {
    ///     println!("Found {} with entity {:?}", identity.username, entity);
    /// }
    /// ```
    pub fn find_player_by_name(&self, username: &str) -> Option<(Entity, &PlayerIdentity)> {
        self.players
            .iter()
            .find(|(_, _, identity)| identity.username == username)
            .map(|(entity, _, identity)| (entity, identity))
    }
}
