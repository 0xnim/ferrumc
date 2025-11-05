//! Common event types for plugins

use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_world::block_state_id::BlockStateId;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

/// Event emitted when a player attempts to place a block.
///
/// This event can be cancelled by plugins (e.g., for protection/permissions).
///
/// # Example
///
/// ```rust,no_run
/// fn validate_placement(mut events: EventReader<BlockPlacedEvent>) {
///     for event in events.read() {
///         if is_protected_area(event.x, event.y, event.z) {
///             event.cancel("This area is protected");
///         }
///     }
/// }
/// ```
#[derive(Event, Clone)]
pub struct BlockPlacedEvent {
    /// The player who placed the block
    pub player: Entity,
    /// Block position
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// The block that was placed
    pub block: BlockStateId,
    /// Whether the placement has been cancelled (thread-safe)
    cancelled: Arc<AtomicBool>,
    /// Optional cancellation reason (thread-safe)
    cancel_reason: Arc<RwLock<Option<String>>>,
}

impl BlockPlacedEvent {
    /// Create a new block placed event
    pub fn new(player: Entity, x: i32, y: i32, z: i32, block: BlockStateId) -> Self {
        Self {
            player,
            x,
            y,
            z,
            block,
            cancelled: Arc::new(AtomicBool::new(false)),
            cancel_reason: Arc::new(RwLock::new(None)),
        }
    }

    /// Cancel the block placement
    pub fn cancel(&self, reason: impl Into<String>) {
        self.cancelled.store(true, Ordering::Relaxed);
        if let Ok(mut cancel_reason) = self.cancel_reason.write() {
            *cancel_reason = Some(reason.into());
        }
    }

    /// Check if the placement has been cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    /// Get the cancellation reason (non-consuming)
    pub fn cancel_reason(&self) -> Option<String> {
        self.cancel_reason.read().ok().and_then(|r| r.clone())
    }
}

impl std::fmt::Debug for BlockPlacedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockPlacedEvent")
            .field("player", &self.player)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("block", &self.block)
            .field("cancelled", &self.cancelled.load(Ordering::Relaxed))
            .finish()
    }
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

/// Event emitted when a player takes damage.
///
/// This event supports modification by multiple plugins. Use `modify_amount()`
/// to apply damage modifiers (enchantments, effects, etc.).
///
/// # Example
///
/// ```rust,no_run
/// // Plugin A: Calculate base fall damage
/// fn calculate_fall_damage(world: &mut World) {
///     world.send_event(PlayerDamageEvent::new(
///         player,
///         10.0,
///         None,
///         DamageType::Fall,
///     ));
/// }
///
/// // Plugin B: Apply feather falling enchantment
/// fn apply_feather_falling(mut events: EventReader<PlayerDamageEvent>) {
///     for event in events.read() {
///         if event.damage_type == DamageType::Fall {
///             event.modify_amount(|dmg| dmg * 0.5);  // 50% reduction
///         }
///     }
/// }
///
/// // Plugin C: Apply final damage to health
/// fn apply_damage(mut events: EventReader<PlayerDamageEvent>) {
///     for event in events.read() {
///         if !event.is_cancelled() {
///             let final_damage = event.get_amount();
///             // Apply to health...
///         }
///     }
/// }
/// ```
#[derive(Event, Clone)]
pub struct PlayerDamageEvent {
    /// The player who took damage
    pub player: Entity,
    /// Amount of damage (modifiable by plugins, thread-safe)
    amount: Arc<RwLock<f32>>,
    /// Source of damage (optional entity that caused it)
    pub source: Option<Entity>,
    /// Type of damage
    pub damage_type: DamageType,
    /// Whether the damage has been cancelled (thread-safe)
    cancelled: Arc<AtomicBool>,
}

impl PlayerDamageEvent {
    /// Create a new damage event
    pub fn new(player: Entity, amount: f32, source: Option<Entity>, damage_type: DamageType) -> Self {
        Self {
            player,
            amount: Arc::new(RwLock::new(amount)),
            source,
            damage_type,
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Get the current damage amount
    pub fn get_amount(&self) -> f32 {
        self.amount.read().map(|a| *a).unwrap_or(0.0)
    }

    /// Set the damage amount directly
    pub fn set_amount(&self, amount: f32) {
        if let Ok(mut a) = self.amount.write() {
            *a = amount;
        }
    }

    /// Modify the damage amount with a function
    ///
    /// # Example
    /// ```rust,no_run
    /// event.modify_amount(|damage| damage * 0.8);  // 20% reduction
    /// event.modify_amount(|damage| damage + 5.0);  // Add 5 damage
    /// ```
    pub fn modify_amount<F>(&self, f: F)
    where
        F: FnOnce(f32) -> f32,
    {
        if let Ok(mut amount) = self.amount.write() {
            *amount = f(*amount);
        }
    }

    /// Cancel the damage event entirely
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// Check if the damage has been cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

impl std::fmt::Debug for PlayerDamageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerDamageEvent")
            .field("player", &self.player)
            .field("amount", &self.get_amount())
            .field("source", &self.source)
            .field("damage_type", &self.damage_type)
            .field("cancelled", &self.cancelled.load(Ordering::Relaxed))
            .finish()
    }
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
