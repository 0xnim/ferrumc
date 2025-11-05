# FerrumC Plugin Architecture Migration TODO

This document tracks what features need to be migrated from the binary to the plugin architecture.

## ✅ Completed Migrations

### 1. Animations System
- **API:** `ferrumc-animation-api`
- **Core Systems:** `ferrumc-core-systems/animations`
- **Plugin:** `ferrumc-plugin-animations`
- **Features:**
  - Arm swing animations (main/offhand)
  - Player commands (sneak, sprint, leave bed)
  - Entity pose changes

### 2. Chat System
- **API:** `ferrumc-chat-api`
- **Core Systems:** `ferrumc-core-systems/chat`
- **Plugin:** `ferrumc-plugin-chat`
- **Features:**
  - Chat message formatting and broadcasting
  - System messages

### 3. Default Commands
- **Plugin:** `ferrumc-plugin-default-commands`
- **Features:**
  - Bridge for legacy commands (echo, nested)
  - mq queue → chat API conversion

### 4. Inventory System
- **API:** `ferrumc-inventory-api`
- **Core Systems:** `ferrumc-core-systems/inventory`
- **Plugin:** `ferrumc-plugin-inventory`
- **Features:**
  - Creative mode inventory management
  - Hotbar slot selection (0-8)
  - Inventory validation and updates

---

## 🚧 High Priority - Game Logic (Should be Plugins)

### 5. Block System ⭐ HIGHEST PRIORITY
**Current Location:** `src/bin/src/packet_handlers/play_packets/`
- `place_block.rs` - Block placement with collision detection
- `player_action.rs` - Block breaking

**Needs:**
- ✅ `ferrumc-block-api` - BlockAPI trait, BlockPlaceAttempt, BlockBreakAttempt events
- ✅ `ferrumc-core-systems/blocks` - Packet handlers and broadcasters
- ✅ `ferrumc-plugin-blocks` - Validation, collision checks, placement rules

**Game Logic (belongs in plugin):**
- Item → Block mapping
- Collision detection with entities
- Block face offset calculation
- Placement validation
- Block break validation
- Broadcasting block updates to nearby players

**Core I/O (stays in core-systems):**
- Receiving PlaceBlock/PlayerAction packets
- Sending BlockUpdate/BlockChangeAck packets
- World chunk loading/saving

---

### 6. Movement System
**Current Location:** `src/bin/src/packet_handlers/play_packets/`
- `set_player_position.rs` - Position updates
- `set_player_position_and_rotation.rs` - Position + rotation
- `set_player_rotation.rs` - Rotation only

**Needs:**
- ✅ `ferrumc-movement-api` - MovementAPI trait, events
- ✅ `ferrumc-core-systems/movement` - Packet handlers and broadcasters
- ✅ `ferrumc-plugin-movement` - Movement validation, anti-cheat

**Game Logic:**
- Movement validation (speed, teleport detection)
- Cross-chunk boundary detection (currently in `systems/cross_chunk_boundary.rs`)
- Broadcasting position updates to nearby players
- OnGround state management

---

### 7. Entity Tracking System
**Current Location:** `src/bin/src/packet_handlers/play_packets/player_loaded.rs`

**Needs:**
- ✅ `ferrumc-entity-api` - Entity tracking events
- ✅ `ferrumc-core-systems/entities` - Packet handlers
- ✅ `ferrumc-plugin-entity-tracking` - Entity visibility, spawning

**Game Logic:**
- Tracking which entities are visible to which players
- Entity spawn/despawn events
- Player loaded event handling

---

## 🔧 Medium Priority - Infrastructure (Core Systems)

### 8. Keep Alive System
**Current Location:** `src/bin/src/systems/keep_alive_system.rs`

**Status:** Should stay in binary or move to core-systems (pure I/O)
- Sends periodic keep-alive packets
- No game logic, just network health check

---

### 9. System Messages
**Current Location:** `src/bin/src/systems/system_messages/`
- `player_join.rs` - Join messages
- `player_leave.rs` - Leave messages

**Needs:**
- ✅ Move to `ferrumc-plugin-system-messages`
- Use ChatAPI to broadcast join/leave messages
- Listen to PlayerConnectedEvent / PlayerDisconnectedEvent

---

### 10. Chunk Management
**Current Location:** `src/bin/src/systems/send_chunks.rs`

**Status:** Should stay in core-systems (I/O layer)
- Sends chunks to players
- Pure I/O, no game logic

---

## ✅ Leave in Binary - Core Infrastructure

### Connection Management
- `new_connections.rs` - Accept new connections
- `connection_killer.rs` - Disconnect players
- **Reason:** Core networking infrastructure, not game logic

### World Sync
- `world_sync.rs` - Periodic world saves
- **Reason:** Core persistence layer

### Player Count
- `player_count_update.rs` - Update server list player count
- **Reason:** Server status, not gameplay

### LAN Pinger
- `lan_pinger.rs` - LAN server discovery
- **Reason:** Network infrastructure

### Commands
- `command.rs` - Command parsing and execution
- `command_suggestions.rs` - Tab completion
- **Reason:** Command infrastructure (individual commands are in plugins)

### Other Packets (Low Priority)
- `chunk_batch_ack.rs` - Chunk loading acknowledgment
- `confirm_player_teleport.rs` - Teleport confirmation
- `keep_alive.rs` - Keep-alive response
- **Reason:** Protocol-level packets, minimal/no game logic

---

## 🎯 Recommended Migration Order

1. ✅ **Block System** (High value, clear boundaries, good learning example)
2. ✅ **Inventory System** (Related to blocks, simpler logic)
3. **Movement System** (Complex but critical for gameplay)
4. **Entity Tracking** (Enables more features like mobs, items)
5. **System Messages** (Simple, good for practice)

---

## 📋 Migration Checklist Template

For each system:

- [ ] Create API crate (`src/lib/apis/{name}-api/`)
  - [ ] Define events (input and request)
  - [ ] Define traits (SystemParam API)
  - [ ] Define types (domain models)
  - [ ] Add to workspace Cargo.toml

- [ ] Create core-systems module (`src/lib/core-systems/src/{name}/`)
  - [ ] Implement packet → event handlers
  - [ ] Implement request → packet broadcasters
  - [ ] Add to core-systems Cargo.toml

- [ ] Create plugin (`src/lib/plugins/core/{name}/`)
  - [ ] Implement game logic systems
  - [ ] Use domain API (no direct packet access)
  - [ ] Add to workspace Cargo.toml

- [ ] Integrate
  - [ ] Register core systems in `game_loop.rs`
  - [ ] Register plugin in `plugin_loader.rs`
  - [ ] Add dependencies to `src/bin/Cargo.toml`

- [ ] Clean up
  - [ ] Remove old packet handlers from `src/bin/src/packet_handlers/`
  - [ ] Remove old systems from `src/bin/src/systems/`
  - [ ] Update mod.rs files

- [ ] Test
  - [ ] Run `cargo check`
  - [ ] Run `cargo build`
  - [ ] Test in-game functionality

---

## 📚 Reference Implementations

- **Best example:** Animation system (complete, well-documented)
- **Simpler example:** Chat system (clean event flow)
- **Bridge pattern:** Default-commands (legacy compatibility)
