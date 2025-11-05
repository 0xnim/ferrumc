# Next Plugin Migrations - Priority Order

**Status:** Post-cleanup analysis  
**Date:** November 5, 2025  
**Plugins Migrated:** 6 (animations, blocks, chat, inventory, default-commands, hello)

---

## 🎯 **NEXT PRIORITY: Movement Plugin**

### Why Movement is Next
1. ✅ **High value** - Core gameplay mechanic (players need to move!)
2. ✅ **Clear boundaries** - Well-defined input (packets) and output (position updates)
3. ✅ **Related to blocks** - Cross-chunk boundary detection needed for chunk loading
4. ✅ **~200 lines** - Significant cleanup of binary
5. ✅ **Builds on existing** - Can follow animations/blocks patterns

### Current State (In Binary)
**Packet Handlers:**
- `src/bin/src/packet_handlers/play_packets/set_player_position.rs` (~150 lines)
- `src/bin/src/packet_handlers/play_packets/set_player_position_and_rotation.rs` (~100 lines)
- `src/bin/src/packet_handlers/play_packets/set_player_rotation.rs` (~80 lines)
- `src/bin/src/packet_handlers/player/head_rot.rs` (~50 lines)

**Systems:**
- `src/bin/src/systems/cross_chunk_boundary.rs` (~50 lines) - Chunk loading logic

**Total:** ~430 lines to migrate

### What Movement Plugin Will Do

**Game Logic (Plugin):**
- ✅ Validate movement (speed checks, teleport detection)
- ✅ Update Position/Rotation/OnGround components
- ✅ Detect cross-chunk boundaries
- ✅ Emit CrossChunkBoundaryEvent for chunk system
- ✅ Anti-cheat: distance validation
- ✅ Movement interpolation/smoothing

**Core I/O (Core-Systems):**
- ✅ Receive SetPlayerPosition packets → emit events
- ✅ Receive SetPlayerRotation packets → emit events  
- ✅ Broadcast UpdateEntityPosition packets to nearby players
- ✅ Broadcast UpdateEntityRotation packets to nearby players
- ✅ Handle teleport confirmations

### Migration Steps

#### 1. Create movement-api crate
```
src/lib/apis/movement-api/
├── src/
│   ├── lib.rs
│   ├── events.rs        # PlayerMoveEvent, PlayerRotateEvent, etc.
│   ├── traits.rs        # MovementAPI trait
│   └── types.rs         # Velocity, MovementType, etc.
└── Cargo.toml
```

**Events:**
```rust
// Input events (from core)
PlayerMoveEvent { player, from, to, on_ground }
PlayerRotateEvent { player, yaw, pitch }
PlayerTeleportEvent { player, position, rotation }
CrossChunkBoundaryEvent { player, old_chunk, new_chunk }

// Request events (to core)
UpdatePositionRequest { player, position, on_ground, broadcast }
UpdateRotationRequest { player, yaw, pitch, broadcast }
TeleportPlayerRequest { player, position, rotation }
```

**API Trait:**
```rust
#[derive(SystemParam)]
pub struct MovementAPI<'w> {
    move_events: EventWriter<'w, UpdatePositionRequest>,
    rotate_events: EventWriter<'w, UpdateRotationRequest>,
    teleport_events: EventWriter<'w, TeleportPlayerRequest>,
}

impl<'w> MovementAPI<'w> {
    pub fn update_position(&mut self, player, position, on_ground);
    pub fn update_rotation(&mut self, player, yaw, pitch);
    pub fn teleport(&mut self, player, position, rotation);
    pub fn broadcast_movement(&mut self, player, exclude_self);
}
```

#### 2. Create core-systems/movement module
```
src/lib/core-systems/src/movement/
├── mod.rs
├── packet_handlers.rs   # Packet → Event converters
└── broadcasters.rs      # Request → Packet broadcasters
```

**Packet Handlers:**
- `handle_set_player_position_packets()` - SetPlayerPositionPacket → PlayerMoveEvent
- `handle_set_player_rotation_packets()` - SetPlayerRotationPacket → PlayerRotateEvent
- `handle_confirm_teleport_packets()` - ConfirmTeleport → validation

**Broadcasters:**
- `broadcast_position_updates()` - UpdatePositionRequest → UpdateEntityPositionPacket
- `broadcast_rotation_updates()` - UpdateRotationRequest → UpdateEntityRotationPacket
- `send_teleport_packets()` - TeleportPlayerRequest → TeleportEntityPacket

#### 3. Create movement plugin
```
src/lib/plugins/core/movement/
├── src/
│   └── lib.rs
└── Cargo.toml
```

**Plugin Systems:**
```rust
fn handle_player_movement(
    mut events: EventReader<PlayerMoveEvent>,
    mut movement: MovementAPI,
    mut query: Query<(&mut Position, &mut OnGround)>,
    mut cross_chunk: EventWriter<CrossChunkBoundaryEvent>,
) {
    for event in events.read() {
        // Validate movement (anti-cheat)
        if !validate_movement_speed(&event.from, &event.to) {
            continue; // Reject suspicious movement
        }
        
        // Update position component
        if let Ok((mut pos, mut on_ground)) = query.get_mut(event.player) {
            let old_chunk = (pos.x as i32 >> 4, pos.z as i32 >> 4);
            *pos = event.to;
            *on_ground = OnGround(event.on_ground);
            let new_chunk = (pos.x as i32 >> 4, pos.z as i32 >> 4);
            
            // Detect cross-chunk boundary
            if old_chunk != new_chunk {
                cross_chunk.send(CrossChunkBoundaryEvent {
                    player: event.player,
                    old_chunk,
                    new_chunk,
                });
            }
        }
        
        // Broadcast to nearby players
        movement.broadcast_movement(event.player, true);
    }
}

fn handle_player_rotation(
    mut events: EventReader<PlayerRotateEvent>,
    mut query: Query<&mut Rotation>,
) {
    for event in events.read() {
        if let Ok(mut rotation) = query.get_mut(event.player) {
            rotation.yaw = event.yaw;
            rotation.pitch = event.pitch;
        }
    }
}
```

**Plugin Priority:** 45 (between animations:50 and blocks:40)

**Dependencies:** None (base system)

#### 4. Integration
- Register core systems in `game_loop.rs`
- Register plugin in `plugin_loader.rs`
- Add dependencies to workspace and binary
- Remove old packet handlers from binary
- Update MIGRATION_TODO.md

### Expected Results

**Before:**
- Binary: 430 lines of movement code
- No validation/anti-cheat
- Movement broadcasts not implemented

**After:**
- Binary: -430 lines (just bootstrap)
- movement-api: ~150 lines
- core-systems/movement: ~200 lines
- movement plugin: ~150 lines
- ✅ Validation logic centralized
- ✅ Clean separation of concerns
- ✅ Future: easy to add anti-cheat plugins

---

## 🥈 **Second Priority: Chunk Management Plugin**

### Why Chunk Management is Second
1. ✅ **Depends on movement** - Needs CrossChunkBoundaryEvent
2. ✅ **Medium complexity** - ~250 lines
3. ✅ **Clear I/O boundary** - Chunk loading/sending is clearly I/O
4. ✅ **Completes movement** - Makes movement system fully functional

### Current State (In Binary)
- `src/bin/src/systems/cross_chunk_boundary.rs` (~50 lines) - Logic for what chunks to load
- `src/bin/src/systems/send_chunks.rs` (~150 lines) - I/O for sending chunks
- `src/bin/src/chunk_sending.rs` (~100 lines) - Chunk batching logic

**Total:** ~300 lines

### What Chunk Management Will Do

**Game Logic (Plugin):**
- ✅ Listen to CrossChunkBoundaryEvent
- ✅ Calculate which chunks to load (render distance logic)
- ✅ Determine chunk priority (spiral from center)
- ✅ Request chunk loading from API

**Core I/O (Core-Systems):**
- ✅ Load chunks from world
- ✅ Generate chunks if missing
- ✅ Encode chunk data to packets
- ✅ Send chunk batches to client
- ✅ Handle ChunkBatchAck responses

### Migration Steps

Similar to movement, but create:
- `chunk-api` crate with ChunkAPI trait
- `core-systems/chunks` module
- `chunk-management` plugin

**Events:**
```rust
ChunkLoadRequest { player, chunks: Vec<(i32, i32)>, center }
ChunkUnloadRequest { player, chunks: Vec<(i32, i32)> }
ChunkBatchCompleteEvent { player, chunks_sent }
```

**Plugin Priority:** 35 (runs after movement:45)

**Dependencies:** `["movement"]` (needs CrossChunkBoundaryEvent)

---

## 🥉 **Third Priority: System Messages Plugin**

### Why System Messages is Third
1. ✅ **Simple** - Only ~100 lines total
2. ✅ **Good learning** - Clean example of using ChatAPI
3. ✅ **Low risk** - Simple event listening
4. ✅ **High visibility** - Players see join/leave messages

### Current State (In Binary)
- `src/bin/src/systems/system_messages/player_join.rs` (~15 lines)
- `src/bin/src/systems/system_messages/player_leave.rs` (~15 lines)

**Total:** ~30 lines (small but important!)

### What System Messages Will Do

**Game Logic (Plugin):**
- ✅ Listen to PlayerJoinEvent
- ✅ Listen to PlayerLeaveEvent
- ✅ Format join/leave messages (colored, styled)
- ✅ Broadcast via ChatAPI

**No Core-Systems needed** - Just uses existing ChatAPI

### Migration Steps

```
src/lib/plugins/core/system-messages/
├── src/
│   └── lib.rs
└── Cargo.toml
```

**Plugin:**
```rust
fn handle_player_join(
    mut events: EventReader<PlayerJoinEvent>,
    mut chat: ChatAPI,
) {
    for event in events.read() {
        let mut message = TextComponent::from(format!("{} joined the game", event.username));
        message.color = Some(Color::Named(NamedColor::Yellow));
        chat.broadcast(message);
    }
}

fn handle_player_leave(
    mut events: EventReader<PlayerLeaveEvent>,
    mut chat: ChatAPI,
) {
    for event in events.read() {
        let mut message = TextComponent::from(format!("{} left the game", event.username));
        message.color = Some(Color::Named(NamedColor::Yellow));
        chat.broadcast(message);
    }
}
```

**Plugin Priority:** 20 (decorative, runs after core systems)

**Dependencies:** `["chat"]` (uses ChatAPI)

**Note:** Need to define PlayerJoinEvent and PlayerLeaveEvent (probably in a connection-api or player-api)

---

## 📊 Migration Impact Summary

### After All Three Migrations

| Plugin | Lines | Priority | Dependencies | Status |
|--------|-------|----------|--------------|--------|
| Animations | 140 | 50 | - | ✅ Done |
| Movement | 150 | 45 | - | ⏳ Next |
| Blocks | 200 | 40 | - | ✅ Done |
| Chunk Mgmt | 100 | 35 | movement | ⏳ Planned |
| Chat | 90 | 30 | - | ✅ Done |
| System Msgs | 30 | 20 | chat | ⏳ Planned |
| Commands | 60 | 10 | chat | ✅ Done |
| Inventory | 80 | 40 | - | ✅ Done |
| Hello | 60 | 0 | - | ✅ Done |

**Binary Reduction:**
- Current: ~430 lines in packet_handlers + ~300 in systems = **730 lines**
- After migration: **~50 lines** (just bootstrap)
- **Reduction: 93% of gameplay code moved to plugins**

---

## 🚀 Recommended Timeline

### Week 1: Movement Plugin
**Days 1-2:** Create movement-api crate  
**Days 3-4:** Create core-systems/movement  
**Day 5:** Create movement plugin  
**Day 6:** Integration and testing  
**Day 7:** Bug fixes, documentation

### Week 2: Chunk Management Plugin
**Days 1-2:** Create chunk-api crate  
**Days 3-4:** Create core-systems/chunks  
**Day 5:** Create chunk-management plugin  
**Day 6:** Integration and testing  
**Day 7:** Polish and optimization

### Week 3: System Messages + Cleanup
**Days 1-2:** System messages plugin (simple)  
**Days 3-5:** Comprehensive testing of all plugins  
**Days 6-7:** Documentation, examples, guides

---

## 🎓 Learning Value

Each migration teaches different aspects:

| Migration | Learning Focus |
|-----------|----------------|
| **Movement** | Complex validation, component updates, event chains |
| **Chunk Mgmt** | Geometry calculations, I/O-heavy operations, batching |
| **System Msgs** | Simple plugin, dependency usage, ChatAPI integration |

---

## ✅ Success Criteria

After these three migrations:

1. ✅ Binary has <100 lines of gameplay code
2. ✅ All movement is validated
3. ✅ Chunk loading is properly managed
4. ✅ Players see join/leave messages
5. ✅ 9 total plugins operational
6. ✅ Documentation complete
7. ✅ Architecture principles fully enforced

---

## 🔮 Future Migrations (Lower Priority)

### Entity Tracking (~200 lines)
- Make players visible to each other
- Spawn/despawn entities
- Entity metadata sync

### Health System (~150 lines)
- Health, food, saturation
- Damage from environment
- Death and respawn

### Combat System (~200 lines)
- Attack detection
- Damage calculation
- Knockback

### Item Entities (~150 lines)
- Drop items on ground
- Pickup mechanics
- Item entity physics

---

## 📝 Notes

- Keep `new_connections.rs`, `connection_killer.rs`, `keep_alive_system.rs` in binary (pure infrastructure)
- Keep `world_sync.rs`, `player_count_update.rs`, `lan_pinger.rs` in binary (server management)
- Move ALL gameplay logic to plugins
- Core-systems is for I/O only (packet conversion)

---

**Start with Movement Plugin - it's the logical next step!**
