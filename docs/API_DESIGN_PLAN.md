# FerrumC API Design Plan

Based on analysis of Vintage Story's VSApi architecture, this document outlines a comprehensive API design for FerrumC that enables modular game logic while maintaining performance.

## Design Philosophy

1. **Engine vs Game Logic Separation**: The engine provides protocol, ECS, world, physics, networking. All gameplay (survival, creative) is implemented as mods.
2. **Behavior Composition**: Functionality is added through composable behaviors, not inheritance.
3. **Event-Driven**: Rich event system allows mods to react to and modify game events.
4. **Static Registration**: Mods are Rust crates compiled into the server (no runtime loading).

---

## 1. Core API Structure

### 1.1 API Hierarchy

```
┌─────────────────────────────────────────────────────────┐
│                      CoreApi                            │
│  (Common functionality for all contexts)                │
│  - Block/Entity/Item behavior registration              │
│  - Command registration                                 │
│  - Asset access                                         │
│  - Logging                                              │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                     ServerApi                           │
│  (Server-specific functionality)                        │
│  - World access (read/write blocks, spawn entities)     │
│  - Event subscription                                   │
│  - Tick system registration                             │
│  - Player management                                    │
│  - Component providers                                  │
└─────────────────────────────────────────────────────────┘
```

### 1.2 Trait Definitions

```rust
/// Common API available to all mods.
pub trait CoreApi: Send + Sync {
    // === Behavior Registration ===
    fn register_block_behavior(&mut self, block_id: &str, behavior: Arc<dyn BlockBehavior>);
    fn register_entity_behavior(&mut self, entity_type: &str, behavior: Arc<dyn EntityBehavior>);
    fn register_item_behavior(&mut self, item_id: &str, behavior: Arc<dyn CollectibleBehavior>);

    // === Command Registration ===
    fn register_command(&mut self, command: Command);

    // === Class Registration ===
    fn register_block_class<T: Block>(&mut self, name: &str);
    fn register_item_class<T: Item>(&mut self, name: &str);
    fn register_entity_class<T: EntityDefinition>(&mut self, name: &str);

    // === Asset Access ===
    fn assets(&self) -> &dyn AssetManager;

    // === Logging ===
    fn logger(&self) -> &dyn Logger;
}

/// Server-specific API.
pub trait ServerApi: CoreApi {
    // === World Access ===
    fn world(&self) -> &dyn WorldAccessor;

    // === Event System ===
    fn events(&self) -> &dyn EventApi;

    // === Tick Systems ===
    fn register_tick_system(&mut self, system: TickSystem);

    // === Component Providers ===
    fn register_player_component_provider(&mut self, provider: Arc<dyn ComponentProvider>);
    fn register_entity_component_provider(&mut self, entity_type: &str, provider: Arc<dyn ComponentProvider>);

    // === Player Management ===
    fn players(&self) -> &dyn PlayerManager;

    // === Server Info ===
    fn server(&self) -> &dyn ServerInfo;
}
```

---

## 2. ModSystem Lifecycle

### 2.1 Lifecycle Methods

```rust
pub trait ModSystem: Send + Sync + 'static {
    /// Unique mod identifier (e.g., "ferrumc:survival")
    fn mod_id(&self) -> &'static str;

    /// Mod version
    fn version(&self) -> &'static str { "1.0.0" }

    /// Dependencies (mod IDs this mod requires)
    fn dependencies(&self) -> &'static [&'static str] { &[] }

    /// Optional dependencies (loaded before this mod if present)
    fn optional_dependencies(&self) -> &'static [&'static str] { &[] }

    // === Lifecycle Methods (in execution order) ===

    /// 1. Called first. Register behaviors, commands, class types.
    fn start(&self, api: &mut dyn CoreApi) {}

    /// 2. Called after all JSON assets are loaded and parsed.
    /// Use to access loaded block/item definitions.
    fn assets_loaded(&self, api: &mut dyn ServerApi) {}

    /// 3. Called after assets are registered but before server starts.
    /// Last chance to modify properties, add behaviors to loaded content.
    fn assets_finalize(&self, api: &mut dyn ServerApi) {}

    /// 4. Called when server is ready to accept connections.
    /// Register event listeners, tick systems, spawn initial entities.
    fn start_server_side(&self, api: &mut dyn ServerApi) {}

    /// 5. Called when server is shutting down.
    fn dispose(&self) {}
}
```

### 2.2 Lifecycle Execution Order

```
Server Startup:
  1. Load all mod crates (static registration via ctor)
  2. Sort mods by dependencies
  3. For each mod: call start()
  4. Load JSON assets (blocks, items, entities, recipes)
  5. For each mod: call assets_loaded()
  6. Register all loaded content
  7. For each mod: call assets_finalize()
  8. Initialize world, load chunks
  9. For each mod: call start_server_side()
  10. Accept player connections

Server Shutdown:
  1. Disconnect all players
  2. For each mod (reverse order): call dispose()
  3. Save world
  4. Exit
```

---

## 3. Behavior System

### 3.1 Handling Enum

```rust
/// Controls behavior chain execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Handling {
    /// Continue to next behavior and default logic.
    #[default]
    Pass,
    /// Stop behavior chain, but run default logic.
    Handled,
    /// Stop behavior chain AND prevent default logic.
    PreventDefault,
    /// Stop this specific behavior chain only.
    PreventSubsequent,
}
```

### 3.2 BlockBehavior

```rust
pub trait BlockBehavior: Send + Sync + 'static {
    /// Called when block is placed.
    fn on_placed(&self, ctx: &mut BlockPlaceContext) -> Handling { Handling::Pass }

    /// Called when block is broken by player.
    fn on_broken(&self, ctx: &mut BlockBreakContext) -> Handling { Handling::Pass }

    /// Called when block is removed (any cause).
    fn on_removed(&self, ctx: &mut BlockRemoveContext) -> Handling { Handling::Pass }

    /// Called when player interacts (right-click).
    fn on_interact_start(&self, ctx: &mut BlockInteractContext) -> Handling { Handling::Pass }
    fn on_interact_step(&self, ctx: &mut BlockInteractContext, dt: f32) -> Handling { Handling::Pass }
    fn on_interact_stop(&self, ctx: &mut BlockInteractContext) -> Handling { Handling::Pass }
    fn on_interact_cancel(&self, ctx: &mut BlockInteractContext) -> Handling { Handling::Pass }

    /// Called when neighboring block changes.
    fn on_neighbor_changed(&self, ctx: &mut BlockNeighborContext) -> Handling { Handling::Pass }

    /// Called every tick while block exists.
    fn on_tick(&self, ctx: &mut BlockTickContext) -> Handling { Handling::Pass }

    /// Called when entity is inside block bounds.
    fn on_entity_inside(&self, ctx: &mut BlockEntityContext) -> Handling { Handling::Pass }

    /// Called when entity collides with block.
    fn on_entity_collide(&self, ctx: &mut BlockEntityContext) -> Handling { Handling::Pass }

    // === Property Overrides ===

    /// Override block hardness (affects break time).
    fn get_hardness(&self, ctx: &BlockContext) -> Option<f32> { None }

    /// Override block drops.
    fn get_drops(&self, ctx: &BlockContext) -> Option<Vec<ItemStack>> { None }

    /// Override if block can be placed at position.
    fn can_place_at(&self, ctx: &BlockPlaceContext) -> Option<bool> { None }

    /// Override light emission.
    fn get_light_level(&self, ctx: &BlockContext) -> Option<u8> { None }
}
```

### 3.3 EntityBehavior

```rust
pub trait EntityBehavior: Send + Sync + 'static {
    /// Called every game tick.
    fn on_tick(&self, ctx: &mut EntityTickContext) -> Handling { Handling::Pass }

    /// Called when entity first spawns.
    fn on_spawn(&self, ctx: &mut EntitySpawnContext) -> Handling { Handling::Pass }

    /// Called when entity is loaded from save.
    fn on_loaded(&self, ctx: &mut EntityLoadContext) -> Handling { Handling::Pass }

    /// Called when entity despawns.
    fn on_despawn(&self, ctx: &mut EntityDespawnContext) -> Handling { Handling::Pass }

    /// Called when entity receives damage.
    fn on_damage(&self, ctx: &mut DamageContext) -> Handling { Handling::Pass }

    /// Called when entity dies.
    fn on_death(&self, ctx: &mut DeathContext) -> Handling { Handling::Pass }

    /// Called when another entity interacts with this one.
    fn on_interact(&self, ctx: &mut EntityInteractContext) -> Handling { Handling::Pass }

    /// Called when entity falls and hits ground.
    fn on_fall(&self, ctx: &mut EntityFallContext) -> Handling { Handling::Pass }

    /// Called when entity enters water.
    fn on_enter_water(&self, ctx: &mut EntityWaterContext) -> Handling { Handling::Pass }

    /// Called when entity mounts another entity.
    fn on_mount(&self, ctx: &mut EntityMountContext) -> Handling { Handling::Pass }

    /// Called when entity dismounts.
    fn on_dismount(&self, ctx: &mut EntityMountContext) -> Handling { Handling::Pass }

    // === Properties ===

    /// Whether this behavior can run off main thread.
    fn is_thread_safe(&self) -> bool { false }
}
```

### 3.4 CollectibleBehavior (Items)

```rust
pub trait CollectibleBehavior: Send + Sync + 'static {
    // === Held Interaction (right-click while holding) ===
    fn on_held_interact_start(&self, ctx: &mut HeldInteractContext) -> Handling { Handling::Pass }
    fn on_held_interact_step(&self, ctx: &mut HeldInteractContext, dt: f32) -> Handling { Handling::Pass }
    fn on_held_interact_stop(&self, ctx: &mut HeldInteractContext) -> Handling { Handling::Pass }
    fn on_held_interact_cancel(&self, ctx: &mut HeldInteractContext) -> Handling { Handling::Pass }

    // === Attack (left-click on entity) ===
    fn on_attack_start(&self, ctx: &mut AttackContext) -> Handling { Handling::Pass }
    fn on_attack_step(&self, ctx: &mut AttackContext, dt: f32) -> Handling { Handling::Pass }
    fn on_attack_stop(&self, ctx: &mut AttackContext) -> Handling { Handling::Pass }

    // === Block Breaking ===
    fn on_block_breaking(&self, ctx: &mut MiningContext, dt: f32) -> Handling { Handling::Pass }
    fn on_block_broken(&self, ctx: &mut MiningContext) -> Handling { Handling::Pass }

    // === Passive ===
    fn on_held_idle(&self, ctx: &mut HeldContext) -> Handling { Handling::Pass }
    fn on_ground_idle(&self, ctx: &mut GroundItemContext) -> Handling { Handling::Pass }
    fn on_collected(&self, ctx: &mut CollectContext) -> Handling { Handling::Pass }

    // === Property Overrides ===
    fn get_mining_speed(&self, ctx: &MiningContext) -> Option<f32> { None }
    fn get_attack_damage(&self, ctx: &AttackContext) -> Option<f32> { None }
    fn get_attack_speed(&self, ctx: &AttackContext) -> Option<f32> { None }
    fn can_harvest(&self, ctx: &MiningContext) -> Option<bool> { None }
}
```

---

## 4. Event System

### 4.1 Event API

```rust
pub trait EventApi {
    // === Player Events ===
    fn on_player_join(&self, priority: EventPriority, handler: PlayerJoinHandler);
    fn on_player_leave(&self, priority: EventPriority, handler: PlayerLeaveHandler);
    fn on_player_death(&self, priority: EventPriority, handler: PlayerDeathHandler);
    fn on_player_respawn(&self, priority: EventPriority, handler: PlayerRespawnHandler);
    fn on_player_chat(&self, priority: EventPriority, handler: PlayerChatHandler);
    fn on_player_command(&self, priority: EventPriority, handler: PlayerCommandHandler);
    fn on_player_gamemode_change(&self, priority: EventPriority, handler: GamemodeChangeHandler);

    // === Block Events ===
    fn on_block_place(&self, priority: EventPriority, handler: BlockPlaceHandler);
    fn on_block_break(&self, priority: EventPriority, handler: BlockBreakHandler);
    fn on_block_interact(&self, priority: EventPriority, handler: BlockInteractHandler);

    // === Entity Events ===
    fn on_entity_spawn(&self, priority: EventPriority, handler: EntitySpawnHandler);
    fn on_entity_despawn(&self, priority: EventPriority, handler: EntityDespawnHandler);
    fn on_entity_damage(&self, priority: EventPriority, handler: EntityDamageHandler);
    fn on_entity_death(&self, priority: EventPriority, handler: EntityDeathHandler);
    fn on_entity_interact(&self, priority: EventPriority, handler: EntityInteractHandler);

    // === World Events ===
    fn on_chunk_load(&self, priority: EventPriority, handler: ChunkLoadHandler);
    fn on_chunk_unload(&self, priority: EventPriority, handler: ChunkUnloadHandler);
    fn on_world_save(&self, priority: EventPriority, handler: WorldSaveHandler);

    // === Server Events ===
    fn on_tick(&self, priority: EventPriority, handler: TickHandler);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Highest = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Lowest = 4,
    Monitor = 5, // Read-only, cannot cancel
}
```

### 4.2 Event Contexts

```rust
/// Base for cancellable events.
pub struct CancellableEvent {
    pub cancelled: bool,
}

pub struct PlayerJoinEvent {
    pub player: Entity,
    pub username: String,
    pub uuid: Uuid,
    pub is_new_player: bool,
}

pub struct PlayerDeathEvent {
    pub player: Entity,
    pub damage_source: DamageSource,
    pub killer: Option<Entity>,
    pub drop_items: bool, // Can be modified
    pub keep_inventory: bool, // Can be modified
    pub cancellable: CancellableEvent,
}

pub struct BlockBreakEvent {
    pub player: Entity,
    pub position: BlockPos,
    pub block: BlockState,
    pub drops: Vec<ItemStack>, // Can be modified
    pub experience: u32, // Can be modified
    pub cancellable: CancellableEvent,
}

pub struct EntityDamageEvent {
    pub entity: Entity,
    pub source: DamageSource,
    pub damage: f32, // Can be modified
    pub cancellable: CancellableEvent,
}
```

---

## 5. World Access

### 5.1 WorldAccessor Trait

```rust
pub trait WorldAccessor: Send + Sync {
    // === Block Access ===
    fn get_block(&self, pos: BlockPos) -> Option<BlockState>;
    fn set_block(&self, pos: BlockPos, block: BlockState) -> Result<(), WorldError>;
    fn get_block_entity(&self, pos: BlockPos) -> Option<Entity>;

    // === Chunk Access ===
    fn is_chunk_loaded(&self, chunk_x: i32, chunk_z: i32) -> bool;
    fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<&Chunk>;

    // === Entity Spawning ===
    fn spawn_entity(&self, entity_type: &str, pos: Position) -> Result<Entity, WorldError>;
    fn spawn_entity_with(&self, entity_type: &str, pos: Position, setup: impl FnOnce(&mut EntityCommands)) -> Result<Entity, WorldError>;
    fn despawn_entity(&self, entity: Entity) -> Result<(), WorldError>;

    // === Entity Queries ===
    fn get_entities_in_box(&self, min: Position, max: Position) -> Vec<Entity>;
    fn get_entities_in_radius(&self, center: Position, radius: f64) -> Vec<Entity>;
    fn get_nearest_player(&self, pos: Position, max_distance: f64) -> Option<Entity>;

    // === Raycasting ===
    fn raycast_block(&self, origin: Position, direction: Vec3, max_distance: f64) -> Option<BlockHit>;
    fn raycast_entity(&self, origin: Position, direction: Vec3, max_distance: f64) -> Option<EntityHit>;

    // === World Properties ===
    fn get_time(&self) -> i64;
    fn set_time(&self, time: i64);
    fn get_weather(&self) -> Weather;
    fn set_weather(&self, weather: Weather);
    fn get_spawn_point(&self) -> Position;
}
```

---

## 6. Command System

### 6.1 Command Builder API

```rust
pub struct Command {
    name: String,
    aliases: Vec<String>,
    description: String,
    usage: String,
    examples: Vec<String>,
    permission: Option<String>,
    requires_player: bool,
    arguments: Vec<Box<dyn ArgumentParser>>,
    subcommands: Vec<Command>,
    handler: CommandHandler,
}

impl Command {
    pub fn new(name: &str) -> CommandBuilder;
}

pub struct CommandBuilder {
    // ... builder pattern
}

impl CommandBuilder {
    pub fn description(self, desc: &str) -> Self;
    pub fn usage(self, usage: &str) -> Self;
    pub fn aliases(self, aliases: &[&str]) -> Self;
    pub fn example(self, example: &str) -> Self;
    pub fn permission(self, perm: &str) -> Self;
    pub fn requires_player(self) -> Self;
    pub fn arg<T: ArgumentParser>(self, arg: T) -> Self;
    pub fn subcommand(self, cmd: Command) -> Self;
    pub fn handler(self, handler: impl Fn(&CommandContext) -> CommandResult) -> Command;
}
```

### 6.2 Argument Parsers

```rust
pub trait ArgumentParser: Send + Sync {
    fn name(&self) -> &str;
    fn parse(&self, input: &str) -> Result<ParsedArg, ParseError>;
    fn suggestions(&self, partial: &str, ctx: &CommandContext) -> Vec<String>;
    fn is_optional(&self) -> bool;
}

// Built-in parsers
pub struct IntArg { name: String, min: Option<i32>, max: Option<i32> }
pub struct FloatArg { name: String, min: Option<f64>, max: Option<f64> }
pub struct StringArg { name: String, quoted: bool }
pub struct WordArg { name: String }
pub struct BoolArg { name: String }
pub struct PlayerArg { name: String } // Supports @a, @p, @r, @e selectors
pub struct EntityArg { name: String }
pub struct BlockPosArg { name: String } // Supports ~ relative coords
pub struct ItemArg { name: String }
pub struct BlockArg { name: String }
pub struct EnumArg<T: EnumArgValue> { name: String }
pub struct GreedyStringArg { name: String } // Consumes rest of input
```

### 6.3 Command Context

```rust
pub struct CommandContext {
    pub caller: CommandCaller,
    pub args: ParsedArgs,
    pub api: Arc<dyn ServerApi>,
}

pub enum CommandCaller {
    Player(Entity),
    Console,
    CommandBlock(BlockPos),
}

impl CommandContext {
    pub fn get_int(&self, name: &str) -> Result<i32, ArgError>;
    pub fn get_float(&self, name: &str) -> Result<f64, ArgError>;
    pub fn get_string(&self, name: &str) -> Result<&str, ArgError>;
    pub fn get_player(&self, name: &str) -> Result<Entity, ArgError>;
    pub fn get_players(&self, name: &str) -> Result<Vec<Entity>, ArgError>;
    pub fn get_block_pos(&self, name: &str) -> Result<BlockPos, ArgError>;

    pub fn reply(&self, message: impl Into<String>);
    pub fn reply_error(&self, message: impl Into<String>);
}
```

---

## 7. Inventory System

### 7.1 Inventory Traits

```rust
pub trait Inventory: Send + Sync {
    fn id(&self) -> &str;
    fn size(&self) -> usize;
    fn get_slot(&self, index: usize) -> Option<&ItemSlot>;
    fn get_slot_mut(&mut self, index: usize) -> Option<&mut ItemSlot>;
    fn set_slot(&mut self, index: usize, stack: Option<ItemStack>) -> Result<(), InventoryError>;

    /// Find best slot for item (for shift-click, etc.)
    fn get_suitable_slot(&self, stack: &ItemStack) -> Option<usize>;

    /// Check if item can go in slot.
    fn can_insert(&self, index: usize, stack: &ItemStack) -> bool;
}

pub struct ItemSlot {
    pub stack: Option<ItemStack>,
    pub locked: bool,
}

pub struct ItemStack {
    pub item_id: String,
    pub count: u32,
    pub damage: u32,
    pub nbt: Option<NbtCompound>,
}
```

### 7.2 Player Inventory Access

```rust
pub trait PlayerManager {
    fn get_player(&self, entity: Entity) -> Option<&dyn PlayerAccessor>;
    fn get_player_by_name(&self, name: &str) -> Option<&dyn PlayerAccessor>;
    fn get_player_by_uuid(&self, uuid: Uuid) -> Option<&dyn PlayerAccessor>;
    fn get_online_players(&self) -> Vec<Entity>;
}

pub trait PlayerAccessor {
    fn entity(&self) -> Entity;
    fn username(&self) -> &str;
    fn uuid(&self) -> Uuid;
    fn gamemode(&self) -> GameMode;
    fn set_gamemode(&mut self, mode: GameMode);

    // Inventory
    fn inventory(&self) -> &dyn Inventory;
    fn inventory_mut(&mut self) -> &mut dyn Inventory;
    fn held_item(&self) -> Option<&ItemStack>;
    fn held_slot(&self) -> usize;

    // Communication
    fn send_message(&self, message: impl Into<TextComponent>);
    fn send_action_bar(&self, message: impl Into<TextComponent>);
    fn send_title(&self, title: impl Into<TextComponent>, subtitle: impl Into<TextComponent>);
    fn kick(&self, reason: impl Into<TextComponent>);
}
```

---

## 8. Shared Components (API-level)

Components in `ferrumc-api` that any mod can use:

```rust
// Marker components for cross-mod communication
#[derive(Component)] pub struct DamageImmune;
#[derive(Component)] pub struct Invulnerable; // No damage, no knockback
#[derive(Component)] pub struct NoClip;
#[derive(Component)] pub struct Flying;
#[derive(Component)] pub struct Invisible;
#[derive(Component)] pub struct Glowing;
#[derive(Component)] pub struct Silent;
#[derive(Component)] pub struct NoGravity;
#[derive(Component)] pub struct Frozen;

// Data components that mods can attach
#[derive(Component)] pub struct CustomName(pub TextComponent);
#[derive(Component)] pub struct Team(pub String);
#[derive(Component)] pub struct Score(pub i32);
```

---

## 9. Implementation Phases

### Phase 1: Core API (Current)
- [x] ModSystem trait
- [x] CoreApi / ServerApi traits
- [x] Basic behavior traits
- [x] Component providers
- [ ] Behavior registries integration

### Phase 2: Event System
- [ ] EventApi trait
- [ ] Event priority system
- [ ] Cancellable events
- [ ] Convert existing Bevy messages to events

### Phase 3: World Access
- [ ] WorldAccessor trait
- [ ] Block read/write API
- [ ] Entity spawning API
- [ ] Raycasting

### Phase 4: Commands
- [ ] Command builder API
- [ ] Argument parsers
- [ ] Player/entity selectors (@a, @p, etc.)
- [ ] Tab completion

### Phase 5: Inventory
- [ ] Inventory trait
- [ ] PlayerAccessor trait
- [ ] Container interactions
- [ ] Recipe system hooks

### Phase 6: Content Loading
- [ ] JSON asset loading for blocks/items
- [ ] assets_loaded / assets_finalize hooks
- [ ] Dynamic content registration

---

## 10. Example Mod

```rust
use ferrumc_api::prelude::*;
use std::sync::Arc;

pub struct MyMod;

impl ModSystem for MyMod {
    fn mod_id(&self) -> &'static str { "mymod:example" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn dependencies(&self) -> &'static [&'static str] { &["ferrumc:survival"] }

    fn start(&self, api: &mut dyn CoreApi) {
        // Register explosive block behavior
        api.register_block_behavior("minecraft:tnt", Arc::new(TntBehavior));

        // Register sword behavior
        api.register_item_behavior("minecraft:diamond_sword", Arc::new(SharpnessBehavior { level: 5 }));

        // Register command
        api.register_command(
            Command::new("boom")
                .description("Create an explosion")
                .permission("mymod.boom")
                .requires_player()
                .arg(FloatArg::new("radius").range(1.0, 10.0))
                .handler(|ctx| {
                    let radius = ctx.get_float("radius")?;
                    // Create explosion...
                    ctx.reply(format!("Created explosion with radius {}", radius));
                    Ok(())
                })
        );
    }

    fn start_server_side(&self, api: &mut dyn ServerApi) {
        // Subscribe to events
        api.events().on_player_death(EventPriority::Normal, |event| {
            if event.killer.is_some() {
                // PvP death - maybe award points?
            }
            Handling::Pass
        });

        // Register component provider
        api.register_player_component_provider(Arc::new(MyComponentProvider));
    }
}

struct TntBehavior;
impl BlockBehavior for TntBehavior {
    fn on_interact_start(&self, ctx: &mut BlockInteractContext) -> Handling {
        if ctx.held_item.map(|i| i.item_id == "minecraft:flint_and_steel").unwrap_or(false) {
            // Prime the TNT
            ctx.world.set_block(ctx.position, BlockState::AIR);
            ctx.world.spawn_entity("minecraft:tnt", ctx.position.center());
            Handling::PreventDefault
        } else {
            Handling::Pass
        }
    }
}

#[ctor::ctor]
fn register() {
    ferrumc_api_server::register_mod(Arc::new(MyMod));
}
```
