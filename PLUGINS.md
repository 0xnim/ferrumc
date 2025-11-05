# FerrumC Plugin System - Complete Guide

**Last Updated:** November 5, 2025  
**Status:** Architecture redesign in progress

---

## Table of Contents

1. [Overview](#overview)
2. [Current Architecture](#current-architecture)
3. [Architecture Problems](#architecture-problems)
4. [Redesigned Architecture](#redesigned-architecture)
5. [Migration Guide](#migration-guide)
6. [Current Plugin Status](#current-plugin-status)
7. [API Reference](#api-reference)

---

## Overview

FerrumC uses **compiled-in plugins** to organize gameplay features separately from infrastructure.

### Core Principle

```
Core Infrastructure (I/O)
    ↓ provides mechanisms
Domain APIs (contracts)
    ↓ consumed by
Plugins (game logic)
```

**Rules:**
- **Core** handles all I/O (packets, network, database, world operations)
- **Domain APIs** define contracts between core and plugins
- **Plugins** contain only game logic (no I/O allowed)

### Why Plugins?

Not for third-party extensions, but for **architectural separation**:
- ✅ Clean boundaries between infrastructure and gameplay
- ✅ Modular code organization
- ✅ Easy to add/remove features
- ✅ Testable in isolation
- ✅ Prevents spaghetti code

**Question to ask:** "Could the server boot without this?" 
- **No** → Core infrastructure
- **Yes** → Plugin

---

## Current Architecture

### Directory Structure

```
src/lib/
├── core/                  # ECS components, base types
├── net/                   # Networking infrastructure
├── world/                 # World/chunk storage
├── storage/               # Database backend
├── utils/                 # Utilities
│
├── apis/                  # Domain API crates
│   ├── animation-api/     # Animation events, traits, types
│   ├── block-api/         # Block events, traits, types
│   ├── chat-api/          # Chat events, traits, types
│   └── inventory-api/     # Inventory events, traits
│
├── plugin-api/            # Base plugin system
│   ├── context.rs         # PluginContext
│   ├── entity.rs          # EntityExt helpers
│   ├── events.rs          # Common events
│   └── world.rs           # WorldExt helpers
│
└── plugins/               # Gameplay plugins
    ├── core/              # Essential (can't disable)
    │   ├── animations/    # Animation logic
    │   ├── blocks/        # Block placement/breaking
    │   ├── chat/          # Chat formatting
    │   ├── inventory/     # Inventory management
    │   └── default-commands/  # Command bridge
    │
    └── examples/
        └── hello/         # Example plugin
```

### Plugin Trait

```rust
pub trait Plugin: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn author(&self) -> &'static str { "Unknown" }
    fn description(&self) -> &'static str { "" }
    fn dependencies(&self) -> Vec<&'static str> { vec![] }
    fn priority(&self) -> i32 { 0 }
    
    fn build(&self, ctx: &mut PluginContext<'_>);
}
```

### Current Plugin Example

```rust
use ferrumc_plugin_api::*;
use ferrumc_animation_api::AnimationAPI;

#[derive(Default)]
pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn name(&self) -> &'static str { "animations" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn priority(&self) -> i32 { 50 }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        ctx.register_event::<PlayerSwingArmEvent>();
        ctx.add_tick_system(handle_player_swings);
    }
}

fn handle_player_swings(
    mut events: EventReader<PlayerSwingArmEvent>,
    mut animations: AnimationAPI,
) {
    for event in events.read() {
        let animation = match event.hand {
            Hand::Main => AnimationType::SwingMainArm,
            Hand::Off => AnimationType::SwingOffhand,
        };
        animations.play_animation(event.player, animation);
    }
}
```

---

## Architecture Problems

### Problem 1: No Enforcement

```rust
// ❌ Current PluginContext gives everything
pub struct PluginContext<'a> {
    pub world: &'a mut World,     // Can query anything!
    pub state: GlobalState,        // Can access database!
}

// Nothing prevents this:
fn bad_plugin(
    mut conn: Query<&StreamWriter>,     // Direct network access
    state: Res<GlobalStateResource>,    // Direct database access
) {
    // Plugin can violate separation principles
}
```

**Issue:** "Please don't do I/O" is documentation, not enforcement.

### Problem 2: Bypassable APIs

```rust
// Domain APIs are optional wrappers
fn good_plugin(mut api: AnimationAPI) { }      // Suggested
fn bad_plugin(mut events: EventWriter<...>) { } // Also works, bypasses API
```

**Issue:** Plugins can bypass domain APIs entirely.

### Problem 3: Mixed Responsibilities

```rust
// PluginContext does too much
impl<'a> PluginContext<'a> {
    pub world: &'a mut World,        // Access to everything
    pub state: GlobalState,           // World + database
    pub add_tick_system(...);         // Scheduler access
    pub register_event(...);          // Event registration
}
```

**Issue:** No separation between different concerns.

### Problem 4: Events in Wrong Place

`plugin-api/events.rs` contains domain events like `BlockPlacedEvent`, `PlayerDamageEvent`.

**Issue:** Should be in domain API crates (e.g., `block-api`), not base plugin-api.

### Problem 5: No Capability System

Plugins can:
- Query any component (including `StreamWriter`)
- Access any resource (including database)
- Call any system
- No permissions, no boundaries

---

## Redesigned Architecture

### Core Principle: Capability-Based Access

**Plugins declare capabilities upfront. Type system enforces boundaries.**

### Phase 1: Plugin Manifest

```rust
impl Plugin for AnimationsPlugin {
    fn name(&self) -> &'static str { "animations" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn priority(&self) -> i32 { 50 }
    
    /// NEW: Declare required capabilities
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_animation_api()      // Request animation domain
            .with_entity_queries()     // Request entity queries
            .build()
    }
    
    fn build(&self, ctx: PluginBuildContext<'_>) {
        ctx.systems()
            .add_tick(handle_player_swings);
    }
}
```

### Phase 2: Capabilities System

```rust
#[derive(Default, Clone)]
pub struct PluginCapabilities {
    // Domain API access
    pub animation_api: bool,
    pub block_api: bool,
    pub chat_api: bool,
    pub entity_api: bool,
    
    // Query capabilities
    pub entity_queries: bool,    // Query entities/components
    pub world_queries: bool,     // Query world/chunks (read-only)
    
    // Resource capabilities
    pub resources: Vec<ResourceCapability>,
    
    // NEVER allowed in plugins:
    // pub network_access: false,  // ❌ Not even an option
    // pub database_access: false, // ❌ Not even an option
}

impl PluginCapabilities {
    pub fn builder() -> PluginCapabilitiesBuilder { ... }
}
```

### Phase 3: Build Context (Type-Safe)

```rust
/// Context provided during plugin build
/// 
/// NO direct World or State access!
pub struct PluginBuildContext<'a> {
    capabilities: PluginCapabilities,
    systems: SystemRegistry<'a>,
    events: EventRegistry<'a>,
    config: PluginConfig,
    
    // NO world: &mut World
    // NO state: GlobalState
}

impl<'a> PluginBuildContext<'a> {
    pub fn systems(&mut self) -> &mut SystemRegistry<'a>;
    pub fn events(&mut self) -> &mut EventRegistry<'a>;
    pub fn config(&self) -> &PluginConfig;
}
```

### Phase 4: Domain APIs (Enforced)

```rust
// Animation API - THE ONLY way to interact with animations
#[derive(SystemParam)]
pub struct AnimationAPI<'w> {
    // Private fields - can't bypass
    animation_requests: EventWriter<'w, PlayAnimationRequest>,
    swing_events: EventReader<'w, PlayerSwingArmEvent>,
}

impl<'w> AnimationAPI<'w> {
    // Commands (Plugin → Core)
    pub fn play_animation(&mut self, entity: Entity, animation: AnimationType);
    
    // Events (Core → Plugin)
    pub fn swing_events(&mut self) -> &mut EventReader<'w, PlayerSwingArmEvent>;
}

// Can't do: EventWriter<PlayAnimationRequest> directly
// Must use AnimationAPI
```

### Phase 5: Restricted Query System

```rust
/// Safe entity queries - ONLY safe components allowed
#[derive(SystemParam)]
pub struct EntityQueries<'w, 's> {
    // Allow querying safe components
    positions: Query<'w, 's, &'static Position>,
    identities: Query<'w, 's, &'static PlayerIdentity>,
    
    // NO Query<&StreamWriter> allowed!
    // NO Query<&mut GlobalState> allowed!
}

impl<'w, 's> EntityQueries<'w, 's> {
    pub fn position(&self, entity: Entity) -> Option<&Position>;
    pub fn iter_players(&self) -> impl Iterator<...>;
    pub fn players_in_range(&self, center: &Position, range: f64) -> Vec<Entity>;
}
```

### Phase 6: Compile-Time Enforcement

```rust
// Plugins CANNOT import these (not public to plugins)
use ferrumc_net::connection::StreamWriter;  // ❌ Compile error
use ferrumc_storage::lmdb::LmdbBackend;     // ❌ Compile error

// Plugins CAN ONLY import domain APIs
use ferrumc_animation_api::AnimationAPI;    // ✅ OK
use ferrumc_block_api::BlockRequests;       // ✅ OK
use ferrumc_plugin_api::EntityQueries;      // ✅ OK
```

### Redesigned Plugin Example

```rust
use ferrumc_plugin_api::*;
use ferrumc_animation_api::{AnimationReader, AnimationWriter};

#[derive(Default)]
pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn name(&self) -> &'static str { "animations" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn priority(&self) -> i32 { 50 }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_animation_api()  // Only what we need
            .build()
    }
    
    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        ctx.events()
            .register::<PlayerSwingArmEvent>();
        
        ctx.systems()
            .add_tick(handle_player_swings);
    }
}

// System uses ONLY domain API
fn handle_player_swings(
    mut anim_reader: AnimationReader,
    mut anim_writer: AnimationWriter,
) {
    for event in anim_reader.swing_events().read() {
        let animation = match event.hand {
            Hand::Main => AnimationType::SwingMainArm,
            Hand::Off => AnimationType::SwingOffhand,
        };
        anim_writer.play_animation(event.player, animation);
    }
}

// ❌ CAN'T DO THIS - Type system prevents it:
// fn bad(conn: Query<&StreamWriter>) { }  // StreamWriter not in scope
```

### Benefits of Redesign

| Old | New |
|-----|-----|
| "Please don't access packets/database" | "You CAN'T access packets/database" |
| Documentation-based separation | Type-system enforced separation |
| Domain APIs are optional wrappers | Domain APIs are mandatory gateways |
| Direct World/State access | Capability-based access only |
| No visibility into plugin needs | Explicit capability declarations |

---

## Migration Guide

### Migration Path (5 Weeks)

**Week 1: Add Capability System**
1. Create `PluginCapabilities` type
2. Add `capabilities()` to Plugin trait (default: all for backward compat)
3. Create `PluginBuildContext` wrapper

**Week 2: Domain API Consolidation**
1. Move events from `plugin-api/events.rs` to domain APIs
2. Consolidate domain APIs (readers + writers)
3. Make domain API fields private

**Week 3: Query APIs**
1. Create `EntityQueries` SystemParam
2. Create `WorldQueries` SystemParam
3. Update plugins to use new query APIs

**Week 4: Remove Direct Access**
1. Remove `world` and `state` from PluginContext
2. Enforce capability checks
3. Update all plugins to declare capabilities

**Week 5: Type-System Enforcement**
1. Make network/storage crates non-pub to plugins
2. Validate system params against capabilities
3. Full enforcement

### Backward Compatibility

During migration:
```rust
// Old plugins keep working with default capabilities
impl Plugin for OldPlugin {
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::all()  // Default: allow everything
    }
}

// New plugins declare explicitly
impl Plugin for NewPlugin {
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_animation_api()
            .build()
    }
}
```

---

## Current Plugin Status

### Migrated Plugins (6 core + 1 example)

| Plugin | Lines | Priority | Status | Issues |
|--------|-------|----------|--------|--------|
| **animations** | 140 | 50 | ✅ Complete | Good separation |
| **blocks** | 200 | 40 | ⚠️ Has I/O violations | Direct chunk loading/saving |
| **chat** | 90 | 30 | ✅ Complete | Good separation |
| **inventory** | 80 | 40 | ⚠️ Direct state access | Should use InventoryAPI |
| **default-commands** | 60 | 10 | ⚠️ Bridge plugin | Temporary, will be removed |
| **hello** (example) | 60 | 0 | ✅ Complete | Perfect example |

### Remaining Migrations

| Feature | Lines | Effort | Priority |
|---------|-------|--------|----------|
| **movement** | 430 | 1 week | HIGH - Core mechanic |
| **chunk-management** | 300 | 1 week | HIGH - Completes movement |
| **system-messages** | 30 | 1 day | MEDIUM - Polish |

**After these 3 migrations:**
- Binary: <100 lines of gameplay code (93% reduction!)
- 9 plugins operational
- All gameplay in plugins, all I/O in core

### Priority Guidelines

```
Anti-cheat (future):      100+  Run first (validation)
Animations:                50   Base systems
Movement:                  45   Base systems
Blocks:                    40   Validation/placement
Inventory:                 40   Management
Chunk Management:          35   Runs after movement
Chat:                      30   Formatting
System Messages:           20   Decorative
Default Commands:          10   Bridge (runs late)
Health (future):            0   Final processing
Hello (example):            0   No specific order
```

### Critical Issues to Fix

#### 🔴 **CRITICAL: BlocksPlugin I/O Violations**

```rust
// ❌ Current: Plugin does I/O directly
let chunk = state.0.world.load_chunk_owned(cx, cz, "overworld")?;
state.0.world.save_chunk(chunk, cx, cz, "overworld")?;
state.0.terrain_generator.generate_chunk(...);
```

**Fix:** Move chunk I/O to core-systems, use BlockAPI in plugin.

#### 🟡 **MEDIUM: Missing Priority Declarations**

All plugins should declare `priority()` for predictable execution order.

#### 🟡 **MEDIUM: Direct State Access**

InventoryPlugin uses `state.0.players.is_connected()` - should be an API method.

---

## API Reference

### Plugin Trait (Current)

```rust
pub trait Plugin: Send + Sync + 'static {
    /// Unique identifier (lowercase with underscores)
    fn name(&self) -> &'static str;
    
    /// Semantic version (e.g., "1.0.0")
    fn version(&self) -> &'static str;
    
    /// Author(s)
    fn author(&self) -> &'static str { "Unknown" }
    
    /// Description
    fn description(&self) -> &'static str { "" }
    
    /// Plugin dependencies (loaded first)
    fn dependencies(&self) -> Vec<&'static str> { vec![] }
    
    /// Execution priority (higher = runs first)
    /// Default: 0, Anti-cheat: 100+, Logging: -100
    fn priority(&self) -> i32 { 0 }
    
    /// Build plugin - register systems, events, resources
    fn build(&self, ctx: &mut PluginContext<'_>);
}
```

### PluginContext (Current)

```rust
pub struct PluginContext<'a> {
    pub world: &'a mut World,        // ⚠️ Too much access
    pub state: GlobalState,          // ⚠️ Too much access
    // ...
}

impl<'a> PluginContext<'a> {
    // System registration
    pub fn add_tick_system<M>(&mut self, system: impl IntoSystem<(), (), M>);
    pub fn add_timed_system<M>(&mut self, name: String, period: Duration, system: ...);
    
    // Event registration
    pub fn register_event<T: Event>(&mut self);
    
    // Resource registration
    pub fn insert_resource<R: Resource>(&mut self, resource: R);
    
    // Configuration
    pub fn get_config<T: Deserialize>(&self, key: &str) -> Option<T>;
}
```

### Domain APIs (Current)

**AnimationAPI:**
```rust
#[derive(SystemParam)]
pub struct AnimationAPI<'w> {
    animation_events: EventWriter<'w, PlayAnimationRequest>,
    pose_events: EventWriter<'w, SetEntityPoseRequest>,
}

impl<'w> AnimationAPI<'w> {
    pub fn play_animation(&mut self, entity: Entity, animation: AnimationType);
    pub fn play_animation_global(&mut self, entity: Entity, animation: AnimationType);
    pub fn set_pose(&mut self, entity: Entity, entity_id: VarInt, pose: EntityPose);
}
```

**BlockRequests:**
```rust
#[derive(SystemParam)]
pub struct BlockRequests<'w> {
    place_events: EventWriter<'w, PlaceBlockRequest>,
    break_events: EventWriter<'w, BreakBlockRequest>,
}

impl<'w> BlockRequests<'w> {
    pub fn place_block(&mut self, player: Entity, position: NetworkPosition, 
                       block: BlockStateId, sequence: VarInt);
    pub fn break_block(&mut self, player: Entity, position: NetworkPosition, 
                       sequence: VarInt);
}
```

**ChatAPI:**
```rust
#[derive(SystemParam)]
pub struct ChatAPI<'w> {
    events: EventWriter<'w, SendChatMessageRequest>,
}

impl<'w> ChatAPI<'w> {
    pub fn send_message(&mut self, player: Entity, message: TextComponent);
    pub fn broadcast(&mut self, message: TextComponent);
}
```

### Helper Traits (Current)

**EntityExt:**
```rust
pub trait EntityExt {
    fn players_in_range(&self, pos: &Position, range: f64) -> Vec<Entity>;
    fn broadcast_packet<P: NetEncode>(&self, packet: &P);
    fn broadcast_packet_in_range<P: NetEncode>(&self, packet: &P, pos: &Position, range: f64);
    fn player_count(&self) -> usize;
}
```

**WorldExt:**
```rust
pub trait WorldExt {
    fn get_chunk(&self, x: i32, z: i32, dimension: &str) -> Result<Chunk>;
    fn set_block_at(&mut self, x: i32, y: i32, z: i32, block: BlockStateId) -> Result<()>;
    fn get_block_at(&self, x: i32, y: i32, z: i32) -> Option<BlockStateId>;
    fn chunk_exists(&self, x: i32, z: i32, dimension: &str) -> bool;
}
```

---

## Quick Start

### Creating a New Plugin

1. **Create plugin crate:**
```bash
mkdir -p src/lib/plugins/core/my-feature/src
cd src/lib/plugins/core/my-feature
```

2. **Create Cargo.toml:**
```toml
[package]
name = "ferrumc-plugin-my-feature"
version = "0.1.0"
edition = "2021"

[dependencies]
ferrumc-plugin-api = { path = "../../../plugin-api" }
ferrumc-my-domain-api = { path = "../../../apis/my-domain-api" }
bevy_ecs = { workspace = true }
tracing = { workspace = true }

[lints]
workspace = true
```

3. **Create src/lib.rs:**
```rust
use ferrumc_plugin_api::*;

#[derive(Default)]
pub struct MyFeaturePlugin;

impl Plugin for MyFeaturePlugin {
    fn name(&self) -> &'static str { "my_feature" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn priority(&self) -> i32 { 50 }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        ctx.add_tick_system(my_system);
    }
}

fn my_system() {
    // Your logic here
}
```

4. **Register plugin:**

Add to workspace `Cargo.toml`:
```toml
[workspace]
members = [
    # ...
    "src/lib/plugins/core/my-feature",
]
```

Add to `src/bin/Cargo.toml`:
```toml
[dependencies]
ferrumc-plugin-my-feature = { path = "../lib/plugins/core/my-feature" }
```

Add to `src/bin/src/plugin_loader.rs`:
```rust
registry.register::<ferrumc_plugin_my_feature::MyFeaturePlugin>();
```

5. **Build and test:**
```bash
cargo build
./target/debug/ferrumc
```

---

## Best Practices

### ✅ Do

- Use domain APIs for all interactions
- Declare `priority()` for predictable ordering
- Declare `dependencies()` for required plugins
- Listen to events, don't poll
- Keep plugins focused on one feature
- Document what your plugin does
- Write tests for plugin logic

### ❌ Don't

- Access raw packets or network directly
- Directly manipulate database
- Load/save chunks in plugins (use API)
- Create global state outside ECS
- Hardcode values (use config)
- Ignore cancellation flags on events
- Bypass domain APIs with direct `EventWriter`

---

## Future Work

### Planned Domain APIs
- `movement-api` - Player movement events and validation
- `entity-api` - Entity tracking and spawning
- `health-api` - Health, damage, death mechanics
- `combat-api` - Attack, damage calculation

### Planned Plugins
- `entity-tracking` - Make players visible to each other
- `health` - Health, food, saturation systems
- `combat` - PvP/PvE mechanics
- `item-entities` - Drop/pickup items
- `crafting` - Crafting system
- `mobs` - Mob AI and spawning

### Architecture Improvements
See **Redesigned Architecture** section for full details of planned improvements.

---

## Getting Help

- **Discord:** https://discord.gg/qT5J8EMjwk
- **GitHub Issues:** Report bugs or ask questions
- **Documentation:** This file + [ARCHITECTURE.md](ARCHITECTURE.md)

---

**Last Updated:** November 5, 2025  
**Contributors:** FerrumC Team
