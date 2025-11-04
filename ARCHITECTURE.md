# FerrumC Architecture Documentation

**Last Updated:** November 4, 2025  
**Version:** 1.21.8 Minecraft Protocol  
**Repository:** https://github.com/ferrumc-rs/ferrumc

---

## Table of Contents

1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [Core Architecture](#core-architecture)
4. [Detailed System Documentation](#detailed-system-documentation)
5. [Data Flow](#data-flow)
6. [Performance Optimizations](#performance-optimizations)
7. [Development Reference](#development-reference)

---

## Overview

### What is FerrumC?

FerrumC is a **Minecraft 1.21.8 server implementation** written entirely in Rust from the ground up. It leverages Rust's safety guarantees, zero-cost abstractions, and powerful concurrency primitives to achieve:

- **High Performance**: Custom NBT/Anvil parsers, SIMD optimizations, minimal I/O lag
- **Memory Efficiency**: Designed to use minimal memory while providing full server features
- **Full Multithreading**: Unlike vanilla's single main thread, FerrumC utilizes all CPU cores
- **Vanilla Client Compatibility**: Works with unmodified Minecraft 1.21.8 clients

### Design Philosophy

1. **Not just faster, but better**: Improves on vanilla in setup, configurability, and extensibility
2. **Performance-first**: Willing to use unsafe Rust and custom implementations when needed
3. **Ease of use**: Target audience is users running servers for friends/family
4. **No compromises**: Custom NBT/Anvil libraries using experimental APIs and SIMD

---

## Project Structure

### Workspace Layout

FerrumC uses a **Cargo workspace** with 40+ crates organized by functional domain:

```
ferrumc/
├── src/
│   ├── bin/                      # Main binary application
│   │   ├── src/
│   │   │   ├── main.rs          # Entry point, initialization
│   │   │   ├── game_loop.rs     # ECS scheduler, tick loop
│   │   │   ├── systems/         # Core game systems
│   │   │   └── packet_handlers/ # Network packet handlers
│   │   └── Cargo.toml
│   │
│   ├── lib/                      # Library crates providing business logic
│   │   ├── adapters/            # Data format parsers
│   │   │   ├── nbt/            # NBT serialization/deserialization
│   │   │   └── anvil/          # Anvil region file parser
│   │   │
│   │   ├── core/                # Core ECS components and logic
│   │   │   ├── identity/       # Player identity (UUID, username)
│   │   │   ├── transform/      # Position, rotation, grounding
│   │   │   ├── chunks/         # Chunk loading state
│   │   │   └── state/          # Server state management
│   │   │
│   │   ├── net/                 # Networking layer
│   │   │   ├── crates/codec/   # Packet encoding/decoding
│   │   │   ├── crates/encryption/ # Packet encryption
│   │   │   ├── connection.rs   # Connection handling
│   │   │   └── packets/        # Packet definitions
│   │   │
│   │   ├── world/               # World management
│   │   │   ├── chunk_format.rs # Chunk data structures
│   │   │   ├── db_functions.rs # Chunk persistence
│   │   │   └── importing.rs    # World importing from vanilla
│   │   │
│   │   ├── world_gen/           # Terrain generation
│   │   │   ├── biomes/         # Biome-specific generators
│   │   │   └── lib.rs          # Noise generation
│   │   │
│   │   ├── storage/             # K/V database backend
│   │   │   ├── lmdb.rs         # LMDB/heed wrapper
│   │   │   └── compressors/    # Compression (Gzip, Zstd, etc.)
│   │   │
│   │   ├── commands/            # Command system
│   │   │   ├── infrastructure.rs # Command registration
│   │   │   ├── graph/          # Command tree structure
│   │   │   └── arg/            # Argument parsers
│   │   │
│   │   ├── plugins/             # Plugin interface
│   │   ├── inventories/         # Inventory management
│   │   ├── registry/            # Game registry (blocks, items, etc.)
│   │   ├── scheduler/           # Timed schedule system
│   │   ├── text/                # Text component formatting
│   │   │
│   │   └── utils/               # Utilities
│   │       ├── general_purpose/ # Common utilities
│   │       ├── logging/        # Logging setup
│   │       ├── profiling/      # Performance profiling
│   │       └── threadpool/     # Thread pool for async tasks
│   │
│   └── tests/                   # Integration tests
│
├── configs/                     # Configuration files
├── world/                       # World data
├── Cargo.toml                   # Workspace definition
├── AGENTS.md                    # AI development guide
├── README.md                    # Project overview
├── CONTRIBUTING.md              # Contribution guidelines
└── ARCHITECTURE.md              # This file
```

### Workspace Dependencies

All dependencies are centralized in the workspace `Cargo.toml`:

- **Async**: Tokio 1.48 (networking, I/O)
- **ECS**: Bevy ECS 0.16.1 (entity-component system)
- **Database**: Heed 0.22.0 (LMDB K/V store)
- **Concurrency**: parking_lot, rayon, crossbeam
- **Serialization**: serde, craftflow-nbt, bitcode
- **Compression**: flate2, lzzzz, yazi
- **Logging**: tracing, tracing-subscriber

---

## Core Architecture

### 1. Entity Component System (ECS)

FerrumC uses **Bevy ECS** for lockless, data-oriented concurrency.

#### Key Components

| Component | Location | Description |
|-----------|----------|-------------|
| `PlayerIdentity` | `core/identity/player_identity.rs` | UUID, username, short UUID |
| `Position` | `core/transform/position.rs` | x, y, z coordinates |
| `Rotation` | `core/transform/rotation.rs` | Yaw, pitch angles |
| `OnGround` | `core/transform/grounded.rs` | Ground collision state |
| `ChunkReceiver` | `core/chunks/chunk_receiver.rs` | Chunk loading state machine |
| `Inventory` | `inventories/inventory.rs` | Player inventory slots |
| `StreamWriter` | `net/connection.rs` | Async packet writer |

#### System Registration

Systems are registered in `game_loop.rs` during scheduler initialization:

```rust
// Tick schedule (runs at configured TPS)
register_packet_handlers(schedule);     // Handle incoming packets
register_player_systems(schedule);      // Update player state
register_command_systems(schedule);     // Execute commands
register_game_systems(schedule);        // Core game logic
```

#### Events

| Event | File | Description |
|-------|------|-------------|
| `TransformEvent` | `net/packets/packet_events.rs` | Position/rotation changes |
| `CrossChunkBoundaryEvent` | `core/chunks/` | Player changed chunks |
| `CommandDispatchEvent` | `commands/events.rs` | Command execution request |
| `ResolvedCommandDispatchEvent` | `commands/events.rs` | Command parsed and ready |

#### Resources

Global resources registered in the ECS World:

- `GlobalStateResource`: Shared server state (world, threadpool, etc.)
- `NewConnectionRecv`: Channel receiving new TCP connections
- `PlayerCountUpdateCooldown`: Cooldown tracker for status updates
- `WorldSyncTracker`: Tracks last world sync time

### 2. Game Loop & Scheduler

**File:** `src/bin/src/game_loop.rs`

The game loop uses a **timed scheduler** that runs multiple schedules at different frequencies:

#### Schedules

| Schedule | Period | Behavior | Systems |
|----------|--------|----------|---------|
| **Tick** | 1/TPS (configurable) | Burst (catch-up) | Packet handling, player updates, commands, game logic |
| **World Sync** | 15 seconds | Skip | Flush chunks to disk |
| **Player Count** | 10 seconds | Skip | Update server list player count |
| **Keepalive** | 1 second | Skip | Send keepalive packets to clients |
| **Plugin Schedules** | Varies | Configurable | Plugin-registered systems |

#### Missed Tick Behaviors

- **Burst**: Runs missed ticks back-to-back (up to max catch-up limit)
- **Skip**: Skips missed ticks entirely
- **Delay**: Adjusts next run time based on how late the tick was

#### Main Loop Pseudocode

```rust
while !shutdown {
    // Check all schedules for which are due
    if let Some(due_schedule) = scheduler.peek_next_due() {
        if due_schedule.time <= now {
            // Run the schedule
            due_schedule.schedule.run(&mut ecs_world);
            scheduler.after_run(due_schedule);
        } else {
            // Sleep until next schedule is due
            scheduler.park_until_next_due();
        }
    }
}
```

### 3. Networking

**File:** `src/lib/net/src/connection.rs`

Networking uses **Tokio async runtime** with a hybrid threading model:

#### Connection Flow

1. **TCP Acceptor Thread** (`tcp_conn_acceptor` in `game_loop.rs`):
   - Runs in dedicated thread with Tokio current-thread runtime
   - Accepts incoming TCP connections
   - Spawns `handle_connection` task for each client

2. **Connection Handler** (`handle_connection`):
   - Performs handshake with timeout (10 seconds max)
   - Creates `StreamWriter` for async packet writing
   - Sends `NewConnection` to ECS via crossbeam channel
   - Enters packet receive loop

3. **Packet Processing**:
   ```
   TCP Stream → PacketSkeleton::new() → handle_packet() → Channel → ECS System
   ```

4. **Packet Sending**:
   ```
   ECS System → StreamWriter.send() → Tokio channel → Async write task
   ```

#### StreamWriter

The `StreamWriter` component manages asynchronous packet writes:

- **Unbounded channel** buffers outgoing packets
- **Background Tokio task** writes packets to socket
- **Automatic disconnection** on write errors
- **Compression** can be toggled dynamically

#### Packet Registration

**File:** `src/lib/derive_macros/src/net/packets/mod.rs`

The `setup_packet_handling!` macro automatically generates:

1. `handle_packet()` function - Dispatches packets to channels by ID
2. `PacketSender` struct - Contains crossbeam channels for each packet type
3. `create_packet_senders()` - Initializes channels and registers in ECS

Packet handlers are registered in `register_packet_handlers()`:

```rust
schedule.add_systems((
    keep_alive::keep_alive,
    set_player_position::handle_set_player_position,
    command::handle_command,
    // ... 15+ more handlers
));
```

### 4. World Management

**Files:** `src/lib/world/`, `src/lib/storage/`

World data uses a **two-tier persistence model**:

#### Chunk Storage

1. **In-Memory Cache** (Moka LRU cache):
   - Fast access to recently-used chunks
   - Configurable size limit

2. **LMDB Database** (via Heed wrapper):
   - Memory-mapped file for performance
   - ACID guarantees
   - Compression support (Gzip, Zstd, Brotli, etc.)

#### Chunk Format

**File:** `src/lib/world/src/chunk_format.rs`

```rust
pub struct Chunk {
    pub sections: [Section; 24],           // 24 vertical sections
    pub block_entities: Vec<BlockEntity>,
    pub heightmaps: Heightmaps,
    // ...
}

pub struct Section {
    pub block_count: u16,
    pub blocks: BlockStates,               // Palette + bit-packed array
    pub biomes: BiomeStates,              // Palette + bit-packed array
    pub sky_light: [u8; 2048],
    pub block_light: [u8; 2048],
}
```

#### Chunk Operations

| Operation | File | Description |
|-----------|------|-------------|
| `save_chunk` | `world/db_functions.rs` | Serializes and stores chunk |
| `load_chunk` | `world/db_functions.rs` | Deserializes and caches chunk |
| `chunk_exists` | `world/db_functions.rs` | Checks if chunk is in database |
| `batch_get_chunks` | `world/db_functions.rs` | Loads multiple chunks efficiently |
| `sync` | `world/lib.rs` | Flushes cache to disk |

#### World Importing

**File:** `src/lib/world/src/importing.rs`

Vanilla Minecraft worlds can be imported from Anvil format:

```bash
ferrumc import --import-path <path_to_world>
```

Process:
1. Scans `.mca` region files
2. Parses Anvil format using custom parser
3. Converts to FerrumC chunk format
4. Stores in LMDB database with compression

### 5. World Generation

**Files:** `src/lib/world_gen/`

FerrumC implements **procedural terrain generation** using noise functions.

#### Noise Generation

**File:** `src/lib/world_gen/src/lib.rs`

```rust
pub struct NoiseGenerator {
    noise: OpenSimplex,  // From 'noise' crate
}

// Multi-octave noise with frequency and amplitude variation
pub fn get_noise(&self, x: f64, y: f64, z: f64) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    
    for _ in 0..4 {  // 4 octaves
        total += self.noise.get([x * frequency, y * frequency, z * frequency]) * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    total
}
```

#### Biome Generators

**File:** `src/lib/world_gen/src/biomes/plains.rs`

Each biome implements the `BiomeGenerator` trait:

```rust
pub trait BiomeGenerator {
    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Result<Chunk>;
}

impl BiomeGenerator for PlainsBiome {
    fn generate_chunk(&self, chunk_x: i32, chunk_z: i32) -> Result<Chunk> {
        // 1. Create base water layer (sections -4 to 4)
        // 2. Generate height map using noise
        // 3. Fill stone for buried sections
        // 4. Use EditBatch for efficient grass/sand placement
        // 5. Calculate lighting
    }
}
```

#### Generation Pipeline

```
Chunk Request → WorldGenerator::generate_chunk()
              → BiomeGenerator::generate_chunk()
              → NoiseGenerator::get_noise()
              → Chunk with blocks, biomes, lighting
              → Save to database
```

### 6. NBT & Anvil Parsing

FerrumC implements **custom NBT and Anvil parsers** for performance.

#### NBT (Named Binary Tag)

**Files:** `src/lib/adapters/nbt/`

**Zero-copy deserialization** using a "tape" pattern:

```rust
pub enum NbtTag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List { element_type: u8, elements: Vec<NbtTag> },
    Compound(Vec<(String, NbtTag)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}
```

**Derive macros** for automatic serialization:

```rust
#[derive(NBTSerialize, NBTDeserialize)]
struct MyData {
    foo: i32,
    bar: String,
}
```

#### Anvil Region Files

**File:** `src/lib/adapters/anvil/src/lib.rs`

Anvil `.mca` files contain 32x32 chunk regions:

```rust
pub struct LoadedAnvilFile {
    data: Mmap,  // Memory-mapped file
}

impl LoadedAnvilFile {
    // Parse header to find chunk locations
    pub fn parse_chunk_locations(&self) -> Vec<ChunkLocation>;
    
    // Load specific chunk with decompression
    pub fn load_chunk(&self, x: u8, z: u8) -> Result<Vec<u8>>;
}
```

**Compression support**: Gzip, Zlib, LZ4, uncompressed

### 7. Command System

**Files:** `src/lib/commands/`

Commands use a **graph-based parsing system** with compile-time registration.

#### Command Graph

**File:** `src/lib/commands/src/graph/mod.rs`

```
Root
├── echo <string>                    # /echo hello
├── gamemode <mode> [player]         # /gamemode creative
└── nested
    ├── command1 <arg>               # /nested command1 value
    └── command2                     # /nested command2
```

#### Command Definition

**File:** `src/lib/default_commands/src/echo.rs`

```rust
#[command]
async fn echo_command(text: GreedyString, sender: Sender) -> anyhow::Result<()> {
    sender.send_system_message(format!("Echo: {}", text.0));
    Ok(())
}
```

The `#[command]` macro:
1. Registers command in the global command graph
2. Generates ECS system for handling the command
3. Auto-extracts and parses arguments from `CommandContext`

#### Command Flow

```
Client sends chat message starting with '/'
              ↓
Packet handler receives command string
              ↓
CommandDispatchEvent emitted
              ↓
Command resolver parses command graph
              ↓
ResolvedCommandDispatchEvent emitted
              ↓
Command system executes with parsed arguments
```

#### Argument Types

**File:** `src/lib/commands/src/arg/primitive/`

- `Bool`: Boolean flags
- `Int`, `Long`, `Float`, `Double`: Numeric arguments
- `SingleWord`: Single word (no spaces)
- `QuotableString`: Quoted or single word
- `GreedyString`: Consumes rest of input
- `Duration`: Time durations (e.g., "5s", "1h30m")

### 8. Plugin System

**Files:** `src/lib/plugins/`, `src/lib/scheduler/`

Plugin support is **planned** using Rust FFI (currently minimal implementation).

#### Plugin API

Plugins will be able to:
- Register custom commands
- Add ECS systems to schedules
- Listen to events
- Access world data

#### Timed Scheduler Integration

**File:** `src/lib/scheduler/src/lib.rs`

Plugins can register custom schedules:

```rust
pub struct TimedSchedule {
    pub name: String,
    pub period: Duration,
    pub schedule: Schedule,
    pub behavior: MissedTickBehavior,
    pub max_catch_up: Option<usize>,
    pub phase: Duration,  // Offset within period
}
```

---

## Detailed System Documentation

### Chunk Loading System

**Files:** `src/lib/core/src/chunks/`, `src/bin/src/chunk_sending.rs`

#### Chunk Receiver State Machine

```rust
pub enum ChunkReceiver {
    NotSentYet,                       // Initial state
    Sending { chunks_sent: usize },   // Actively sending chunks
    Sent,                             // All chunks sent
}
```

#### Loading Process

1. **New Connection**:
   - Player spawns with `ChunkReceiver::NotSentYet`

2. **Position Update**:
   - `cross_chunk_boundary` system detects chunk changes
   - Emits `CrossChunkBoundaryEvent`

3. **Chunk Sending**:
   - System calculates spiral pattern around player
   - Sends chunks in batches based on `chunk_batch_size` config
   - Updates `ChunkReceiver` state

4. **Chunk Batch Finished**:
   - Client acknowledges batch
   - Server sends next batch

### Player Movement System

**Files:** `src/bin/src/packet_handlers/play_packets/set_player_*.rs`

Player movement is validated and synchronized:

1. **Client sends position packet** (set_player_position, set_player_rotation, etc.)
2. **System validates movement**:
   - Check distance (anti-cheat)
   - Verify ground state
3. **Update ECS components** (Position, Rotation, OnGround)
4. **Emit TransformEvent**
5. **Broadcast to other players** (if entity tracking is implemented)

### Inventory Management

**Files:** `src/lib/inventories/`, `src/bin/src/packet_handlers/inventory.rs`

#### Inventory Component

```rust
pub struct Inventory {
    pub slots: [Option<ItemStack>; 46],  // 36 main + 9 hotbar + armor
}
```

#### Operations

- **Set slot**: Updates inventory slot and syncs to client
- **Get slot**: Retrieves item stack from slot
- **Swap slots**: Exchanges two slots
- **Clear**: Removes all items

### Message Queue System

**File:** `src/lib/core/src/mq.rs`, `src/bin/src/systems/mq.rs`

The message queue broadcasts system messages to players:

```rust
// Enqueue message
state.mq.queue_broadcast_message("Player joined!");

// Process queue (runs every tick)
system process_mq(query: Query<&StreamWriter>, mq: Res<MessageQueue>) {
    while let Some(msg) = mq.dequeue() {
        for writer in query.iter() {
            writer.send(msg.clone());
        }
    }
}
```

### Connection Lifecycle

1. **TCP Accept**: `tcp_conn_acceptor` accepts connection
2. **Handshake**: `handle_handshake` processes handshake packet
3. **Login**: Authentication and encryption (if online mode)
4. **Configuration**: Registry data, resource packs, feature flags
5. **Play**: Spawn player entity, send chunks, start gameplay
6. **Disconnect**: Cleanup entity, save player data, broadcast leave message

### Shutdown Process

**File:** `src/bin/src/systems/shutdown_systems/`

Graceful shutdown:

1. **Ctrl-C handler** sets `shut_down` flag
2. **Game loop exits**
3. **Shutdown schedule runs**:
   - Flush world data to disk
   - Disconnect all players
   - Close database
4. **TCP acceptor shuts down**
5. **Process exits**

---

## Data Flow

### Packet Processing Flow

```
TCP Socket (Tokio)
      ↓
PacketSkeleton::new() - Read packet length & ID
      ↓
handle_packet() - Macro-generated dispatcher
      ↓
Crossbeam channel
      ↓
ECS System (Bevy) - Runs during tick schedule
      ↓
Update Components & Emit Events
      ↓
StreamWriter::send() - Queue response packet
      ↓
Tokio channel
      ↓
Async write task
      ↓
TCP Socket (Tokio)
```

### Chunk Generation Flow

```
Player moves to unloaded chunk
      ↓
CrossChunkBoundaryEvent emitted
      ↓
Chunk loading system checks cache
      ↓
Cache miss → Check database
      ↓
Database miss → WorldGenerator::generate_chunk()
      ↓
BiomeGenerator::generate_chunk() (uses NoiseGenerator)
      ↓
Chunk created with blocks, biomes, lighting
      ↓
Save to database (compressed)
      ↓
Add to cache
      ↓
Send to client (ChunkDataAndUpdateLight packet)
      ↓
Client renders chunk
```

### Command Execution Flow

```
Client sends chat message starting with '/'
      ↓
ChatCommand packet received
      ↓
CommandDispatchEvent { sender, command_string }
      ↓
Command resolver parses command graph
      ↓
Match command nodes (literals & arguments)
      ↓
Extract and parse arguments (Int, String, etc.)
      ↓
ResolvedCommandDispatchEvent { sender, command, args }
      ↓
Command system executes
      ↓
Send response to sender
```

---

## Performance Optimizations

### 1. Custom NBT Parser

- **Zero-copy deserialization** using tape pattern
- **SIMD instructions** for parsing (when available)
- **Minimal allocations** during parsing

### 2. Memory-Mapped Files

- Anvil region files are memory-mapped for fast access
- OS handles paging and caching

### 3. Chunk Compression

- Database uses configurable compression (Zstd recommended)
- Balance between storage size and CPU usage

### 4. Batch Operations

- `EditBatch` for bulk block changes
- Database batch get/put reduces transaction overhead

### 5. Bevy ECS

- **Lockless concurrency** via archetypes
- **Cache-friendly data layout**
- **Parallel system execution** (when systems don't conflict)

### 6. Thread Pool

- Chunk generation uses thread pool for parallelism
- Offloads heavy work from tick thread

### 7. Caching

- **Moka LRU cache** for chunks
- Configurable size and eviction policy

### 8. Networking

- **Unbounded channels** minimize blocking
- **Async I/O** for efficient connection handling
- **Compression** enabled dynamically after login

---

## Development Reference

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Hyper-optimized build (LTO, single codegen unit)
cargo build --profile hyper

# Type checking (faster than build)
cargo check

# Run tests
cargo test

# Run specific test
cargo test test_name

# Run tests for specific package
cargo test -p ferrumc-text

# Linting
cargo clippy --all-targets -- -Dwarnings

# Formatting
cargo fmt --all

# Security audit
cargo audit
```

### Profiling

```bash
# Build with profiling symbols
cargo build --profile profiling

# Enable heap profiling (requires dhat feature)
cargo build --features dhat
```

### Code Style

- **Error handling**: Each crate has `thiserror`-based error types
- **Paths**: Use `get_root_path()` instead of relative paths
- **Imports**: Use `crate::*` in tests
- **Lints**: Use `#[expect(lint)]` instead of `#[allow(lint)]`
- **Cloning**: Avoid except for startup/config loading
- **Tests**: Mark data-dump tests with `#[ignore]`

### Adding a New Crate

1. Create directory in `src/lib/`
2. Add to workspace members in root `Cargo.toml`
3. Create `Cargo.toml` with workspace dependencies
4. Create `src/lib.rs` and `src/errors.rs`
5. Define `thiserror`-based error types

### Adding a New Packet Handler

1. Define packet struct in `src/lib/net/src/packets/incoming/`
2. Implement `NetDecode` trait
3. Create handler in `src/bin/src/packet_handlers/play_packets/`
4. Register in `play_packets/mod.rs`

### Adding a New Command

1. Create file in `src/lib/default_commands/src/`
2. Define function with `#[command]` attribute
3. Specify arguments as function parameters
4. Command is auto-registered at startup

### Adding a New ECS System

1. Create system function (queries, resources as parameters)
2. Register in appropriate schedule in `game_loop.rs`
3. Define events if needed in module's `events.rs`

### Testing

```rust
// Unit tests
#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn test_something() {
        // Test code
    }
    
    // Data generation test (not run in CI)
    #[test]
    #[ignore]
    fn generate_test_data() {
        // Generate test fixtures
    }
}
```

### Documentation

```rust
/// Brief description.
///
/// Longer description explaining behavior.
///
/// # Examples
///
/// ```
/// let x = foo();
/// ```
///
/// # Errors
///
/// Returns error if...
pub fn foo() -> Result<()> { }
```

---

## Key Technologies

| Technology | Purpose | Notes |
|------------|---------|-------|
| **Rust** | Language | Nightly required for some features |
| **Bevy ECS** | Entity Component System | 0.16.1, multi-threaded |
| **Tokio** | Async runtime | Current-thread runtime for networking |
| **Heed** | Database | LMDB wrapper, memory-mapped |
| **Moka** | Cache | LRU cache for chunks |
| **Noise** | Terrain generation | OpenSimplex noise |
| **Tracing** | Logging | Structured logging with spans |
| **Clap** | CLI parsing | Command-line arguments |
| **Flate2** | Compression | Gzip/Zlib support |

---

## Glossary

- **ECS**: Entity Component System - Data-oriented architecture pattern
- **NBT**: Named Binary Tag - Minecraft's data serialization format
- **Anvil**: Minecraft's world storage format (region files)
- **Chunk**: 16x16x384 block region (24 sections of 16x16x16)
- **Section**: 16x16x16 block cube within a chunk
- **Palette**: Compact storage for block states using indirection
- **LMDB**: Lightning Memory-Mapped Database
- **TPS**: Ticks Per Second - Game loop frequency
- **StreamWriter**: Component handling async packet transmission
- **Bevy Schedule**: Collection of ECS systems run together

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Pull request process
- Code style guidelines
- Branch naming conventions
- Testing requirements

---

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

---

## Additional Resources

- **Official Website**: https://www.ferrumc.com
- **Documentation**: https://docs.ferrumc.com
- **Discord**: https://discord.gg/qT5J8EMjwk
- **Minecraft Protocol**: https://minecraft.wiki/w/Java_Edition_protocol
- **NBT Format**: https://minecraft.wiki/w/NBT_format
- **Anvil Format**: https://minecraft.wiki/w/Anvil_file_format

---

*This document is maintained as part of the FerrumC project. For questions or corrections, please open an issue on GitHub or ask in Discord.*
