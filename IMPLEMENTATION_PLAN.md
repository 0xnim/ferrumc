# Plugin API Redesign - Implementation Plan

**Goal:** Implement capability-based plugin system with compile-time enforcement

---

## Phase Overview

1. **Phase 1: Capability System** (Foundation) - 2 hours
2. **Phase 2: Build Context** (New API) - 1 hour  
3. **Phase 3: Query System** (Safe queries) - 2 hours
4. **Phase 4: Domain API Consolidation** (Move events) - 3 hours
5. **Phase 5: Plugin Migration** (Update all plugins) - 2 hours
6. **Phase 6: Enforcement** (Remove old access) - 1 hour
7. **Phase 7: Testing & Validation** (Verify) - 1 hour

**Total Estimated Time:** 12 hours

---

## Phase 1: Capability System ✅

### Files to Create
- `src/lib/plugin-api/src/capabilities.rs`

### Files to Modify
- `src/lib/plugin-api/src/lib.rs` - Add capabilities module
- `src/lib/plugin-api/src/lib.rs` - Add capabilities() to Plugin trait

### Implementation
```rust
// capabilities.rs
pub struct PluginCapabilities {
    pub animation_api: bool,
    pub block_api: bool,
    pub chat_api: bool,
    pub inventory_api: bool,
    pub entity_queries: bool,
    pub world_queries: bool,
}

pub struct PluginCapabilitiesBuilder { ... }
```

---

## Phase 2: Build Context ✅

### Files to Create
- `src/lib/plugin-api/src/build_context.rs`

### Files to Modify
- `src/lib/plugin-api/src/context.rs` - Keep but deprecate
- `src/lib/plugin-api/src/lib.rs` - Export PluginBuildContext

### Implementation
```rust
// build_context.rs
pub struct PluginBuildContext<'a> {
    capabilities: PluginCapabilities,
    systems: SystemRegistry<'a>,
    events: EventRegistry<'a>,
    config: PluginConfig,
    // NO world, NO state
}

pub struct SystemRegistry<'a> { ... }
pub struct EventRegistry<'a> { ... }
```

---

## Phase 3: Query System ✅

### Files to Create
- `src/lib/plugin-api/src/queries/entity.rs`
- `src/lib/plugin-api/src/queries/world.rs`
- `src/lib/plugin-api/src/queries/mod.rs`

### Implementation
```rust
// queries/entity.rs
#[derive(SystemParam)]
pub struct EntityQueries<'w, 's> {
    positions: Query<'w, 's, &'static Position>,
    identities: Query<'w, 's, &'static PlayerIdentity>,
    // NO StreamWriter, NO GlobalState
}

// queries/world.rs
#[derive(SystemParam)]
pub struct WorldQueries<'w> {
    state: Res<'w, GlobalStateResource>,
    // Read-only access only
}
```

---

## Phase 4: Domain API Consolidation ✅

### Animation API
**Move events:** plugin-api/events.rs → animation-api/events.rs
**Consolidate API:** Separate readers/writers or combined

### Block API
**Move events:** plugin-api/events.rs → block-api/events.rs
**Already has:** BlockRequests/BlockBroadcasts separation

### Chat API
**Move events:** plugin-api/events.rs → chat-api/events.rs
**Update:** ChatAPI SystemParam

### Inventory API
**Move events:** plugin-api/events.rs → inventory-api/events.rs
**Create:** InventoryAPI SystemParam

---

## Phase 5: Plugin Migration ✅

### Update Each Plugin
1. Add `capabilities()` method
2. Replace `PluginContext` with `PluginBuildContext`
3. Update system signatures to use new query APIs
4. Use consolidated domain APIs

### Plugins to Update
- animations
- blocks
- chat
- inventory
- default-commands
- hello

---

## Phase 6: Enforcement ✅

### Remove Old Access
- Mark `PluginContext` as deprecated
- Remove `world` field (breaking change)
- Remove `state` field (breaking change)

### Add Validation
- Validate system params match declared capabilities
- Runtime checks in registry

---

## Phase 7: Testing ✅

### Verification
- All plugins compile
- Server starts
- Plugins load correctly
- Functionality works (place blocks, chat, etc.)
- No I/O violations possible

---

## Execution Order

1. Create capability system (backward compatible)
2. Create build context (parallel to old)
3. Create query APIs (new tools)
4. Update domain APIs (consolidate)
5. Migrate plugins one by one
6. Remove old PluginContext
7. Test everything

---

## Rollback Points

- After Phase 1: Can rollback, capabilities optional
- After Phase 2: Can rollback, old context still works
- After Phase 3: Can rollback, queries are additions
- After Phase 4: Hard to rollback (events moved)
- After Phase 5: Hard to rollback (plugins updated)
- After Phase 6: Breaking changes committed
- After Phase 7: Done

---

## Success Criteria

✅ All plugins use PluginBuildContext  
✅ All plugins declare capabilities  
✅ No direct World/State access  
✅ Domain APIs are mandatory gateways  
✅ Compile-time enforcement works  
✅ All tests pass  
✅ Server runs correctly  
