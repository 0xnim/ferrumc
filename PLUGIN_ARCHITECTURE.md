# FerrumC Plugin Architecture

**Comprehensive guide to the plugin system architecture and design principles**

---

## Table of Contents

1. [Core Principles](#core-principles)
2. [Architecture Overview](#architecture-overview)
3. [Domain API Crates](#domain-api-crates)
4. [Event System](#event-system)
5. [Plugin Lifecycle](#plugin-lifecycle)
6. [API Design Patterns](#api-design-patterns)
7. [Implementation Guide](#implementation-guide)
8. [Examples](#examples)

---

## Core Principles

### Separation of Concerns

```
┌─────────────────────────────────────────────┐
│  CORE INFRASTRUCTURE (src/lib/)             │
│  - Networking (TCP, packets, encryption)    │
│  - World storage (database, chunks)         │
│  - ECS framework (Bevy ECS)                 │
│  - NBT/Anvil parsing                        │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│  DOMAIN API CRATES (src/lib/apis/)          │
│  - Events (high-level gameplay events)      │
│  - Traits (BlockAPI, AnimationAPI, etc.)    │
│  - Types (domain models)                    │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│  PLUGINS (src/lib/plugins/)                 │
│  - Gameplay logic only                      │
│  - Use domain APIs                          │
│  - No direct I/O access                     │
└─────────────────────────────────────────────┘
```

### Key Rules

1. **Core handles I/O** - Packets, network, database operations stay in core
2. **APIs define contracts** - Domain APIs provide the interface between core and plugins
3. **Plugins contain logic** - Game rules, validation, mechanics live in plugins
4. **No I/O in plugins** - Plugins never directly access packets, network, or database

---

## Architecture Overview

### Directory Structure

```
src/lib/
├── core/                    # ECS components, base types
├── net/                     # Networking infrastructure
├── world/                   # World/chunk infrastructure
├── storage/                 # Database backend
├── utils/                   # Utilities
│
├── apis/                    # 🆕 Domain-specific API crates
│   ├── animation-api/       # Animation events, traits, types
│   ├── block-api/           # Block events, traits, types
│   ├── chat-api/            # Chat events, traits, types
│   ├── entity-api/          # Entity tracking events, traits
│   ├── inventory-api/       # Inventory events, traits
│   ├── movement-api/        # Movement events, traits
│   └── combat-api/          # Combat events, traits (future)
│
├── plugin-api/              # Base plugin system
│   ├── src/
│   │   ├── lib.rs           # Plugin trait
│   │   ├── context.rs       # PluginContext
│   │   └── macros.rs        # Helper macros
│   └── Cargo.toml
│
└── plugins/
    ├── core/                # Essential gameplay (can't disable)
    │   ├── animations/      # Animation logic
    │   ├── blocks/          # Block placement/breaking rules
    │   ├── chat/            # Chat formatting and broadcasting
    │   ├── movement/        # Movement validation
    │   └── inventory/       # Inventory management
    │
    └── gameplay/            # Optional features (can disable)
        ├── combat/          # PvP/PvE mechanics (future)
        ├── health/          # Health/hunger mechanics (future)
        └── entities/        # Mob AI (future)
```

---

## Domain API Crates

### Purpose

Domain API crates sit between core infrastructure and plugins. They provide:

1. **Events** - High-level gameplay events (not raw packets)
2. **Traits** - APIs that plugins use to interact with the game
3. **Types** - Domain models and data structures

### Example: Animation API

**File:** `src/lib/apis/animation-api/Cargo.toml`
```toml
[package]
name = "ferrumc-animation-api"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy_ecs = { workspace = true }
ferrumc-core = { workspace = true }

[lints]
workspace = true
```

**File:** `src/lib/apis/animation-api/src/lib.rs`
```rust
pub mod events;
pub mod traits;
pub mod types;

pub use events::*;
pub use traits::*;
pub use types::*;
```

**File:** `src/lib/apis/animation-api/src/types.rs`
```rust
/// Animation types that can be played
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    SwingMainArm,
    SwingOffhand,
    TakeDamage,
    LeaveBed,
    CriticalEffect,
    MagicCriticalEffect,
}

/// Which hand is being used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Main,
    Off,
}
```

**File:** `src/lib/apis/animation-api/src/events.rs`
```rust
use bevy_ecs::prelude::*;
use super::types::*;

/// High-level event: Player swung their arm
#[derive(Event, Clone)]
pub struct PlayerSwingArmEvent {
    pub player: Entity,
    pub hand: Hand,
}

/// Request to play an animation for an entity
#[derive(Event, Clone)]
pub struct PlayAnimationRequest {
    pub entity: Entity,
    pub animation: AnimationType,
    pub broadcast_to_all: bool,  // true = all players, false = nearby only
}
```

**File:** `src/lib/apis/animation-api/src/traits.rs`
```rust
use bevy_ecs::prelude::*;
use super::events::*;
use super::types::*;

/// Plugin API for triggering animations
pub trait AnimationAPI {
    /// Play an animation for an entity, visible to nearby players
    fn play_animation(&mut self, entity: Entity, animation: AnimationType);
    
    /// Play animation visible to all players
    fn play_animation_global(&mut self, entity: Entity, animation: AnimationType);
}

impl AnimationAPI for World {
    fn play_animation(&mut self, entity: Entity, animation: AnimationType) {
        self.send_event(PlayAnimationRequest {
            entity,
            animation,
            broadcast_to_all: false,
        });
    }
    
    fn play_animation_global(&mut self, entity: Entity, animation: AnimationType) {
        self.send_event(PlayAnimationRequest {
            entity,
            animation,
            broadcast_to_all: true,
        });
    }
}
```

---

## Event System

### Event Flow

```
1. Network Layer (Core)
   ↓ Receives raw packet
   
2. Packet Handler (Core)
   ↓ Converts packet → high-level event
   
3. Domain Event (API Crate)
   ↓ Emitted to ECS world
   
4. Plugin System
   ↓ Reads event, applies game logic
   
5. Response Event (API Crate)
   ↓ Plugin emits response/request
   
6. Core Systems
   ↓ Processes response, performs I/O
   
7. Network Layer (Core)
   ↓ Sends packets to clients
```

### Event Types

#### 1. Input Events (from network)

Events triggered by player actions:

```rust
// From raw packets → high-level events
#[derive(Event, Clone)]
pub struct PlayerSwingArmEvent {
    pub player: Entity,
    pub hand: Hand,
}

#[derive(Event, Clone)]
pub struct BlockPlaceAttempt {
    pub player: Entity,
    pub position: BlockPos,
    pub face: BlockFace,
    pub item_in_hand: ItemStack,
}
```

#### 2. Request Events (to core)

Plugins request core to perform actions:

```rust
#[derive(Event, Clone)]
pub struct PlayAnimationRequest {
    pub entity: Entity,
    pub animation: AnimationType,
    pub broadcast_to_all: bool,
}

#[derive(Event, Clone)]
pub struct SetBlockRequest {
    pub position: BlockPos,
    pub block: BlockStateId,
}
```

#### 3. Cancellable Events

Events that plugins can veto:

```rust
use std::cell::Cell;

#[derive(Event)]
pub struct BlockPlaceAttempt {
    pub player: Entity,
    pub position: BlockPos,
    pub block: BlockStateId,
    
    // Internal cancellation state
    cancelled: Cell<bool>,
    cancel_reason: Cell<Option<String>>,
}

impl BlockPlaceAttempt {
    pub fn cancel(&self, reason: impl Into<String>) {
        self.cancelled.set(true);
        self.cancel_reason.set(Some(reason.into()));
    }
    
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.get()
    }
    
    pub fn cancel_reason(&self) -> Option<String> {
        self.cancel_reason.take()
    }
}
```

**Usage:**
```rust
// Anti-cheat plugin (runs first due to priority)
fn validate_placement(mut events: EventReader<BlockPlaceAttempt>) {
    for event in events.read() {
        if is_placing_too_fast(event.player) {
            event.cancel("Placing blocks too quickly");
        }
    }
}

// Blocks plugin (runs after validation)
fn handle_placement(
    mut events: EventReader<BlockPlaceAttempt>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        if event.is_cancelled() {
            continue;  // Respect cancellation
        }
        
        // Proceed with placement
        state.0.set_block(event.position, event.block);
    }
}
```

#### 4. Response Events

Validation responses with success/failure:

```rust
use uuid::Uuid;

#[derive(Event)]
pub struct BlockPlaceRequest {
    pub request_id: Uuid,
    pub player: Entity,
    pub position: BlockPos,
    pub block: BlockStateId,
}

#[derive(Event)]
pub struct BlockPlaceResponse {
    pub request_id: Uuid,
    pub success: bool,
    pub reason: Option<DenialReason>,
}

#[derive(Debug, Clone)]
pub enum DenialReason {
    Collision,
    NoPermission,
    ProtectedArea,
    InvalidBlock,
}
```

---

## Plugin Lifecycle

### Plugin Trait

```rust
pub trait Plugin: Send + Sync + 'static {
    /// Unique plugin identifier
    fn name(&self) -> &'static str;
    
    /// Plugin version (semantic versioning)
    fn version(&self) -> &'static str;
    
    /// Plugin author(s)
    fn author(&self) -> &'static str {
        "Unknown"
    }
    
    /// Description of what the plugin does
    fn description(&self) -> &'static str {
        ""
    }
    
    /// Names of plugins this plugin depends on
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }
    
    /// Execution priority (higher = runs first)
    /// Default: 0
    /// Anti-cheat: 100
    /// Logging: -100
    fn priority(&self) -> i32 {
        0
    }
    
    // Lifecycle hooks
    
    /// Called when plugin is loaded (before build)
    fn on_load(&mut self) {}
    
    /// Build the plugin - register systems, events, resources
    fn build(&self, ctx: &mut PluginContext<'_>);
    
    /// Called when server is ready (after all plugins built)
    fn on_enable(&mut self) {}
    
    /// Called when server is shutting down
    fn on_disable(&mut self) {}
    
    /// Called when plugin configuration changes
    fn on_config_reload(&mut self, new_config: &PluginConfig) {}
}
```

### Lifecycle Phases

```
1. Registration (compile-time)
   ↓ Plugin registered in plugin_loader.rs
   
2. Configuration Loading
   ↓ plugins.toml loaded
   
3. Dependency Resolution
   ↓ Topological sort by dependencies
   
4. on_load() Hook
   ↓ Plugin initialization
   
5. build() Hook
   ↓ Register systems, events, resources
   
6. on_enable() Hook
   ↓ Server is ready
   
7. Runtime
   ↓ Systems run every tick
   
8. on_disable() Hook
   ↓ Server shutting down
```

---

## API Design Patterns

### Pattern 1: Query vs Mutate Separation

Separate read-only from write operations:

```rust
// Read-only queries
pub trait BlockQuery {
    fn get_block(&self, pos: BlockPos) -> Option<BlockStateId>;
    fn is_solid(&self, pos: BlockPos) -> bool;
    fn get_light_level(&self, pos: BlockPos) -> u8;
}

// Write operations
pub trait BlockMutate {
    fn set_block(&mut self, pos: BlockPos, block: BlockStateId) -> Result<()>;
    fn break_block(&mut self, pos: BlockPos) -> Result<BlockStateId>;
}

impl BlockQuery for GlobalState {
    fn get_block(&self, pos: BlockPos) -> Option<BlockStateId> {
        let (cx, cz) = pos.to_chunk_coords();
        self.world.get_chunk(cx, cz).ok()
            .and_then(|chunk| chunk.get_block(pos.x, pos.y, pos.z))
    }
}

impl BlockMutate for GlobalState {
    fn set_block(&mut self, pos: BlockPos, block: BlockStateId) -> Result<()> {
        // Implementation
    }
}
```

### Pattern 2: Contextual APIs

Provide scoped access instead of global state:

```rust
pub struct WorldContext<'w> {
    state: &'w GlobalState,
}

impl<'w> WorldContext<'w> {
    pub fn get_block(&self, pos: BlockPos) -> Option<BlockStateId> {
        // Scoped implementation
    }
}

// System signature uses context
fn handle_block_placement(
    world: WorldContext,  // Scoped, not Res<GlobalStateResource>
    events: EventReader<BlockPlaceAttempt>,
) {
    // Plugin only has access to what WorldContext exposes
}
```

### Pattern 3: Typed Entity Handles

Use typed wrappers for type safety:

```rust
#[derive(Copy, Clone, Debug)]
pub struct PlayerHandle(pub(crate) Entity);

#[derive(Copy, Clone, Debug)]
pub struct MobHandle(pub(crate) Entity);

impl PlayerHandle {
    pub fn entity(&self) -> Entity {
        self.0
    }
}

// API uses typed handles
pub trait PlayerAPI {
    fn teleport(&mut self, player: PlayerHandle, pos: Position);
    fn send_message(&self, player: PlayerHandle, msg: &str);
}

// Can't accidentally pass a mob to player API
```

### Pattern 4: Builder Pattern for Complex Events

```rust
pub struct BlockPlaceAttemptBuilder {
    player: Entity,
    position: BlockPos,
    block: BlockStateId,
    face: Option<BlockFace>,
    cursor_position: Option<CursorPosition>,
}

impl BlockPlaceAttemptBuilder {
    pub fn new(player: Entity, position: BlockPos, block: BlockStateId) -> Self {
        Self {
            player,
            position,
            block,
            face: None,
            cursor_position: None,
        }
    }
    
    pub fn face(mut self, face: BlockFace) -> Self {
        self.face = Some(face);
        self
    }
    
    pub fn cursor_position(mut self, pos: CursorPosition) -> Self {
        self.cursor_position = Some(pos);
        self
    }
    
    pub fn build(self) -> BlockPlaceAttempt {
        BlockPlaceAttempt {
            player: self.player,
            position: self.position,
            block: self.block,
            face: self.face.unwrap_or(BlockFace::Up),
            cursor_position: self.cursor_position.unwrap_or_default(),
            cancelled: Cell::new(false),
            cancel_reason: Cell::new(None),
        }
    }
}
```

### Pattern 5: Helper Macros

Reduce boilerplate:

```rust
// src/lib/plugin-api/src/macros.rs

#[macro_export]
macro_rules! register_events {
    ($ctx:expr, $($event:ty),* $(,)?) => {
        $(
            $ctx.register_event::<$event>();
        )*
    };
}

// Usage
fn build(&self, ctx: &mut PluginContext<'_>) {
    register_events!(ctx,
        BlockPlaceAttempt,
        BlockBreakAttempt,
        BlockChanged,
    );
}
```

---

## Implementation Guide

### Step 1: Create Domain API Crate

```bash
# Create crate structure
mkdir -p src/lib/apis/animation-api/src
cd src/lib/apis/animation-api
```

**Create Cargo.toml:**
```toml
[package]
name = "ferrumc-animation-api"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy_ecs = { workspace = true }
ferrumc-core = { workspace = true }

[lints]
workspace = true
```

**Create source files:**
```rust
// src/lib.rs
pub mod events;
pub mod traits;
pub mod types;

pub use events::*;
pub use traits::*;
pub use types::*;

// src/types.rs
#[derive(Debug, Clone, Copy)]
pub enum AnimationType {
    SwingMainArm,
    SwingOffhand,
}

// src/events.rs
use bevy_ecs::prelude::*;
use super::types::*;

#[derive(Event, Clone)]
pub struct PlayerSwingArmEvent {
    pub player: Entity,
    pub hand: Hand,
}

// src/traits.rs
pub trait AnimationAPI {
    fn play_animation(&mut self, entity: Entity, animation: AnimationType);
}
```

### Step 2: Implement Packet → Event Converter (Core)

```rust
// src/lib/core/src/animations/packet_handlers.rs (or similar)

use ferrumc_animation_api::*;
use ferrumc_net::SwingArmPacketReceiver;

/// Core system: Converts raw packets into high-level events
pub fn handle_swing_arm_packets(
    packets: Res<SwingArmPacketReceiver>,
    mut events: EventWriter<PlayerSwingArmEvent>,
) {
    for (packet, entity) in packets.0.try_iter() {
        events.send(PlayerSwingArmEvent {
            player: entity,
            hand: if packet.hand == 0 { Hand::Main } else { Hand::Off },
        });
    }
}
```

### Step 3: Implement Event → Packet Broadcaster (Core)

```rust
// src/lib/core/src/animations/broadcast.rs

use ferrumc_animation_api::*;
use ferrumc_net::packets::outgoing::entity_animation::EntityAnimationPacket;

/// Core system: Converts high-level requests into network packets
pub fn broadcast_animations(
    mut requests: EventReader<PlayAnimationRequest>,
    query: Query<&PlayerIdentity>,
    conn_query: Query<(Entity, &StreamWriter)>,
    state: Res<GlobalStateResource>,
) {
    for req in requests.read() {
        let animation_id = match req.animation {
            AnimationType::SwingMainArm => 0,
            AnimationType::SwingOffhand => 3,
        };
        
        let game_id = query.get(req.entity).expect("Game ID not found");
        let packet = EntityAnimationPacket::new(
            VarInt::new(game_id.short_uuid),
            animation_id
        );
        
        // Broadcast logic
        for (entity, conn) in conn_query.iter() {
            if !req.broadcast_to_all && entity == req.entity {
                continue;
            }
            if !state.0.players.is_connected(entity) {
                continue;
            }
            conn.send_packet_ref(&packet).ok();
        }
    }
}
```

### Step 4: Create Plugin

```rust
// src/lib/plugins/core/animations/src/lib.rs

use ferrumc_plugin_api::*;
use ferrumc_animation_api::*;

#[derive(Default)]
pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn name(&self) -> &'static str { "animations" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn description(&self) -> &'static str {
        "Handles entity animations like arm swings and damage effects"
    }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        // Register events
        ctx.register_event::<PlayerSwingArmEvent>();
        ctx.register_event::<PlayAnimationRequest>();
        
        // Register gameplay logic systems
        ctx.add_tick_system(handle_player_swings);
    }
}

/// Plugin logic: When player swings, trigger animation
fn handle_player_swings(
    mut events: EventReader<PlayerSwingArmEvent>,
    world: &mut World,
) {
    for event in events.read() {
        let animation = match event.hand {
            Hand::Main => AnimationType::SwingMainArm,
            Hand::Off => AnimationType::SwingOffhand,
        };
        
        // Use the API
        world.play_animation(event.player, animation);
    }
}
```

### Step 5: Register Core Systems

```rust
// src/bin/src/game_loop.rs or similar

// In tick schedule building:
fn build_tick(s: &mut Schedule) {
    // Core packet handlers
    s.add_systems(ferrumc_core::animations::packet_handlers::handle_swing_arm_packets);
    
    // Core broadcasters
    s.add_systems(ferrumc_core::animations::broadcast::broadcast_animations);
    
    // Other core systems...
}
```

### Step 6: Register Plugin

```rust
// src/bin/src/plugin_loader.rs

pub fn create_plugin_registry() -> Result<PluginRegistry, PluginError> {
    let mut registry = PluginRegistry::new();
    registry.load_config("plugins.toml")?;
    
    // Register plugins
    registry.register::<ferrumc_plugin_animations::AnimationsPlugin>();
    
    registry.sort_by_dependencies()?;
    Ok(registry)
}
```

---

## Examples

### Example 1: Complete Animation System

**API Crate:** `ferrumc-animation-api`

```rust
// src/lib/apis/animation-api/src/types.rs
#[derive(Debug, Clone, Copy)]
pub enum AnimationType {
    SwingMainArm = 0,
    SwingOffhand = 3,
    TakeDamage = 1,
    LeaveBed = 2,
    CriticalEffect = 4,
    MagicCriticalEffect = 5,
}

#[derive(Debug, Clone, Copy)]
pub enum Hand {
    Main = 0,
    Off = 1,
}

// src/lib/apis/animation-api/src/events.rs
#[derive(Event, Clone)]
pub struct PlayerSwingArmEvent {
    pub player: Entity,
    pub hand: Hand,
}

#[derive(Event, Clone)]
pub struct PlayAnimationRequest {
    pub entity: Entity,
    pub animation: AnimationType,
    pub broadcast_to_all: bool,
}

// src/lib/apis/animation-api/src/traits.rs
pub trait AnimationAPI {
    fn play_animation(&mut self, entity: Entity, animation: AnimationType);
    fn play_animation_global(&mut self, entity: Entity, animation: AnimationType);
}

impl AnimationAPI for World {
    fn play_animation(&mut self, entity: Entity, animation: AnimationType) {
        self.send_event(PlayAnimationRequest {
            entity,
            animation,
            broadcast_to_all: false,
        });
    }
    
    fn play_animation_global(&mut self, entity: Entity, animation: AnimationType) {
        self.send_event(PlayAnimationRequest {
            entity,
            animation,
            broadcast_to_all: true,
        });
    }
}
```

**Plugin:** `ferrumc-plugin-animations`

```rust
// src/lib/plugins/core/animations/src/lib.rs
use ferrumc_plugin_api::*;
use ferrumc_animation_api::*;

#[derive(Default)]
pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn name(&self) -> &'static str { "animations" }
    fn version(&self) -> &'static str { "1.0.0" }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        register_events!(ctx,
            PlayerSwingArmEvent,
            PlayAnimationRequest,
        );
        
        ctx.add_tick_system(handle_player_swings);
        ctx.add_tick_system(handle_damage_animations);
    }
}

fn handle_player_swings(
    mut events: EventReader<PlayerSwingArmEvent>,
    world: &mut World,
) {
    for event in events.read() {
        let animation = match event.hand {
            Hand::Main => AnimationType::SwingMainArm,
            Hand::Off => AnimationType::SwingOffhand,
        };
        world.play_animation(event.player, animation);
    }
}

fn handle_damage_animations(
    mut events: EventReader<PlayerDamageEvent>,  // From health-api
    world: &mut World,
) {
    for event in events.read() {
        if event.amount > 0.0 {
            world.play_animation(event.player, AnimationType::TakeDamage);
        }
    }
}
```

### Example 2: Block System with Cancellation

**API Crate:** `ferrumc-block-api`

```rust
// src/lib/apis/block-api/src/events.rs
use std::cell::Cell;

#[derive(Event)]
pub struct BlockPlaceAttempt {
    pub player: Entity,
    pub position: BlockPos,
    pub block: BlockStateId,
    pub face: BlockFace,
    
    cancelled: Cell<bool>,
    cancel_reason: Cell<Option<String>>,
}

impl BlockPlaceAttempt {
    pub fn new(player: Entity, position: BlockPos, block: BlockStateId, face: BlockFace) -> Self {
        Self {
            player,
            position,
            block,
            face,
            cancelled: Cell::new(false),
            cancel_reason: Cell::new(None),
        }
    }
    
    pub fn cancel(&self, reason: impl Into<String>) {
        self.cancelled.set(true);
        self.cancel_reason.set(Some(reason.into()));
    }
    
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.get()
    }
}

#[derive(Event)]
pub struct BlockChanged {
    pub position: BlockPos,
    pub old_block: BlockStateId,
    pub new_block: BlockStateId,
}

// src/lib/apis/block-api/src/traits.rs
pub trait BlockAPI {
    fn set_block(&self, pos: BlockPos, block: BlockStateId) -> Result<()>;
    fn get_block(&self, pos: BlockPos) -> Option<BlockStateId>;
    fn can_place_block(&self, pos: BlockPos, player: Entity) -> bool;
}
```

**Plugin:** `ferrumc-plugin-blocks`

```rust
// src/lib/plugins/core/blocks/src/lib.rs
use ferrumc_plugin_api::*;
use ferrumc_block_api::*;

#[derive(Default)]
pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn name(&self) -> &'static str { "blocks" }
    fn version(&self) -> &'static str { "1.0.0" }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        register_events!(ctx,
            BlockPlaceAttempt,
            BlockBreakAttempt,
            BlockChanged,
        );
        
        ctx.add_tick_system(validate_block_placement);
        ctx.add_tick_system(handle_block_placement);
        ctx.add_tick_system(handle_block_breaking);
    }
}

// Validation system (runs early due to system ordering)
fn validate_block_placement(
    mut events: EventReader<BlockPlaceAttempt>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        // Check collision
        if !state.0.can_place_block(event.position, event.player) {
            event.cancel("Cannot place block here - collision detected");
            continue;
        }
        
        // More validation...
    }
}

// Placement system (runs after validation)
fn handle_block_placement(
    mut events: EventReader<BlockPlaceAttempt>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        if event.is_cancelled() {
            continue;
        }
        
        if let Err(e) = state.0.set_block(event.position, event.block) {
            error!("Failed to place block: {}", e);
        }
    }
}
```

---

## Best Practices

### 1. Keep APIs Small and Focused

❌ **Don't:** Create one giant `GameAPI` with everything
```rust
pub trait GameAPI {
    fn set_block(...);
    fn play_animation(...);
    fn send_chat(...);
    fn give_item(...);
    // Too many responsibilities!
}
```

✅ **Do:** Create focused domain APIs
```rust
pub trait BlockAPI { ... }
pub trait AnimationAPI { ... }
pub trait ChatAPI { ... }
pub trait InventoryAPI { ... }
```

### 2. Use Events for Async Communication

❌ **Don't:** Call methods directly across plugins
```rust
// Bad: Direct coupling
combat_plugin.deal_damage(player, amount);
```

✅ **Do:** Use events
```rust
// Good: Loose coupling
world.send_event(DealDamageEvent { player, amount });
```

### 3. Provide Both High and Low Level APIs

```rust
// High-level convenience
pub trait BlockAPI {
    fn set_block(&self, pos: BlockPos, block: BlockStateId) -> Result<()>;
}

// Low-level control
pub trait BlockAPIAdvanced {
    fn set_block_no_update(&self, pos: BlockPos, block: BlockStateId) -> Result<()>;
    fn set_block_with_data(&self, pos: BlockPos, block: BlockStateId, data: NBT) -> Result<()>;
}
```

### 4. Document Event Flow

Every API should document the event flow:

```rust
/// # Event Flow
/// 
/// 1. Network layer receives `PlaceBlockPacket`
/// 2. Core emits `BlockPlaceAttempt` event
/// 3. Validation plugins can cancel the event
/// 4. Blocks plugin processes non-cancelled events
/// 5. Core emits `BlockChanged` event
/// 6. Core broadcasts `BlockUpdate` packet to clients
#[derive(Event)]
pub struct BlockPlaceAttempt { ... }
```

### 5. Version Your APIs

```toml
[package]
name = "ferrumc-block-api"
version = "1.2.0"  # Semantic versioning

# Breaking changes = major version bump
# New features = minor version bump
# Bug fixes = patch version bump
```

---

## Migration Strategy

### Phase 1: Create API Crates (Week 1-2)

1. Create `animation-api` crate
2. Create `block-api` crate
3. Create `chat-api` crate
4. Create `movement-api` crate

### Phase 2: Implement Core Handlers (Week 2-3)

1. Move packet handlers to core
2. Implement packet → event converters
3. Implement event → packet broadcasters

### Phase 3: Create Plugins (Week 3-4)

1. Create `animations` plugin
2. Create `blocks` plugin
3. Create `chat` plugin
4. Create `movement` plugin

### Phase 4: Migration (Week 4-5)

1. Delete old handlers from binary
2. Test each plugin independently
3. Update documentation

---

## Summary

The FerrumC plugin architecture provides:

✅ **Clean separation** - Core handles I/O, plugins handle logic  
✅ **Flexible APIs** - Domain-specific API crates  
✅ **Event-driven** - Loose coupling via events  
✅ **Type-safe** - Compile-time guarantees  
✅ **Extensible** - Easy to add new features  
✅ **Testable** - Plugins can be tested in isolation  

This architecture enables rapid feature development while maintaining code quality and preventing spaghetti code.
