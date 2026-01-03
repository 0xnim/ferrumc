//! Component providers for entity setup.
//!
//! Mods use component providers to add their components to
//! entities when they are created (e.g., players joining).

use bevy_ecs::prelude::*;
use bevy_ecs::system::EntityCommands;

/// Context for player entity setup.
#[derive(Debug)]
pub struct PlayerSetupContext {
    /// Player's unique ID
    pub uuid: u128,
    /// Player's username
    pub username: String,
    /// Whether this is a new player (first join ever)
    pub is_new_player: bool,
    /// Cached player data from previous session (if any)
    pub cached_data: Option<CachedPlayerData>,
}

/// Cached data from a player's previous session.
#[derive(Debug, Clone, Default)]
pub struct CachedPlayerData {
    /// Last known position
    pub position: Option<(f64, f64, f64)>,
    /// Last known rotation
    pub rotation: Option<(f32, f32)>,
    /// Game mode (0=survival, 1=creative, 2=adventure, 3=spectator)
    pub game_mode: Option<u8>,
    /// Custom mod data (serialized)
    pub mod_data: std::collections::HashMap<String, Vec<u8>>,
}

/// Context for non-player entity setup.
#[derive(Debug)]
pub struct EntitySetupContext {
    /// Entity type identifier (e.g., "minecraft:pig")
    pub entity_type: String,
    /// Spawn position
    pub position: (f64, f64, f64),
    /// Custom spawn data (from spawn egg, spawner, etc.)
    pub spawn_data: Option<Vec<u8>>,
}

/// Trait for providing components to entities during creation.
///
/// Mods implement this trait to add their components to players
/// and other entities when they spawn.
///
/// # Example
///
/// ```ignore
/// struct SurvivalComponentProvider;
///
/// impl ComponentProvider for SurvivalComponentProvider {
///     fn provide_player_components(
///         &self,
///         commands: &mut EntityCommands,
///         ctx: &PlayerSetupContext,
///     ) {
///         // Add survival components
///         commands.insert((
///             Health::new(20.0),
///             Hunger::new(20, 5.0),
///             Experience::default(),
///         ));
///
///         // Restore from cache if available
///         if let Some(cached) = &ctx.cached_data {
///             if let Some(health) = cached.mod_data.get("health") {
///                 // Deserialize and apply cached health
///             }
///         }
///     }
/// }
/// ```
pub trait ComponentProvider: Send + Sync + 'static {
    /// Add components to a player entity.
    ///
    /// Called when a player joins the server, after base components
    /// are added but before the player is fully initialized.
    fn provide_player_components(
        &self,
        _commands: &mut EntityCommands,
        _ctx: &PlayerSetupContext,
    ) {
    }

    /// Add components to a non-player entity.
    ///
    /// Called when an entity is spawned, after base components
    /// are added.
    fn provide_entity_components(
        &self,
        _commands: &mut EntityCommands,
        _ctx: &EntitySetupContext,
    ) {
    }

    /// Called when saving player data to cache.
    ///
    /// Return data to be stored and restored on next join.
    fn save_player_data(
        &self,
        _entity: Entity,
        _world: &World,
    ) -> Option<Vec<u8>> {
        None
    }

    /// Unique identifier for this provider (for data storage).
    fn provider_id(&self) -> &'static str;
}
