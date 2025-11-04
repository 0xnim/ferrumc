# Core Infrastructure vs Gameplay Plugins

**Purpose:** Clear separation between core server infrastructure and gameplay features

**See also:** [PLUGIN_ARCHITECTURE.md](PLUGIN_ARCHITECTURE.md) for detailed architecture guide

---

## The Distinction

### CORE (Infrastructure)
**Location:** `src/lib/core/`, `src/lib/net/`, `src/lib/world/`, etc.

Core infrastructure that makes the server work:
- ✅ Networking (TCP, packets, encryption, compression)
- ✅ World storage (database, chunk persistence)
- ✅ ECS system (Bevy integration)
- ✅ Connection handling (accept, handshake, disconnect)
- ✅ Packet encoding/decoding (I/O layer)
- ✅ NBT/Anvil parsing
- ✅ World generation (noise, biomes)
- ✅ Configuration system
- ✅ Logging, profiling, utilities

**Core handles ALL I/O:** Packets in, packets out, database reads/writes.

**Question:** "Could the server boot without this?" → **No** = It's core

### DOMAIN APIs (The Bridge)
**Location:** `src/lib/apis/`

Domain-specific API crates that sit between core and plugins:
- 🔌 Events (high-level gameplay events, not raw packets)
- 🔌 Traits (APIs that plugins use: BlockAPI, AnimationAPI, etc.)
- 🔌 Types (domain models: AnimationType, BlockPos, etc.)

**APIs define the contract** between core infrastructure and plugin logic.

### PLUGINS (Gameplay Logic)
**Location:** `src/lib/plugins/`

Gameplay features built using domain APIs:
- 🎮 Player movement **logic** (validation rules)
- 🎮 Block placement **mechanics** (collision checks, placement rules)
- 🎮 Block breaking **mechanics** (tool checks, drop logic)
- 🎮 Commands like `/gamemode`, `/tp`, `/give`
- 🎮 Chat formatting and broadcasting
- 🎮 Combat system
- 🎮 Health/hunger mechanics
- 🎮 Entity AI and spawning
- 🎮 Inventory interactions
- 🎮 Crafting recipes

**Plugins contain ONLY logic:** Game rules, validation, mechanics. No I/O.

**Question:** "Could the server boot without this?" → **Yes** = It's a plugin

---

## Current State Analysis

### ✅ What's Correctly in Core

**`src/lib/` libraries (Keep as-is):**
```
✅ adapters/nbt/          # NBT parsing - core infrastructure
✅ adapters/anvil/        # Anvil parsing - core infrastructure
✅ core/                  # ECS components - core infrastructure
✅ net/                   # Networking layer - core infrastructure
✅ storage/               # Database - core infrastructure
✅ world/                 # Chunk format, world struct - core infrastructure
✅ world_gen/             # Terrain generation - core infrastructure
✅ text/                  # Text components - core utility
✅ utils/                 # Utilities - core infrastructure
✅ scheduler/             # ECS scheduler - core infrastructure
✅ config/                # Configuration - core infrastructure
✅ registry/              # Game data - core infrastructure
✅ plugin-api/            # Plugin system - core infrastructure
```

**These are the "engine" - they stay as libraries.**

---

### 🎮 What Should Be Plugins

**Currently in `src/bin/src/packet_handlers/` and `src/bin/src/systems/`:**

These implement **gameplay mechanics** and should become plugins:

#### Plugin 1: `player-movement` 🎮
**What it does:** Handle player movement mechanics

**Move from binary:**
- `packet_handlers/play_packets/set_player_position.rs`
- `packet_handlers/play_packets/set_player_rotation.rs`
- `packet_handlers/play_packets/set_player_position_and_rotation.rs`
- `packet_handlers/play_packets/player_command.rs` (sprint, sneak)
- `packet_handlers/play_packets/swing_arm.rs`
- `packet_handlers/player/head_rot.rs`

**What it provides:**
- Movement validation
- Position syncing to other players (when entity tracking exists)
- Sprint/sneak mechanics
- Animation handling

**Uses core:** networking (to receive packets), ECS (to store position)

---

#### Plugin 2: `blocks` 🎮
**What it does:** Block placement and breaking mechanics

**Move from binary:**
- `packet_handlers/play_packets/place_block.rs`
- `packet_handlers/play_packets/player_action.rs`

**What it provides:**
- Block placement rules
- Collision detection
- Block breaking logic
- Block update broadcasting

**Uses core:** world (to load/save chunks), networking (to send updates)

---

#### Plugin 3: `commands` 🎮
**What it does:** Implement Minecraft commands

**Wrap existing:**
- `src/lib/commands/` (keep as lib, used by plugin)
- `src/lib/default_commands/` (move into plugin)

**Move from binary:**
- `packet_handlers/play_packets/command.rs`
- `packet_handlers/play_packets/command_suggestions.rs`

**What it provides:**
- `/gamemode`, `/echo`, etc.
- Command execution
- Tab completion

**Uses core:** commands lib (infrastructure), networking

---

#### Plugin 4: `chat` 🎮
**What it does:** Chat message handling and broadcasting

**Move from binary:**
- `packet_handlers/play_packets/chat_message.rs`
- `systems/mq.rs` (message queue)
- `systems/system_messages/`

**What it provides:**
- Chat broadcasting to players
- Chat formatting
- System messages

**Uses core:** networking, text formatting

---

#### Plugin 5: `inventory` 🎮
**What it does:** Inventory management mechanics

**Wrap existing:**
- `src/lib/inventories/` (keep as lib, used by plugin)

**Move from binary:**
- `packet_handlers/play_packets/set_creative_mode_slot.rs`
- `packet_handlers/play_packets/set_held_item.rs`
- `packet_handlers/player/send_inventory_updates.rs`

**What it provides:**
- Inventory slot updates
- Creative mode inventory
- Hotbar selection

**Uses core:** networking, inventories lib

---

### 🔧 What Stays in Binary

**`src/bin/src/` (Bootstrap only - ~500 lines):**

```
✅ main.rs                   # Entry point, CLI parsing
✅ cli.rs                    # CLI definitions
✅ game_loop.rs              # ECS scheduler, tick loop
✅ plugin_loader.rs          # Plugin initialization
✅ errors.rs                 # Binary errors
✅ systems/shutdown_systems/ # Graceful shutdown
```

**These files orchestrate the server but don't implement gameplay.**

---

## The Clean Separation

### Core Infrastructure (src/lib/)
**Handles all I/O operations:**

```rust
// Example: Core converts raw packets into high-level events
pub fn handle_swing_arm_packets(
    packets: Res<SwingArmPacketReceiver>,  // ← Raw packet from network
    mut events: EventWriter<PlayerSwingArmEvent>,  // ← High-level event
) {
    for (packet, entity) in packets.0.try_iter() {
        events.send(PlayerSwingArmEvent {
            player: entity,
            hand: if packet.hand == 0 { Hand::Main } else { Hand::Off },
        });
    }
}
```

### Domain APIs (src/lib/apis/)
**Define the contract between core and plugins:**

```rust
// Example: Animation API provides high-level interface
pub trait AnimationAPI {
    fn play_animation(&mut self, entity: Entity, animation: AnimationType);
}

#[derive(Event)]
pub struct PlayerSwingArmEvent {
    pub player: Entity,
    pub hand: Hand,
}
```

### Plugins (src/lib/plugins/)
**Implement game logic using APIs:**

```rust
// Example: Animation plugin contains logic, uses API
impl Plugin for AnimationsPlugin {
    fn build(&self, ctx: &mut PluginContext<'_>) {
        ctx.add_tick_system(handle_player_swings);
    }
}

fn handle_player_swings(
    mut events: EventReader<PlayerSwingArmEvent>,  // ← High-level event
    world: &mut World,
) {
    for event in events.read() {
        // Game logic: should we show the animation?
        world.play_animation(event.player, AnimationType::SwingMainArm);  // ← Use API
    }
}
```

---

## What This Means

### Core Provides:
- 📦 Packet I/O (receive packets, send packets)
- 📡 Network infrastructure (TCP, encryption, compression)
- 🗄️ Database operations (read, write, cache)
- 🌍 World storage (chunk format, persistence)
- 🔌 ECS framework (Bevy integration)
- 🔄 Event routing (packet → event → packet)

### Domain APIs Provide:
- 📋 High-level events (PlayerSwingArmEvent, BlockPlaceAttempt)
- 🎨 Clean interfaces (AnimationAPI, BlockAPI, ChatAPI)
- 📦 Domain types (AnimationType, BlockPos, Hand)
- 🔒 Abstraction layer (plugins never see raw packets)

### Plugins Provide:
- 🎮 Game logic (when should animation play?)
- 🎮 Validation rules (can block be placed here?)
- 🎮 Mechanics (how does combat work?)
- 🎮 Features (commands, chat formatting)
- 🎮 Behavior (mob AI, entity spawning)

---

## Concrete Example: Animation System

### ❌ Wrong: Plugin Handles Packets Directly

```rust
// NO - Plugin should not see raw packets
fn handle_swing_arm(
    packets: Res<SwingArmPacketReceiver>,  // ← Raw packet access
    conn_query: Query<&StreamWriter>,       // ← Raw network access
) {
    for (packet, entity) in packets.0.try_iter() {
        // Plugin should not deal with packet encoding/broadcasting
    }
}
```

### ✅ Correct: Three-Layer Architecture

**1. Core (Packet → Event):**
```rust
// src/lib/core/src/animations/packet_handlers.rs
pub fn handle_swing_arm_packets(
    packets: Res<SwingArmPacketReceiver>,  // ← I/O layer
    mut events: EventWriter<PlayerSwingArmEvent>,  // ← High-level event
) {
    for (packet, entity) in packets.0.try_iter() {
        events.send(PlayerSwingArmEvent {
            player: entity,
            hand: if packet.hand == 0 { Hand::Main } else { Hand::Off },
        });
    }
}
```

**2. Domain API (Events + Traits):**
```rust
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
}

// src/lib/apis/animation-api/src/traits.rs
pub trait AnimationAPI {
    fn play_animation(&mut self, entity: Entity, animation: AnimationType);
}
```

**3. Plugin (Game Logic):**
```rust
// src/lib/plugins/core/animations/src/lib.rs
fn handle_player_swings(
    mut events: EventReader<PlayerSwingArmEvent>,  // ← Listen to event
    world: &mut World,
) {
    for event in events.read() {
        // Game logic only
        let animation = match event.hand {
            Hand::Main => AnimationType::SwingMainArm,
            Hand::Off => AnimationType::SwingOffhand,
        };
        world.play_animation(event.player, animation);  // ← Use API
    }
}
```

**4. Core (Event → Packet):**
```rust
// src/lib/core/src/animations/broadcast.rs
pub fn broadcast_animations(
    mut requests: EventReader<PlayAnimationRequest>,
    conn_query: Query<&StreamWriter>,  // ← I/O layer
) {
    for req in requests.read() {
        let packet = EntityAnimationPacket::new(/* ... */);
        // Broadcast to network
    }
}
```

**Flow:** `Packet → Event → Plugin Logic → Request Event → Packet`

**Core handles I/O. Plugin handles logic. API defines the contract.**

---

## Migration Summary

### From Binary → Plugins (Gameplay Features)

| Current Location | Move To Plugin | What It Does | Lines |
|------------------|----------------|--------------|-------|
| `packet_handlers/play_packets/set_player_position*.rs` | `player-movement` | Movement mechanics | ~200 |
| `packet_handlers/play_packets/place_block.rs` | `blocks` | Block placement | ~170 |
| `packet_handlers/play_packets/player_action.rs` | `blocks` | Block breaking | ~80 |
| `packet_handlers/play_packets/chat_message.rs` | `chat` | Chat handling | ~50 |
| `packet_handlers/play_packets/command*.rs` | `commands` | Command execution | ~150 |
| `packet_handlers/play_packets/set_*_slot.rs` | `inventory` | Inventory mechanics | ~100 |
| `systems/mq.rs` | `chat` | Message broadcasting | ~50 |
| `systems/cross_chunk_boundary.rs` | `chunk-management` | Chunk loading logic | ~100 |
| `systems/send_chunks.rs` | `chunk-management` | Chunk sending logic | ~150 |
| `chunk_sending.rs` | `chunk-management` | Chunk transmission | ~100 |

**Total:** ~1,150 lines of gameplay code → 5-6 plugins

### Keep in Core (Infrastructure)

| Location | Keep Because | What It Does |
|----------|--------------|--------------|
| `src/lib/world/` | Infrastructure | Chunk format, database, storage mechanism |
| `src/lib/net/` | Infrastructure | TCP, packets, encryption, codec |
| `src/lib/storage/` | Infrastructure | Database backend |
| `src/lib/world_gen/` | Infrastructure | Terrain generation algorithm |
| `src/lib/core/` | Infrastructure | ECS components, base types |

### Keep in Binary (Bootstrap)

| File | Keep Because | What It Does |
|------|--------------|--------------|
| `main.rs` | Bootstrap | Entry point |
| `game_loop.rs` | Bootstrap | ECS scheduler |
| `plugin_loader.rs` | Bootstrap | Initialize plugins |
| `shutdown_systems/` | Bootstrap | Graceful shutdown |

---

## Recommended Plugin Structure

```
src/lib/
├── core/                  # ✅ ECS components, base types
├── net/                   # ✅ Networking infrastructure
├── world/                 # ✅ World/chunk storage
├── world_gen/             # ✅ Terrain generation
├── storage/               # ✅ Database
├── utils/                 # ✅ Utilities
├── config/                # ✅ Configuration
│
├── apis/                  # 🔌 DOMAIN API LAYER (NEW!)
│   ├── animation-api/     # Animation events, traits, types
│   ├── block-api/         # Block events, traits, types
│   ├── chat-api/          # Chat events, traits, types
│   ├── entity-api/        # Entity tracking events, traits
│   ├── inventory-api/     # Inventory events, traits
│   ├── movement-api/      # Movement events, traits
│   └── combat-api/        # Combat events, traits (future)
│
├── plugin-api/            # ✅ Base plugin system
│   ├── src/
│   │   ├── lib.rs         # Plugin trait
│   │   ├── context.rs     # PluginContext
│   │   └── macros.rs      # Helper macros
│
└── plugins/               # 🎮 GAMEPLAY PLUGINS
    ├── core/              # Essential gameplay (can't disable)
    │   ├── animations/    # Animation logic
    │   ├── blocks/        # Block placement/breaking rules
    │   ├── chat/          # Chat formatting and broadcasting
    │   ├── movement/      # Movement validation
    │   └── inventory/     # Inventory management
    │
    └── gameplay/          # Optional features (can disable)
        ├── combat/        # PvP/PvE mechanics (future)
        ├── health/        # Health/hunger mechanics (future)
        └── entities/      # Mob AI (future)

src/bin/
└── src/
    ├── main.rs            # ✅ Entry point
    ├── game_loop.rs       # ✅ ECS scheduler, register core systems
    ├── plugin_loader.rs   # ✅ Load and build plugins
    └── ...                # ✅ Bootstrap only
```

**Key Addition:** `src/lib/apis/` directory contains domain-specific API crates.

---

## Clearer Examples

### Example: Block Placement

**Core Infrastructure (src/lib/world/):**
```rust
// Provides the MECHANISM to modify blocks
impl Chunk {
    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: BlockStateId) {
        // Low-level block setting, palette management
    }
}
```

**Gameplay Plugin (src/lib/plugins/core/blocks/):**
```rust
// Implements the GAMEPLAY RULES
fn handle_place_block(event: PlaceBlockPacket, world: &World) {
    // 1. Check if player can place here (collision detection)
    // 2. Check if holding correct item
    // 3. Use core's set_block() to modify world
    // 4. Send block update to nearby players
    // 5. Play sound effects
}
```

**Binary just loads the plugin:**
```rust
registry.register::<BlocksPlugin>();
```

---

## What Should Move to Plugins

### 🎮 Gameplay Features Currently in Binary

These **implement game rules** and should be plugins:

1. **Player Movement Plugin**
   - `set_player_position*.rs` - Movement **mechanics**
   - `player_command.rs` - Sprint/sneak **mechanics**
   - Uses core: networking (packets), ECS (Position component)

2. **Blocks Plugin**
   - `place_block.rs` - Placement **rules**
   - `player_action.rs` - Breaking **rules**
   - Uses core: world (set_block), networking (send updates)

3. **Chunk Management Plugin**
   - `cross_chunk_boundary.rs` - When to load chunks (**game logic**)
   - `send_chunks.rs` - Which chunks to send (**game logic**)
   - `chunk_sending.rs` - Formatting chunk packets (**game logic**)
   - Uses core: world (load_chunk), networking (send packets)

4. **Inventory Plugin**
   - `set_creative_mode_slot.rs` - Creative inventory **rules**
   - `set_held_item.rs` - Hotbar selection **mechanics**
   - Uses core: inventories lib, networking

5. **Chat Plugin**
   - `chat_message.rs` - Chat **handling**
   - `mq.rs` - Broadcast **logic**
   - Uses core: networking, text formatting

6. **Commands Plugin**
   - `command.rs` - Command **execution**
   - `command_suggestions.rs` - Tab completion **logic**
   - Uses core: commands lib (infrastructure)

---

## What Stays in Core

### ✅ Core Infrastructure Libraries

**These are NOT plugins - they're fundamental:**

```
src/lib/
├── net/                   # Networking: TCP, packets, encoding
│   ├── connection.rs      # Connection handling
│   ├── packets/           # Packet definitions
│   ├── crates/codec/      # Packet encoding
│   └── crates/encryption/ # Encryption
│
├── world/                 # World: chunks, storage, persistence
│   ├── chunk_format.rs    # Chunk data structure
│   ├── db_functions.rs    # Database operations
│   ├── edits.rs           # Block modification primitives
│   └── lib.rs             # World struct
│
├── storage/               # Database: LMDB backend
│   └── lmdb.rs            # Database wrapper
│
├── world_gen/             # Terrain: noise, biome generation
│   └── biomes/            # Biome generators
│
├── core/                  # ECS: components, state
│   ├── transform/         # Position, Rotation components
│   ├── identity/          # PlayerIdentity component
│   └── chunks/            # Chunk tracking components
│
└── adapters/              # Parsing: NBT, Anvil
    ├── nbt/               # NBT serialization
    └── anvil/             # Region file parsing
```

**Why these stay:** They're **mechanisms** that plugins use, not gameplay.

---

## Crystal Clear Examples

### Networking Core vs Plugin

**Core (src/lib/net/):**
```rust
// Provides: "How to send a packet"
impl StreamWriter {
    pub fn send_packet(&self, packet: impl NetEncode) {
        // Low-level: encoding, compression, TCP write
    }
}
```

**Plugin (src/lib/plugins/core/player-movement/):**
```rust
// Uses core to implement: "When player moves, what happens"
fn handle_player_movement(packet: SetPlayerPositionPacket, writer: &StreamWriter) {
    // Game logic: validate movement, check for cheating
    // Use core's send_packet() to notify other players
}
```

---

### World Core vs Plugin

**Core (src/lib/world/):**
```rust
// Provides: "How to load a chunk from database"
impl World {
    pub fn load_chunk(&self, x: i32, z: i32) -> Result<Chunk> {
        // Low-level: cache lookup, database read, decompression
    }
}
```

**Plugin (src/lib/plugins/core/chunk-management/):**
```rust
// Uses core to implement: "Which chunks should be loaded for this player"
fn cross_chunk_boundary(player_pos: &Position, world: &World, writer: &StreamWriter) {
    // Game logic: calculate view distance, load chunks in spiral
    // Use core's load_chunk() + send_packet()
}
```

---

## The Key Insight

**If it's about HOW the server works → Core**  
**If it's about WHAT the game does → Plugin**

### Core = Server Engine
- Packet protocol
- Database storage
- Chunk format
- ECS framework
- Connection handling

### Plugins = Minecraft Gameplay
- Block placement rules
- Player movement validation
- Chat broadcasting logic
- Command implementations
- Combat mechanics

---

## Migration Priority

### Must Move (Most Important)

1. **`player-movement`** - Currently 200 lines in binary
2. **`blocks`** - Currently 250 lines in binary
3. **`chunk-management`** - Currently 350 lines in binary
4. **`chat`** - Currently 100 lines in binary
5. **`commands`** - Currently 150 lines in binary
6. **`inventory`** - Currently 100 lines in binary

**Total:** ~1,150 lines of gameplay code to extract

### Can Stay in Binary (Infrastructure)

- **Connection systems** - These might stay in binary as they're close to infrastructure
  - `new_connections.rs` - Spawning player entities
  - `connection_killer.rs` - Cleanup
  - `keep_alive_system.rs` - Network health

- **Utility systems**
  - `world_sync.rs` - Could become plugin, but it's infrastructure-ish
  - `player_count_update.rs` - Could become plugin
  - `lan_pinger.rs` - Could become plugin

---

## After Migration

### Binary (src/bin/)
```
main.rs          - Parse CLI, start server
game_loop.rs     - ECS tick loop, scheduler
plugin_loader.rs - Initialize plugins
```
**~500 lines total**

### Core (src/lib/core, net, world, etc.)
```
Networking infrastructure
World storage infrastructure  
Chunk format and persistence
ECS components
Parsers (NBT, Anvil)
Utilities
```
**No change - already clean**

### Plugins (src/lib/plugins/core/)
```
player-movement/     - Movement mechanics
blocks/              - Block interaction mechanics
chunk-management/    - Chunk loading logic
chat/                - Chat broadcasting
commands/            - Command execution
inventory/           - Inventory mechanics
```
**~1,150 lines organized into features**

---

## Summary

**You're right!** The plugin system is an **organizational tool** for FerrumC's own features, not for external developers.

**The division:**
- **Core** = Server infrastructure (networking, database, world, ECS)
- **Plugins** = Minecraft gameplay features (movement, blocks, chat, commands)
- **Binary** = Bootstrap (just loads core and plugins)

**What needs to move:** ~1,150 lines of gameplay code from `src/bin/src/packet_handlers/` and `src/bin/src/systems/` into ~6 feature plugins.

**Core infrastructure stays exactly where it is** - it's already well-organized in `src/lib/`.

This gives you a clean separation: **infrastructure (core) vs features (plugins)**.
