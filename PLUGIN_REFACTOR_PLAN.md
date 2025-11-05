# Plugin Refactor Plan: Core vs Vanilla Split

## Philosophy

**Core Plugins:** Receive network data → Update server ECS state (NO broadcasting)
**Vanilla Plugins:** Validate + decide who sees what → Broadcast to other players

## Current Progress

### ✅ Created (Ready to Use)

1. **`ferrumc-plugin-entity-sync`** (core)
   - Updates Position/Rotation/OnGround from packets
   - Fires CrossChunkBoundaryEvent
   - NO broadcasting

2. **`ferrumc-plugin-vanilla-movement`** (vanilla)
   - Validates movement (teleport detection)
   - Broadcasts to all players (vanilla behavior)
   - Future: anti-cheat, physics

## Remaining Work

### Core Plugins to Create

3. **`ferrumc-plugin-inventory-sync`** (core)
   ```rust
   // Just updates inventory components, no broadcasting
   - handle_set_creative_slot() → Update inventory component
   - handle_set_held_item() → Update hotbar component
   ```

### Vanilla Plugins to Create

4. **`ferrumc-plugin-vanilla-blocks`** (vanilla)
   ```rust
   // Block placement/breaking logic + broadcasting
   - validate_placement() → Check rules
   - broadcast_block_change() → Send to all players
   ```

5. **`ferrumc-plugin-vanilla-chat`** (vanilla)
   ```rust
   // Chat formatting + broadcasting
   - format_message() → Apply vanilla formatting
   - broadcast_chat() → Send to all players
   ```

6. **`ferrumc-plugin-vanilla-animations`** (vanilla)
   ```rust
   // Animation logic + broadcasting
   - handle_swing() → Determine animation
   - broadcast_animation() → Send to all players
   ```

7. **`ferrumc-plugin-vanilla-join-leave`** (vanilla)
   ```rust
   // Join/leave message formatting
   - format_join_message() → Yellow text
   - broadcast_to_all() → Send to all players
   ```

## Directory Structure

```
src/lib/plugins/
├── core/                    # State updates only
│   ├── entity-sync/         # ✅ Position/rotation updates
│   └── inventory-sync/      # TODO: Inventory updates
│
└── vanilla/                 # Vanilla Minecraft behavior
    ├── movement/            # ✅ Movement validation + broadcasting
    ├── blocks/              # TODO: Block rules + broadcasting
    ├── chat/                # TODO: Chat formatting + broadcasting
    ├── animations/          # TODO: Animation logic + broadcasting
    └── join-leave/          # TODO: Join/leave messages
```

## Workspace Updates Needed

### Add to `Cargo.toml` members:
```toml
"src/lib/plugins/core/entity-sync",
"src/lib/plugins/core/inventory-sync",
"src/lib/plugins/vanilla/movement",
"src/lib/plugins/vanilla/blocks",
"src/lib/plugins/vanilla/chat",
"src/lib/plugins/vanilla/animations",
"src/lib/plugins/vanilla/join-leave",
```

### Update `src/bin/Cargo.toml`:
```toml
ferrumc-plugin-entity-sync = { path = "../lib/plugins/core/entity-sync" }
ferrumc-plugin-inventory-sync = { path = "../lib/plugins/core/inventory-sync" }
ferrumc-plugin-vanilla-movement = { path = "../lib/plugins/vanilla/movement" }
ferrumc-plugin-vanilla-blocks = { path = "../lib/plugins/vanilla/blocks" }
ferrumc-plugin-vanilla-chat = { path = "../lib/plugins/vanilla/chat" }
ferrumc-plugin-vanilla-animations = { path = "../lib/plugins/vanilla/animations" }
ferrumc-plugin-vanilla-join-leave = { path = "../lib/plugins/vanilla/join-leave" }
```

### Update `src/bin/src/plugin_loader.rs`:
```rust
// Core plugins (state sync)
registry.register::<ferrumc_plugin_entity_sync::EntitySyncPlugin>();
registry.register::<ferrumc_plugin_inventory_sync::InventorySyncPlugin>();

// Vanilla plugins (game logic)
registry.register::<ferrumc_plugin_vanilla_movement::VanillaMovementPlugin>();
registry.register::<ferrumc_plugin_vanilla_blocks::VanillaBlocksPlugin>();
registry.register::<ferrumc_plugin_vanilla_chat::VanillaChatPlugin>();
registry.register::<ferrumc_plugin_vanilla_animations::VanillaAnimationsPlugin>();
registry.register::<ferrumc_plugin_vanilla_join_leave::VanillaJoinLeavePlugin>();

// Keep
registry.register::<ferrumc_plugin_default_commands::DefaultCommandsPlugin>();
registry.register::<ferrumc_plugin_hello::HelloPlugin>();
```

## Cleanup Needed

### Remove old plugin directories:
```bash
rm -rf src/lib/plugins/core/animations
rm -rf src/lib/plugins/core/blocks
rm -rf src/lib/plugins/core/chat
rm -rf src/lib/plugins/core/inventory
rm -rf src/lib/plugins/core/join-leave
rm -rf src/lib/plugins/core/movement
```

## API Improvements Needed

### Movement API needs per-player broadcasting:
```rust
// Add to BroadcastMovementRequest:
pub receiver: Option<Entity>, // None = all, Some = specific player

// Add methods:
impl MovementAPI {
    pub fn broadcast_movement_to(&mut self, player, receiver, ...);
    pub fn broadcast_movement_all(&mut self, player, ...);
}
```

### Inventory broadcaster needs fixing:
```rust
// Use request data instead of hardcoding:
let packet = SetContainerSlot {
    slot_index: request.slot_index,  // Not HEAD_SLOT
    slot: request.slot.clone(),      // Not hardcoded values
};
```

## Testing Plan

1. Compile with just entity-sync + vanilla-movement
2. Test movement works (position updates + broadcasting)
3. Add inventory-sync
4. Add remaining vanilla plugins one by one
5. Remove old plugins
6. Full integration test

## Next Steps

1. Finish creating remaining plugin skeletons
2. Update workspace Cargo.toml
3. Update plugin_loader.rs
4. cargo check
5. Fix compilation errors
6. Test in-game
7. Remove old plugins
