//! Common event types for plugins

use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_world::block_state_id::BlockStateId;

/// Event emitted when a player places a block
#[derive(Event, Debug, Clone)]
pub struct BlockPlacedEvent {
    /// The player who placed the block
    pub player: Entity,
    /// Block position
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// The block that was placed
    pub block: BlockStateId,
}

/// Event emitted when a player breaks a block
#[derive(Event, Debug, Clone)]
pub struct BlockBrokenEvent {
    /// The player who broke the block
    pub player: Entity,
    /// Block position
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// The block that was broken
    pub previous_block: BlockStateId,
}

/// Event emitted when a player sends a chat message
#[derive(Event, Debug, Clone)]
pub struct ChatMessageEvent {
    /// The player who sent the message
    pub sender: Entity,
    /// The message content
    pub message: String,
}

/// Event emitted when a player joins the server
#[derive(Event, Debug, Clone)]
pub struct PlayerJoinEvent {
    /// The player entity
    pub player: Entity,
    /// Player's username
    pub username: String,
}

/// Event emitted when a player leaves the server
#[derive(Event, Debug, Clone)]
pub struct PlayerLeaveEvent {
    /// The player entity
    pub player: Entity,
    /// Player's username
    pub username: String,
    /// Optional disconnect reason
    pub reason: Option<String>,
}

/// Event emitted when a player moves
#[derive(Event, Debug, Clone)]
pub struct PlayerMoveEvent {
    /// The player who moved
    pub player: Entity,
    /// Previous position
    pub from: Position,
    /// New position
    pub to: Position,
}

/// Event emitted when a player changes their held item
#[derive(Event, Debug, Clone)]
pub struct PlayerChangeHeldItemEvent {
    /// The player
    pub player: Entity,
    /// Previous slot index
    pub from_slot: u8,
    /// New slot index
    pub to_slot: u8,
}

/// Event emitted when a player takes damage
#[derive(Event, Debug, Clone)]
pub struct PlayerDamageEvent {
    /// The player who took damage
    pub player: Entity,
    /// Amount of damage
    pub amount: f32,
    /// Source of damage (optional entity that caused it)
    pub source: Option<Entity>,
    /// Type of damage
    pub damage_type: DamageType,
}

/// Types of damage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    /// Fall damage
    Fall,
    /// Void damage (y < minimum)
    Void,
    /// Lava damage
    Lava,
    /// Fire damage
    Fire,
    /// Drowning
    Drowning,
    /// Entity attack
    EntityAttack,
    /// Projectile hit
    Projectile,
    /// Other/unknown
    Other,
}

/// Event emitted when a player dies
#[derive(Event, Debug, Clone)]
pub struct PlayerDeathEvent {
    /// The player who died
    pub player: Entity,
    /// Killer entity (if applicable)
    pub killer: Option<Entity>,
    /// Death message
    pub death_message: String,
}

/// Event emitted when a player respawns
#[derive(Event, Debug, Clone)]
pub struct PlayerRespawnEvent {
    /// The player who respawned
    pub player: Entity,
    /// Respawn position
    pub position: Position,
}

/// Event emitted when a chunk is loaded
#[derive(Event, Debug, Clone)]
pub struct ChunkLoadEvent {
    /// Chunk X coordinate
    pub chunk_x: i32,
    /// Chunk Z coordinate
    pub chunk_z: i32,
    /// Dimension name
    pub dimension: String,
}

/// Event emitted when a chunk is unloaded
#[derive(Event, Debug, Clone)]
pub struct ChunkUnloadEvent {
    /// Chunk X coordinate
    pub chunk_x: i32,
    /// Chunk Z coordinate
    pub chunk_z: i32,
    /// Dimension name
    pub dimension: String,
}

/// Event emitted when a chunk is generated
#[derive(Event, Debug, Clone)]
pub struct ChunkGeneratedEvent {
    /// Chunk X coordinate
    pub chunk_x: i32,
    /// Chunk Z coordinate
    pub chunk_z: i32,
}
