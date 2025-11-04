# FerrumC Development - Next Steps Guide

**For:** Next developer continuing FerrumC development  
**Date:** November 5, 2025  
**Status:** Plugin architecture designed, ready for implementation

---

## 🎯 Quick Start - What You Need to Know

### 1. Read These First (In Order)

1. **[AGENTS.md](AGENTS.md)** - Commands, conventions, quick reference (5 min)
2. **[ARCHITECTURE.md](ARCHITECTURE.md)** - System architecture overview (15 min)
3. **[PLUGIN_VS_CORE.md](PLUGIN_VS_CORE.md)** - **Critical:** Understand core vs plugins vs APIs (10 min)
4. **[PLUGIN_ARCHITECTURE.md](PLUGIN_ARCHITECTURE.md)** - **NEW!** Complete plugin architecture (30 min)
5. **[PLUGIN_QUICKSTART.md](PLUGIN_QUICKSTART.md)** - How to create plugins (15 min)
6. **[FEATURES.md](FEATURES.md)** - What's implemented, what's not (10 min)

**Total reading time:** ~1.5 hours to fully understand the codebase and architecture

---

## 📋 Current State

### ✅ What's Working

**Core Infrastructure (Complete):**
- ✅ Networking (TCP, packets, encryption, compression)
- ✅ World storage (LMDB, caching, chunk persistence)
- ✅ Chunk format (Anvil-compatible)
- ✅ World generation (noise-based terrain)
- ✅ ECS system (Bevy ECS)
- ✅ NBT/Anvil parsing (custom, optimized)
- ✅ Configuration system
- ✅ Command infrastructure
- ✅ **Plugin system (fully implemented)**

**Basic Gameplay (Partial):**
- 🚧 Player connections/spawning
- 🚧 Player movement (works but not synced to other players)
- 🚧 Block placement/breaking (works but messy code location)
- 🚧 Chat (works but not broadcast)
- 🚧 Commands (infrastructure works, few commands)
- 🚧 Inventory (basic)

### ❌ What's Missing (High Priority)

**Critical Gameplay Features:**
1. **Entity Tracking** - Players can't see each other yet
2. **Health/Damage** - No survival mechanics
3. **Chat Broadcasting** - Messages not sent to other players
4. **Item Entities** - Can't drop/pickup items
5. **Multiple Dimensions** - Only overworld works

See [FEATURES.md](FEATURES.md) for complete list.

---

## 🏗️ Architecture Overview

### The Plugin System Philosophy

FerrumC uses **compiled-in plugins** to organize features:

```
┌─────────────────────────────────────┐
│  Core Infrastructure (src/lib/)     │  ← The "engine"
│  - Networking, Database, ECS        │     (Don't touch unless bugs)
│  - World format, NBT parsing        │
│  - Utilities, Config                │
└─────────────────────────────────────┘
              ↓ provides mechanisms
┌─────────────────────────────────────┐
│  Plugins (src/lib/plugins/)         │  ← Gameplay features
│  - player-movement (mechanics)      │     (Add features here!)
│  - blocks (placement rules)         │
│  - combat, health, entities, etc.   │
└─────────────────────────────────────┘
              ↓ registered by
┌─────────────────────────────────────┐
│  Binary (src/bin/)                  │  ← Bootstrap only
│  - main.rs, game_loop.rs            │     (Minimal code)
│  - plugin_loader.rs                 │
└─────────────────────────────────────┘
```

**Key principle:**
- **Core** = Infrastructure (networking, database, world)
- **Plugins** = Gameplay features (movement rules, combat, commands)
- **Binary** = Bootstrap (just loads everything)

Read [PLUGIN_VS_CORE.md](PLUGIN_VS_CORE.md) for detailed explanation.

---

## 🚀 Recommended Development Path

### NEW ARCHITECTURE: Domain APIs + Plugins

The new architecture requires building in three phases:

1. **Phase 1:** Create domain API crates (events, traits, types)
2. **Phase 2:** Move packet handling to core, implement event converters
3. **Phase 3:** Create plugins that use domain APIs

### Path A: Implement Domain API Architecture (RECOMMENDED)

**Goal:** Build the three-layer architecture (Core ← APIs ← Plugins)

#### Phase 1: Create Domain API Crates (Week 1-2)

Create the API layer that bridges core and plugins.

##### 1. Create `animation-api` crate (2-3 hours)
**Why first:** Core gameplay, well-isolated

**Steps:**
```bash
# Create API crate structure
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
// src/types.rs - Domain types
#[derive(Debug, Clone, Copy)]
pub enum AnimationType {
    SwingMainArm, SwingOffhand, TakeDamage
}

#[derive(Debug, Clone, Copy)]
pub enum Hand { Main, Off }

// src/events.rs - High-level events
#[derive(Event, Clone)]
pub struct PlayerSwingArmEvent {
    pub player: Entity,
    pub hand: Hand,
}

#[derive(Event, Clone)]
pub struct PlayAnimationRequest {
    pub entity: Entity,
    pub animation: AnimationType,
}

// src/traits.rs - Plugin API
pub trait AnimationAPI {
    fn play_animation(&mut self, entity: Entity, animation: AnimationType);
}

impl AnimationAPI for World {
    fn play_animation(&mut self, entity: Entity, animation: AnimationType) {
        self.send_event(PlayAnimationRequest { entity, animation, broadcast_to_all: false });
    }
}
```

**Deliverable:** `ferrumc-animation-api` crate with events, types, and traits

##### 2. Create `block-api` crate (3-4 hours)

Similar structure to animation-api but for blocks:
- Events: `BlockPlaceAttempt` (cancellable), `BlockBreakAttempt`, `BlockChanged`
- Traits: `BlockAPI` (set_block, get_block, can_place_block)
- Types: `BlockPos`, `BlockFace`, `BlockStateId`

##### 3. Create `chat-api` crate (2-3 hours)

- Events: `ChatMessageReceived`, `SendChatMessage`
- Traits: `ChatAPI` (broadcast_message, send_to_player)
- Types: `ChatMessage`, `ChatColor`

##### 4. Create `movement-api` crate (2-3 hours)

- Events: `PlayerMoveEvent`, `PlayerRotateEvent`
- Traits: `MovementAPI` (teleport, set_velocity)
- Types: `Position`, `Rotation`, `Velocity`

**Total Phase 1:** ~10-15 hours to create API foundation

#### Phase 2: Implement Core Handlers (Week 2-3)

Move packet handling from binary to core, create event converters.

##### 1. Create core packet → event converters (4-6 hours)

**Create** `src/lib/core/src/animations/packet_handlers.rs`:
```rust
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

Repeat for: block packets, chat packets, movement packets

##### 2. Create core event → packet broadcasters (4-6 hours)

**Create** `src/lib/core/src/animations/broadcast.rs`:
```rust
pub fn broadcast_animations(
    mut requests: EventReader<PlayAnimationRequest>,
    query: Query<&PlayerIdentity>,
    conn_query: Query<&StreamWriter>,
) {
    for req in requests.read() {
        let packet = EntityAnimationPacket::new(/* ... */);
        // Broadcast to network
    }
}
```

Repeat for: block updates, chat messages, movement sync

##### 3. Register core systems in game_loop.rs (1-2 hours)

```rust
fn build_tick(s: &mut Schedule) {
    // Core packet handlers (I/O layer)
    s.add_systems(ferrumc_core::animations::packet_handlers::handle_swing_arm_packets);
    s.add_systems(ferrumc_core::blocks::packet_handlers::handle_place_block_packets);
    
    // Core broadcasters (I/O layer)
    s.add_systems(ferrumc_core::animations::broadcast::broadcast_animations);
    s.add_systems(ferrumc_core::blocks::broadcast::broadcast_block_changes);
}
```

**Total Phase 2:** ~10-15 hours to implement core I/O layer

#### Phase 3: Create Plugins (Week 3-4)

Build plugins that use domain APIs.

##### 1. Create `animations` plugin (2-3 hours)

Using `animation-api`, implement game logic:
```rust
fn handle_player_swings(
    mut events: EventReader<PlayerSwingArmEvent>,
    world: &mut World,
) {
    for event in events.read() {
        world.play_animation(event.player, AnimationType::SwingMainArm);
    }
}
```

##### 2. Create `blocks` plugin (4-6 hours)

Using `block-api`, implement placement/breaking rules

##### 3. Create `chat` plugin (2-3 hours)

Using `chat-api`, implement formatting and broadcasting

##### 4. Create `movement` plugin (3-4 hours)

Using `movement-api`, implement validation and syncing

**Total Phase 3:** ~12-18 hours to create plugins

**GRAND TOTAL:** ~30-45 hours to complete full migration

---

### Path B: Implement New Features as Plugins (Priority Features)

**Goal:** Add critical missing features using the plugin system

**Priority order (from [FEATURES.md](FEATURES.md)):**

#### 1. Entity Tracking Plugin (2-3 days) 🔴 CRITICAL
**Why:** Players need to see each other!

**Create:** `src/lib/plugins/core/entity-tracking/`

**What it does:**
- Track all entities (players for now)
- Send spawn/despawn packets
- Sync position/rotation to nearby players
- Send entity metadata

**Uses core:**
- ECS (query entities)
- Networking (broadcast packets)

**Implementation hints:**
```rust
fn sync_entities_system(
    query: Query<(Entity, &Position, &Rotation, &PlayerIdentity)>,
    world: &World,
) {
    // For each player
    //   Find other players in range
    //   Send their position updates
}
```

#### 2. Health System Plugin (2-3 days) 🔴 CRITICAL
**Why:** Need survival mechanics!

**Create:** `src/lib/plugins/core/health/`

**What it does:**
- Health, food, saturation tracking
- Damage from fall, void, etc.
- Death and respawn
- Regeneration

**Uses core:**
- ECS (Health component)
- Networking (send health packets)

#### 3. Chat Broadcasting Plugin (1-2 days) 🔴 CRITICAL
**Why:** Communication between players!

**What it does:**
- Broadcast chat to all players
- Format with player names
- System messages

**Uses core:**
- Networking (send packets to all)
- Text formatting

#### 4. Item Entities Plugin (2-3 days) 🔴 CRITICAL
**Why:** Need items!

**Create:** `src/lib/plugins/core/item-entities/`

**What it does:**
- Spawn item entities when dropped
- Pickup mechanics
- Item entity physics

**Uses core:**
- ECS (ItemEntity component)
- World (collision detection)
- Networking (spawn packets)

#### 5. Combat Plugin (3-5 days) 🟡 HIGH
**Create:** `src/lib/plugins/gameplay/combat/`

**What it does:**
- Melee attack detection
- Damage calculation
- Knockback
- PvP/PvE

**Depends on:** health, entity-tracking

---

## 🛠️ Development Workflow

### For Any New Feature

```bash
# 1. Create plugin crate
mkdir -p src/lib/plugins/core/feature-name/src
cd src/lib/plugins/core/feature-name

# 2. Create Cargo.toml (use template above)

# 3. Create lib.rs with Plugin impl
cat > src/lib.rs << 'EOF'
use ferrumc_plugin_api::*;

#[derive(Default)]
pub struct FeaturePlugin;

impl Plugin for FeaturePlugin {
    fn name(&self) -> &'static str { "feature_name" }
    fn version(&self) -> &'static str { "1.0.0" }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        ctx.add_tick_system(my_system);
        ctx.register_event::<MyEvent>();
    }
}

fn my_system() {
    // Your logic
}

#[derive(bevy_ecs::prelude::Event)]
struct MyEvent;
EOF

# 4. Add to workspace (Cargo.toml root)
# Add to members array: "src/lib/plugins/core/feature-name"

# 5. Register plugin (src/bin/src/plugin_loader.rs)
# Add: registry.register::<ferrumc_plugin_feature_name::FeaturePlugin>();

# 6. Add to binary dependencies (src/bin/Cargo.toml)
# Add: ferrumc-plugin-feature-name = { path = "../lib/plugins/core/feature-name" }

# 7. Build and test
cargo build
cargo test --package ferrumc-plugin-feature-name
./target/debug/ferrumc

# 8. Check logs for:
# [INFO] Loading plugin: feature_name v1.0.0
```

---

## 📖 API Reference

### Plugin Trait

```rust
impl Plugin for MyPlugin {
    fn name(&self) -> &'static str;              // Required: unique name
    fn version(&self) -> &'static str;           // Required: semver
    fn author(&self) -> &'static str { .. }      // Optional
    fn description(&self) -> &'static str { .. } // Optional
    fn dependencies(&self) -> Vec<&'static str> { vec![] } // Optional
    fn build(&self, ctx: &mut PluginContext<'_>); // Required: setup
}
```

### PluginContext Methods

```rust
// Systems
ctx.add_tick_system(system);                     // Runs every tick
ctx.add_timed_system(name, period, system);      // Periodic (TODO: needs scheduler fix)

// Events
ctx.register_event::<MyEvent>();                 // Register event type

// Resources
ctx.insert_resource(MyResource::default());      // Global singleton

// Configuration
ctx.get_config::<T>("key");                      // From plugins.toml
ctx.state;                                       // Access GlobalState
ctx.world;                                       // Access ECS World
```

### WorldExt Helpers

```rust
use ferrumc_plugin_api::WorldExt;

// In systems with Res<GlobalStateResource>:
state.0.get_chunk(x, z, "overworld")?;           // Load chunk
state.0.set_block_at(x, y, z, block_id)?;        // Set block
state.0.get_block_at(x, y, z)?;                  // Get block
state.0.chunk_exists(x, z, "overworld");         // Check exists
```

### EntityExt Helpers

```rust
use ferrumc_plugin_api::EntityExt;

// In systems with access to World:
world.players_in_range(&pos, 100.0);             // Get nearby players
world.broadcast_packet(&packet);                 // Send to all
world.broadcast_packet_in_range(&packet, &pos, range);
world.broadcast_message("text");                 // TODO: needs packet impl
world.player_count();                            // Count players
```

### Common Events (ferrumc_plugin_api::events)

```rust
use ferrumc_plugin_api::events::*;

BlockPlacedEvent    // Player placed block
BlockBrokenEvent    // Player broke block
ChatMessageEvent    // Player sent chat
PlayerJoinEvent     // Player connected
PlayerLeaveEvent    // Player disconnected
PlayerMoveEvent     // Player moved
PlayerDamageEvent   // Player took damage
PlayerDeathEvent    // Player died
// ... and more
```

---

## 🎯 Immediate Next Actions

### Option 1: Migrate Existing Features (Recommended First)

**Start with the `blocks` plugin - it's simplest:**

```bash
# 1. Create the plugin
mkdir -p src/lib/plugins/core/blocks/src

# 2. Create Cargo.toml
cat > src/lib/plugins/core/blocks/Cargo.toml << 'EOF'
[package]
name = "ferrumc-plugin-blocks"
version = "0.1.0"
edition = "2021"

[dependencies]
ferrumc-plugin-api = { path = "../../../plugin-api" }
ferrumc-core = { workspace = true }
ferrumc-net = { workspace = true }
ferrumc-state = { workspace = true }
ferrumc-world = { workspace = true }
ferrumc-inventories = { workspace = true }
bevy_ecs = { workspace = true }
tracing = { workspace = true }
serde_json = { workspace = true }
once_cell = { workspace = true }

[lints]
workspace = true
EOF

# 3. Copy files from binary
cp src/bin/src/packet_handlers/play_packets/place_block.rs src/lib/plugins/core/blocks/src/
cp src/bin/src/packet_handlers/play_packets/player_action.rs src/lib/plugins/core/blocks/src/

# 4. Create lib.rs
cat > src/lib/plugins/core/blocks/src/lib.rs << 'EOF'
use ferrumc_plugin_api::*;

pub mod place_block;
pub mod player_action;

#[derive(Default)]
pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn name(&self) -> &'static str { "blocks" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn description(&self) -> &'static str {
        "Block placement and breaking mechanics"
    }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        ctx.add_tick_system(place_block::handle);
        ctx.add_tick_system(player_action::handle);
    }
}
EOF

# 5. Add to workspace Cargo.toml
# Add to members: "src/lib/plugins/core/blocks"

# 6. Add to binary Cargo.toml dependencies
# ferrumc-plugin-blocks = { path = "../lib/plugins/core/blocks" }

# 7. Register in plugin_loader.rs
# Add: registry.register::<ferrumc_plugin_blocks::BlocksPlugin>();

# 8. Remove old registration from game_loop.rs
# Remove: calls to register place_block/player_action handlers

# 9. Delete old files
# rm src/bin/src/packet_handlers/play_packets/place_block.rs
# rm src/bin/src/packet_handlers/play_packets/player_action.rs

# 10. Build and test
cargo build
./target/debug/ferrumc
```

**Verify:** Block placement/breaking still works

**Repeat for:** player-movement, chunk-management, chat, inventory, commands

---

### Option 2: Implement New Feature (High Impact)

**Create the `entity-tracking` plugin - most requested!**

```bash
# 1. Create plugin
mkdir -p src/lib/plugins/core/entity-tracking/src

# 2. Create Cargo.toml (similar to blocks example)

# 3. Create lib.rs
cat > src/lib/plugins/core/entity-tracking/src/lib.rs << 'EOF'
use ferrumc_plugin_api::*;
use ferrumc_plugin_api::events::*;
use ferrumc_core::transform::position::Position;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net::connection::StreamWriter;
use bevy_ecs::prelude::*;

#[derive(Default)]
pub struct EntityTrackingPlugin;

impl Plugin for EntityTrackingPlugin {
    fn name(&self) -> &'static str { "entity_tracking" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn description(&self) -> &'static str {
        "Makes players visible to each other"
    }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        // Register events
        ctx.register_event::<PlayerJoinEvent>();
        ctx.register_event::<PlayerLeaveEvent>();
        
        // Register systems
        ctx.add_tick_system(spawn_player_for_others);
        ctx.add_tick_system(sync_player_positions);
        ctx.add_tick_system(handle_player_join);
        ctx.add_tick_system(handle_player_leave);
    }
}

// System: When player joins, send spawn packets to others
fn handle_player_join(
    mut events: EventReader<PlayerJoinEvent>,
    query: Query<(&Position, &PlayerIdentity)>,
    writers: Query<&StreamWriter>,
) {
    for event in events.read() {
        // Get the new player's data
        let Ok((pos, identity)) = query.get(event.player) else { continue };
        
        // Send spawn packet to all other players
        for writer in writers.iter() {
            // TODO: Create SpawnPlayerPacket
            // writer.send_packet(&SpawnPlayerPacket { ... });
        }
    }
}

// System: Sync positions every tick
fn sync_player_positions(
    query: Query<(Entity, &Position, &PlayerIdentity)>,
    world: &World,
) {
    // For each player, send position updates to nearby players
    for (entity, pos, identity) in query.iter() {
        let nearby = world.players_in_range(pos, 128.0);
        
        for nearby_entity in nearby {
            if nearby_entity == entity { continue; } // Don't send to self
            
            // TODO: Send position update packet to nearby player
        }
    }
}

// More systems...
fn spawn_player_for_others() { /* TODO */ }
fn handle_player_leave(/* ... */) { /* TODO */ }
EOF

# 4-10. Same registration steps as above
```

**Impact:** Players can finally see each other! 🎉

---

## 📝 Step-by-Step Migration Checklist

Use this for **each plugin** you create:

### Pre-Migration
- [ ] Read [PLUGIN_VS_CORE.md](PLUGIN_VS_CORE.md) - Understand the distinction
- [ ] Read [PLUGIN_QUICKSTART.md](PLUGIN_QUICKSTART.md) - Understand the API
- [ ] Identify what code to move (from src/bin/src/)

### Create Plugin Structure
- [ ] Create `src/lib/plugins/core/<name>/src/`
- [ ] Create `Cargo.toml` with dependencies
- [ ] Create `lib.rs` with Plugin trait impl

### Move Code
- [ ] Copy system files from src/bin/
- [ ] Update imports (ferrumc_* crates)
- [ ] Create event definitions if needed
- [ ] Register systems in `Plugin::build()`

### Integration
- [ ] Add to workspace `Cargo.toml` members
- [ ] Add to `src/bin/Cargo.toml` dependencies
- [ ] Register in `src/bin/src/plugin_loader.rs`
- [ ] Remove old registration from `game_loop.rs`
- [ ] Delete old files from src/bin/

### Testing
- [ ] `cargo check` - Compiles
- [ ] `cargo clippy --package <plugin> -- -Dwarnings` - No warnings
- [ ] `cargo test --package <plugin>` - Tests pass
- [ ] `cargo build` - Full build works
- [ ] `./target/debug/ferrumc` - Server starts
- [ ] Test feature in-game

### Documentation
- [ ] Add plugin to [FEATURES.md](FEATURES.md)
- [ ] Update plugin list in this file
- [ ] Add configuration example to plugins.toml

---

## 🗺️ Plugin Dependencies

When creating plugins, respect dependencies:

```
No dependencies:
├── entity-tracking
├── health
└── chat

Depends on entity-tracking:
├── combat (needs to target entities)
└── player-movement (needs to sync to others)

Depends on health:
├── combat (deals damage)
└── food (affects health)

Depends on chat:
└── commands (sends feedback)
```

Declare dependencies in plugin:
```rust
fn dependencies(&self) -> Vec<&'static str> {
    vec!["entity_tracking", "health"]
}
```

---

## ⚠️ Common Pitfalls

### 1. Don't Put Infrastructure in Plugins
❌ **Wrong:** Create a "networking plugin" for TCP handling  
✅ **Right:** TCP is core, use it in gameplay plugins

### 2. Don't Put Gameplay in Core
❌ **Wrong:** Add combat system to `src/lib/core/`  
✅ **Right:** Combat is gameplay, make it a plugin

### 3. Remember to Register
❌ **Wrong:** Create plugin but forget to register in plugin_loader.rs  
✅ **Right:** Always add `registry.register::<YourPlugin>()`

### 4. Test Compilation Early
❌ **Wrong:** Write 500 lines then try to compile  
✅ **Right:** Compile after every 50-100 lines

---

## 🎓 Learning Resources

### For Understanding the Codebase
1. [ARCHITECTURE.md](ARCHITECTURE.md) - How everything works
2. [PLUGIN_VS_CORE.md](PLUGIN_VS_CORE.md) - What goes where
3. Read existing code in `src/lib/plugins/examples/hello/`

### For Implementing Features
1. [FEATURES.md](FEATURES.md) - What needs to be built
2. [PLUGIN_QUICKSTART.md](PLUGIN_QUICKSTART.md) - How to build it
3. Minecraft Protocol: https://minecraft.wiki/w/Java_Edition_protocol

### For Code Style
1. [AGENTS.md](AGENTS.md) - Conventions and commands
2. [CONTRIBUTING.md](CONTRIBUTING.md) - PR guidelines

---

## 🏁 Success Criteria

You'll know you're doing it right when:

1. ✅ `src/bin/src/` has <10 files (just bootstrap)
2. ✅ Each feature is in its own plugin crate
3. ✅ Plugins declare dependencies correctly
4. ✅ Binary's `game_loop.rs` is <200 lines
5. ✅ All gameplay logic is in `src/lib/plugins/`
6. ✅ Core infrastructure is in `src/lib/` (not plugins/)
7. ✅ `cargo build` works
8. ✅ Server starts and logs show plugins loading

---

## 📞 Getting Help

- **Discord:** https://discord.gg/qT5J8EMjwk
- **Read the docs** in this order: AGENTS → ARCHITECTURE → PLUGIN_VS_CORE
- **Check examples:** `src/lib/plugins/examples/hello/`

---

## 🎯 TL;DR - Start Here

1. **Read [PLUGIN_VS_CORE.md](PLUGIN_VS_CORE.md)** (10 min) - Understand the philosophy
2. **Read [PLUGIN_QUICKSTART.md](PLUGIN_QUICKSTART.md)** (15 min) - Learn the API
3. **Create `entity-tracking` plugin** (2-3 days) - Highest priority feature
4. **Or migrate `blocks` plugin** (4-6 hours) - Simplest migration to learn the pattern

---

## 📊 Current Plugin Status

### Implemented Plugins
- ✅ `hello` (example) - Demonstrates the system

### Plugins to Create (Migration)
- ⏳ `player-movement` - Move position/rotation handlers
- ⏳ `blocks` - Move block placement/breaking
- ⏳ `chunk-management` - Move chunk loading logic
- ⏳ `chat` - Move chat broadcasting
- ⏳ `inventory` - Wrap inventory lib + handlers
- ⏳ `commands` - Wrap commands lib + handlers

### Plugins to Create (New Features)
- ⏳ `entity-tracking` - **Most important!** Make players visible
- ⏳ `health` - Health, food, damage
- ⏳ `combat` - PvP/PvE mechanics
- ⏳ `item-entities` - Drop/pickup items
- ⏳ `crafting` - Crafting system
- ⏳ `mobs` - Mob AI and spawning

---

**The foundation is ready. Time to build Minecraft! 🚀**

**Start with:** Either migrate `blocks` (to learn the pattern) or create `entity-tracking` (highest impact)

**Remember:** Core = infrastructure, Plugins = gameplay. When in doubt, check [PLUGIN_VS_CORE.md](PLUGIN_VS_CORE.md).
