//! Behavior traits for modular game logic.
//!
//! Behaviors allow mods to extend block, entity, and item functionality
//! without subclassing. Multiple behaviors can be composed on a single
//! object, and they are called in registration order.

use bevy_ecs::prelude::*;

/// Result of a behavior handler determining control flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Handling {
    /// Continue to the next behavior and default logic.
    #[default]
    Pass,
    /// Stop the behavior chain, but allow default logic to run.
    Handled,
    /// Stop the behavior chain and prevent default logic.
    Prevent,
}

impl Handling {
    /// Returns true if subsequent handlers should be skipped.
    pub fn should_stop(&self) -> bool {
        matches!(self, Handling::Handled | Handling::Prevent)
    }

    /// Returns true if the default action should be prevented.
    pub fn should_prevent(&self) -> bool {
        matches!(self, Handling::Prevent)
    }
}

/// Context provided to block behavior methods.
#[derive(Debug)]
pub struct BlockContext {
    /// Block position in world coordinates
    pub position: BlockPos,
    /// Block ID (e.g., "minecraft:stone")
    pub block_id: String,
    /// Entity that triggered the action (if any)
    pub actor: Option<Entity>,
    /// Whether this is on the server (true) or client (false)
    pub is_server: bool,
    /// Whether the actor is in creative mode
    pub actor_creative: bool,
}

impl BlockContext {
    /// Check if the player/actor is in creative mode.
    pub fn is_player_creative(&self) -> bool {
        self.actor_creative
    }
}

/// Block position in world coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

/// An item stack (item type + count).
#[derive(Debug, Clone)]
pub struct ItemStack {
    /// Item ID (e.g., "minecraft:diamond")
    pub item_id: String,
    /// Stack count
    pub count: u32,
    /// NBT data (optional)
    pub nbt: Option<Vec<u8>>,
}

impl ItemStack {
    pub fn new(item_id: impl Into<String>, count: u32) -> Self {
        Self {
            item_id: item_id.into(),
            count,
            nbt: None,
        }
    }
}

/// Modular behavior for blocks.
///
/// Implement this trait to add custom logic to blocks.
/// Multiple behaviors can be registered for the same block type
/// and will be called in registration order.
///
/// # Example
///
/// ```ignore
/// struct ExplosiveBlockBehavior;
///
/// impl BlockBehavior for ExplosiveBlockBehavior {
///     fn on_broken(&self, ctx: &mut BlockContext) -> Handling {
///         // Create explosion at block position
///         spawn_explosion(ctx.position);
///         Handling::Handled
///     }
/// }
/// ```
pub trait BlockBehavior: Send + Sync + 'static {
    /// Called when the block is placed in the world.
    fn on_placed(&self, _ctx: &mut BlockContext) -> Handling {
        Handling::Pass
    }

    /// Called when the block is broken/destroyed.
    fn on_broken(&self, _ctx: &mut BlockContext) -> Handling {
        Handling::Pass
    }

    /// Called when a player interacts with the block.
    fn on_interact(&self, _ctx: &mut BlockContext) -> Handling {
        Handling::Pass
    }

    /// Called when a neighboring block changes.
    fn on_neighbor_changed(&self, _ctx: &mut BlockContext, _neighbor_pos: BlockPos) -> Handling {
        Handling::Pass
    }

    /// Get the drops when this block is broken.
    /// Return None to use default drops.
    fn get_drops(&self, _ctx: &BlockContext) -> Option<Vec<ItemStack>> {
        None
    }

    /// Get the hardness of this block (affects break time).
    /// Return None to use default hardness.
    fn get_hardness(&self, _ctx: &BlockContext) -> Option<f32> {
        None
    }

    /// Check if the block can be placed at this position.
    fn can_place_at(&self, _ctx: &BlockContext) -> bool {
        true
    }
}

/// Context provided to entity behavior methods.
#[derive(Debug)]
pub struct EntityContext {
    /// The entity this behavior is attached to
    pub entity: Entity,
    /// Delta time since last tick (seconds)
    pub delta_time: f32,
}

impl EntityContext {
    pub fn new(entity: Entity, delta_time: f32) -> Self {
        Self { entity, delta_time }
    }
}

/// Context for damage events.
#[derive(Debug)]
pub struct DamageContext {
    /// Entity being damaged
    pub target: Entity,
    /// Entity dealing damage (if any)
    pub attacker: Option<Entity>,
    /// Damage amount (can be modified)
    pub damage: f32,
    /// Damage type identifier
    pub damage_type: String,
    /// Whether the damage is cancelled
    pub cancelled: bool,
}

/// Context for death events.
#[derive(Debug)]
pub struct DeathContext {
    /// Entity that died
    pub entity: Entity,
    /// Cause of death
    pub cause: String,
    /// Killer entity (if any)
    pub killer: Option<Entity>,
}

/// Context for entity interaction events.
#[derive(Debug)]
pub struct InteractContext {
    /// Entity being interacted with
    pub target: Entity,
    /// Entity doing the interaction (usually player)
    pub actor: Entity,
    /// Whether this is the main hand interaction
    pub main_hand: bool,
}

/// Modular behavior for entities.
///
/// Implement this trait to add custom logic to entities (players, mobs, etc.).
/// Behaviors are ticked every game tick and can respond to various events.
///
/// # Example
///
/// ```ignore
/// struct HungerBehavior;
///
/// impl EntityBehavior for HungerBehavior {
///     fn on_tick(&self, ctx: &mut EntityContext, world: &mut World) {
///         // Decrease hunger over time
///         if let Some(mut hunger) = world.get_mut::<Hunger>(ctx.entity) {
///             hunger.exhaustion += 0.001 * ctx.delta_time;
///         }
///     }
/// }
/// ```
pub trait EntityBehavior: Send + Sync + 'static {
    /// Called every game tick.
    fn on_tick(&self, _ctx: &mut EntityContext, _world: &mut World) {}

    /// Called when the entity spawns.
    fn on_spawn(&self, _ctx: &mut EntityContext, _world: &mut World) {}

    /// Called when the entity is about to despawn.
    fn on_despawn(&self, _ctx: &mut EntityContext, _world: &mut World) {}

    /// Called when the entity receives damage.
    fn on_damage(&self, _ctx: &mut DamageContext, _world: &mut World) -> Handling {
        Handling::Pass
    }

    /// Called when the entity dies.
    fn on_death(&self, _ctx: &mut DeathContext, _world: &mut World) {}

    /// Called when another entity interacts with this entity.
    fn on_interact(&self, _ctx: &mut InteractContext, _world: &mut World) -> Handling {
        Handling::Pass
    }

    /// Whether this behavior is safe to run off the main thread.
    /// Default is false (runs on main thread only).
    fn is_thread_safe(&self) -> bool {
        false
    }
}

/// Context for item use events.
#[derive(Debug)]
pub struct UseContext {
    /// Entity using the item
    pub user: Entity,
    /// Item being used
    pub item: ItemStack,
    /// Whether this is the main hand
    pub main_hand: bool,
}

/// Context for mining speed calculation.
#[derive(Debug)]
pub struct MiningContext {
    /// Entity doing the mining
    pub miner: Entity,
    /// Block being mined
    pub block_id: String,
    /// Block position
    pub position: BlockPos,
    /// Tool being used
    pub tool: Option<ItemStack>,
}

/// Modular behavior for items/collectibles.
///
/// Implement this trait to add custom logic to items.
///
/// # Example
///
/// ```ignore
/// struct FoodBehavior {
///     nutrition: f32,
///     saturation: f32,
/// }
///
/// impl CollectibleBehavior for FoodBehavior {
///     fn on_use(&self, ctx: &mut UseContext, world: &mut World) -> Handling {
///         if let Some(mut hunger) = world.get_mut::<Hunger>(ctx.user) {
///             hunger.food_level += self.nutrition;
///             hunger.saturation += self.saturation;
///         }
///         Handling::Handled
///     }
/// }
/// ```
pub trait CollectibleBehavior: Send + Sync + 'static {
    /// Called when the item is used (right-click).
    fn on_use(&self, _ctx: &mut UseContext, _world: &mut World) -> Handling {
        Handling::Pass
    }

    /// Get the mining speed multiplier for this tool against a block.
    /// Return None to use default speed.
    fn get_mining_speed(&self, _ctx: &MiningContext) -> Option<f32> {
        None
    }

    /// Check if this tool can harvest the given block.
    fn can_harvest(&self, _ctx: &MiningContext) -> bool {
        true
    }

    /// Called when the item is used to attack an entity.
    fn on_attack(
        &self,
        _ctx: &mut UseContext,
        _target: Entity,
        _world: &mut World,
    ) -> Handling {
        Handling::Pass
    }

    /// Get the attack damage for this item.
    /// Return None to use default damage.
    fn get_attack_damage(&self) -> Option<f32> {
        None
    }

    /// Get the attack speed for this item.
    /// Return None to use default speed.
    fn get_attack_speed(&self) -> Option<f32> {
        None
    }
}
