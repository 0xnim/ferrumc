# Plugin API Redesign - Implementation Status

**Date:** November 5, 2025  
**Status:** Phase 1-3 Complete (Foundation Ready)

---

## ✅ Completed Phases

### Phase 1: Capability System ✅ **COMPLETE**

**Files Created:**
- ✅ `src/lib/plugin-api/src/capabilities.rs` (239 lines)
  - `PluginCapabilities` struct with all domain API flags
  - `PluginCapabilitiesBuilder` with fluent API
  - `ResourceCapability` for resource access control

**Files Modified:**
- ✅ `src/lib/plugin-api/src/lib.rs`
  - Added `capabilities()` method to `Plugin` trait
  - Exports capability types
  - Default implementation returns `all()` for backward compat

**Features:**
- ✅ Capability declarations for all domain APIs
- ✅ Builder pattern for fluent API
- ✅ Resource-level access control
- ✅ Read-only resource support
- ✅ Backward compatible (default: all capabilities)

---

### Phase 2: Build Context ✅ **COMPLETE**

**Files Created:**
- ✅ `src/lib/plugin-api/src/build_context.rs` (225 lines)
  - `PluginBuildContext` - no World/State access
  - `SystemRegistry` - system registration API
  - `EventRegistry` - event/resource registration API

**Features:**
- ✅ No direct `World` access
- ✅ No direct `State` access
- ✅ Fluent API for registration
- ✅ Timed system support
- ✅ Resource insertion support
- ✅ Component registration support

**Example Usage:**
```rust
fn build(&self, mut ctx: PluginBuildContext<'_>) {
    ctx.events()
        .register::<MyEvent>()
        .insert_resource(MyResource::default());
    
    ctx.systems()
        .add_tick(my_system)
        .add_timed("periodic", Duration::from_secs(5), periodic_system);
}
```

---

### Phase 3: Query System ✅ **COMPLETE**

**Files Created:**
- ✅ `src/lib/plugin-api/src/queries/mod.rs`
- ✅ `src/lib/plugin-api/src/queries/entity.rs` (178 lines)
  - `EntityQueries` SystemParam
  - Safe component queries (Position, Rotation, PlayerIdentity)
  - NO StreamWriter, NO GlobalState access
- ✅ `src/lib/plugin-api/src/queries/world.rs` (131 lines)
  - `WorldQueries` SystemParam
  - Read-only world access
  - Block queries, chunk loaded checks

**Features:**
- ✅ `EntityQueries` - Safe entity component access
  - `position()`, `rotation()`, `identity()`
  - `iter_players()`, `iter_transforms()`
  - `players_in_range()`, `player_count()`
  - `distance_between()`
- ✅ `WorldQueries` - Read-only world data
  - `get_block()`, `chunk_loaded()`, `is_loaded()`
  - `get_highest_block()`
  - Helper coordinate conversions

**Example Usage:**
```rust
fn my_system(
    entities: EntityQueries,
    world: WorldQueries,
) {
    // Query safe components
    if let Some(pos) = entities.position(player) {
        // Check world data (read-only)
        if let Some(block) = world.get_block(pos.x as i32, pos.y as i32, pos.z as i32) {
            // ...
        }
    }
    
    // Find nearby players
    let nearby = entities.players_in_range(&pos, 100.0);
}
```

---

## 🚧 Remaining Phases

### Phase 4: Domain API Consolidation (Not Started)

**Scope:** LARGE - affects all domain API crates

**Work Required:**

1. **Move Events from plugin-api to Domain APIs**
   - BlockPlacedEvent → block-api
   - BlockBrokenEvent → block-api
   - ChatMessageEvent → chat-api
   - PlayerJoinEvent, PlayerLeaveEvent → player-api (create)
   - PlayerMoveEvent → movement-api (create)
   - PlayerDamageEvent, etc → combat-api (create)

2. **Update Domain API Traits**
   - Make all fields private (prevent bypassing)
   - Consolidate readers + writers
   - Consider AnimationReader/AnimationWriter pattern vs combined API

3. **Create Missing Domain APIs**
   - movement-api
   - player-api
   - combat-api (for damage events)

**Files to Modify:**
- `src/lib/apis/animation-api/src/*`
- `src/lib/apis/block-api/src/*`
- `src/lib/apis/chat-api/src/*`
- `src/lib/apis/inventory-api/src/*`
- `src/lib/plugin-api/src/events.rs` (remove migrated events)

**Estimated Time:** 3-4 hours

---

### Phase 5: Plugin Migration (Not Started)

**Scope:** LARGE - update all 6 plugins

**Work Required:**

For each plugin:
1. Add `capabilities()` method declaring what it needs
2. Change `build(&self, ctx: &mut PluginContext)` to `build(&self, ctx: PluginBuildContext)`
3. Update system signatures to use new APIs:
   - Replace direct `Query<&StreamWriter>` with domain APIs
   - Replace `Res<GlobalStateResource>` with `WorldQueries`
   - Replace ad-hoc queries with `EntityQueries`

**Plugins to Update:**
1. animations (140 lines)
2. blocks (200 lines)
3. chat (90 lines)
4. inventory (80 lines)
5. default-commands (60 lines)
6. hello (60 lines)

**Example Migration:**

**Before:**
```rust
fn build(&self, ctx: &mut PluginContext<'_>) {
    ctx.add_tick_system(my_system);
}

fn my_system(
    state: Res<GlobalStateResource>,  // ❌ Direct state access
    mut conn: Query<&StreamWriter>,   // ❌ Direct network access
) {
    // ...
}
```

**After:**
```rust
fn capabilities(&self) -> PluginCapabilities {
    PluginCapabilities::builder()
        .with_animation_api()
        .with_entity_queries()
        .build()
}

fn build(&self, mut ctx: PluginBuildContext<'_>) {
    ctx.systems().add_tick(my_system);
}

fn my_system(
    mut animations: AnimationAPI,     // ✅ Domain API
    entities: EntityQueries,          // ✅ Safe queries
) {
    // ...
}
```

**Estimated Time:** 2-3 hours

---

### Phase 6: Enforcement (Not Started)

**Scope:** MEDIUM - breaking changes

**Work Required:**

1. **Remove Old Access from PluginContext**
   ```rust
   // Remove these fields:
   // pub world: &'a mut World,
   // pub state: GlobalState,
   ```

2. **Mark PluginContext as Deprecated**
   ```rust
   #[deprecated(note = "Use PluginBuildContext instead")]
   pub struct PluginContext<'a> { ... }
   ```

3. **Add Capability Validation (Optional)**
   - Runtime checks that system params match capabilities
   - Helps catch mistakes during development

**Files to Modify:**
- `src/lib/plugin-api/src/context.rs`
- `src/bin/src/plugin_loader.rs` (validation)

**Estimated Time:** 1 hour

---

### Phase 7: Testing & Validation (Not Started)

**Scope:** SMALL - verification

**Work Required:**

1. **Compilation Tests**
   ```bash
   cargo check
   cargo build
   ```

2. **Runtime Tests**
   ```bash
   cargo test
   ./target/debug/ferrumc
   ```

3. **Functionality Tests**
   - Place/break blocks
   - Send chat messages
   - Inventory operations
   - Animation playback

4. **Enforcement Tests**
   - Try to access StreamWriter (should fail)
   - Try to access GlobalState directly (should fail)
   - Verify capabilities are checked

**Estimated Time:** 1 hour

---

## Summary

### Completed
- ✅ **Phase 1:** Capability System (2 hours)
- ✅ **Phase 2:** Build Context (1 hour)
- ✅ **Phase 3:** Query System (2 hours)

**Total Completed:** 5 hours of foundation work

### Remaining
- 🚧 **Phase 4:** Domain API Consolidation (3-4 hours)
- 🚧 **Phase 5:** Plugin Migration (2-3 hours)
- 🚧 **Phase 6:** Enforcement (1 hour)
- 🚧 **Phase 7:** Testing (1 hour)

**Total Remaining:** 7-9 hours of implementation work

---

## Current State

### What Works ✅
- Capability system is ready to use
- PluginBuildContext is ready to use
- EntityQueries and WorldQueries are ready to use
- Old system still works (backward compatible)
- Code compiles successfully

### What's Next 🔄
1. Move events to domain API crates
2. Update domain API traits to prevent bypassing
3. Migrate one plugin as proof of concept
4. Migrate remaining plugins
5. Remove old PluginContext
6. Test everything

### Migration Strategy
- **Incremental:** Can migrate plugins one by one
- **Backward Compatible:** Old plugins still work during migration
- **Safe:** Can rollback at any point before Phase 6

---

## How to Continue

### Option 1: Continue Full Implementation
Continue with Phases 4-7 to complete the redesign.

### Option 2: Proof of Concept
Migrate just ONE plugin (animations) to validate the approach:
1. Add capabilities() to AnimationsPlugin
2. Update to use PluginBuildContext
3. Test it works
4. Decide whether to proceed with full migration

### Option 3: Document and Defer
- Document current state
- Create migration guide for future work
- Use new APIs for NEW plugins only
- Gradually migrate old plugins over time

---

## Recommendation

**Proceed with Option 2: Proof of Concept**

Reason:
- Validates the entire approach works
- Low risk (one plugin)
- Can test real functionality
- Informs remaining work
- Takes ~1 hour

If proof of concept succeeds → Continue with full migration  
If issues found → Adjust design before migrating all plugins

---

**Next Command:** Migrate animations plugin as proof of concept
