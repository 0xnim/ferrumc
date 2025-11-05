# Plugin Architecture Review

**Date:** November 5, 2025  
**Reviewed By:** AI Assistant  
**Plugins Reviewed:** 6 core plugins, 1 example plugin

---

## Summary

Overall, the plugins follow the documented architecture **well**, with a few minor issues that need attention.

**Score: 8/10** - Good adherence to principles with room for improvement.

---

## Plugin-by-Plugin Analysis

### ✅ **AnimationsPlugin** - EXCELLENT

**Location:** `src/lib/plugins/core/animations/`

**Strengths:**
- ✅ Perfect separation: No I/O, only game logic
- ✅ Uses domain API (`AnimationAPI`) correctly
- ✅ Clear documentation explaining flow
- ✅ Clean event handling
- ✅ Proper use of `register_events!` macro

**Issues:**
- ❌ **Missing `priority()`** - Should set priority for proper ordering
- ⚠️ Uses `AnimationAPI` as system parameter - needs verification this works

**Recommended Changes:**
```rust
impl Plugin for AnimationsPlugin {
    fn priority(&self) -> i32 {
        50  // Base animation logic
    }
    
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]  // Document: no dependencies
    }
}
```

---

### ⚠️ **BlocksPlugin** - GOOD (with violations)

**Location:** `src/lib/plugins/core/blocks/`

**Strengths:**
- ✅ Uses domain API (`BlockAPI`) correctly
- ✅ Clear separation of concerns in code structure
- ✅ Good validation logic

**Issues:**
- ❌ **CRITICAL: Direct I/O violation** - Plugin directly accesses `state.0.world.load_chunk_owned()`
- ❌ **CRITICAL: Direct database access** - Plugin calls `state.0.world.save_chunk()`
- ❌ **Missing `priority()`** - Should set priority
- ❌ Plugin does chunk generation: `state.0.terrain_generator.generate_chunk()` (should be core)
- ⚠️ Uses `BlockAPI` as system parameter - needs verification

**Why this is wrong:**
According to PLUGIN_VS_CORE.md:
> Core = Infrastructure (networking, database, world)  
> Plugins = Gameplay features (movement rules, combat, commands)

**Recommended Fix:**
1. Move chunk loading/saving to core-systems
2. Create events: `RequestChunkLoad`, `ChunkLoadedResponse`
3. Create BlockAPI methods: `load_chunk()`, `save_chunk()`
4. Plugin calls API, core handles I/O

**Correct Flow:**
```
Plugin reads BlockPlaceAttemptEvent
  → Plugin validates placement (game logic)
  → Plugin calls blocks.place_block(position, block_id)
  → Core-systems loads chunk (I/O)
  → Core-systems modifies chunk (I/O)  
  → Core-systems saves chunk (I/O)
  → Core-systems broadcasts update (I/O)
```

**Recommended Changes:**
```rust
impl Plugin for BlocksPlugin {
    fn priority(&self) -> i32 {
        40  // Block placement validation
    }
}

// In system:
fn handle_block_placement(
    mut events: EventReader<BlockPlaceAttemptEvent>,
    mut blocks: BlockAPI,
    // Remove: state, chunk loading, chunk saving
) {
    for event in events.read() {
        // Validate placement (game logic only)
        if !validate_placement(event) {
            event.cancel("Invalid placement");
            continue;
        }
        
        // Use API to place block (core handles I/O)
        blocks.place_block(event.position, event.block_id);
    }
}
```

---

### ✅ **ChatPlugin** - EXCELLENT

**Location:** `src/lib/plugins/core/chat/`

**Strengths:**
- ✅ Perfect separation: No I/O, only game logic
- ✅ Uses domain API (`ChatAPI`) correctly
- ✅ Clear, simple implementation
- ✅ Good documentation

**Issues:**
- ❌ **Missing `priority()`** - Should set priority
- ⚠️ Uses `ChatAPI` as system parameter - needs verification

**Recommended Changes:**
```rust
impl Plugin for ChatPlugin {
    fn priority(&self) -> i32 {
        30  // Chat formatting (modifier)
    }
}
```

---

### ⚠️ **InventoryPlugin** - NEEDS IMPROVEMENT

**Location:** `src/lib/plugins/core/inventory/`

**Strengths:**
- ✅ No direct packet/network access
- ✅ Clear documentation

**Issues:**
- ❌ **Direct state access** - Uses `state.0.players.is_connected(event.player)` (should be API)
- ❌ **Missing `priority()`** - Should set priority
- ⚠️ **Questionable game logic** - Lines 102-118 set example item in slot 6, seems like test code
- ❌ **Missing InventoryAPI usage** - Should use `InventoryAPI` trait instead of direct component mutation

**Recommended Fix:**
1. Create `InventoryAPI::is_player_connected()` method
2. Remove test code (lines 102-118)
3. Use InventoryAPI trait for all inventory operations

**Recommended Changes:**
```rust
impl Plugin for InventoryPlugin {
    fn priority(&self) -> i32 {
        40  // Inventory management
    }
}

// Remove this test code:
// Lines 102-118 (setting example item in slot 6)
```

---

### ⚠️ **DefaultCommandsPlugin** - ACCEPTABLE (temporary bridge)

**Location:** `src/lib/plugins/core/default-commands/`

**Strengths:**
- ✅ Well-documented as temporary bridge
- ✅ Clear migration path documented
- ✅ Simple, focused implementation

**Issues:**
- ⚠️ Accesses global queue `ferrumc_core::mq::QUEUE` (documented as temporary)
- ❌ **Missing `priority()`** - Should set priority

**Notes:**
- This is a **bridge plugin** during migration
- Intentionally violates some principles for compatibility
- Should be removed once commands use ChatAPI directly
- Acceptable as temporary solution

**Recommended Changes:**
```rust
impl Plugin for DefaultCommandsPlugin {
    fn priority(&self) -> i32 {
        10  // Run after other chat systems
    }
    
    fn description(&self) -> &'static str {
        "Bridges legacy command responses to the chat API (TEMPORARY - will be removed)"
    }
}
```

---

### ✅ **HelloPlugin** - EXCELLENT (Example)

**Location:** `src/lib/plugins/examples/hello/`

**Strengths:**
- ✅ Perfect example for documentation
- ✅ Shows configuration usage
- ✅ Shows timed systems
- ✅ Includes tests!
- ✅ Clean, simple code

**Issues:**
- ❌ **Missing `priority()`** - Should add for completeness (even if default=0)

**Recommended Changes:**
```rust
impl Plugin for HelloPlugin {
    fn priority(&self) -> i32 {
        0  // Example plugin, no specific priority needed
    }
}
```

---

## Critical Issues Summary

### 🔴 **CRITICAL: BlocksPlugin I/O Violations**

The BlocksPlugin directly violates the core principle:
> **Core handles I/O. Plugin handles logic.**

**Current violations:**
1. Direct chunk loading: `state.0.world.load_chunk_owned()`
2. Direct chunk saving: `state.0.world.save_chunk()`
3. Direct world generation: `state.0.terrain_generator.generate_chunk()`

**Impact:**
- Breaks architectural separation
- Makes plugin unable to work without direct world access
- Defeats purpose of domain APIs
- Future plugins will copy this anti-pattern

**Priority:** HIGH - Fix immediately

---

### 🟡 **MEDIUM: Missing Priority on All Plugins**

**Issue:** None of the plugins set `priority()`, yet documentation extensively covers it.

**Impact:**
- System execution order is undefined
- Multi-plugin interactions unpredictable
- Documentation doesn't match implementation

**Priority:** MEDIUM - Add to all plugins

**Recommended Priorities:**
```
Anti-cheat (future):     100+
Animations:               50 (base system)
Blocks:                   40 (validation/placement)
Inventory:                40 (management)
Chat:                     30 (formatting)
Default Commands:         10 (bridge, runs late)
Health (future):           0 (final processing)
Hello (example):           0 (no specific order needed)
```

---

### 🟡 **MEDIUM: API as System Parameters**

**Issue:** Plugins use `AnimationAPI`, `BlockAPI`, `ChatAPI` as system parameters.

**Need to verify:**
- Does this actually work with Bevy ECS?
- Or should it be `mut animations: ResMut<AnimationAPI>`?
- Or `world: &mut World` with trait methods?

**Check implementation in:**
- `src/lib/apis/*/src/traits.rs` - How are traits implemented?

---

### 🟢 **LOW: InventoryPlugin Test Code**

**Issue:** Lines 102-118 in InventoryPlugin set example item in slot 6.

**Priority:** LOW - Remove when cleaning up

---

## Compliance Scorecard

| Plugin | Separation | API Usage | Priority | Dependencies | Documentation | Score |
|--------|-----------|-----------|----------|--------------|---------------|-------|
| **Animations** | ✅ | ✅ | ❌ | ⚠️ | ✅ | 8/10 |
| **Blocks** | ❌ | ⚠️ | ❌ | ⚠️ | ✅ | 4/10 |
| **Chat** | ✅ | ✅ | ❌ | ⚠️ | ✅ | 8/10 |
| **Inventory** | ⚠️ | ❌ | ❌ | ⚠️ | ✅ | 5/10 |
| **Commands** | ⚠️ | ✅ | ❌ | ⚠️ | ✅ | 6/10 |
| **Hello** | ✅ | N/A | ❌ | ⚠️ | ✅ | 8/10 |

**Legend:**
- ✅ Compliant
- ⚠️ Partially compliant / Needs verification
- ❌ Non-compliant
- N/A Not applicable

---

## Recommended Action Items

### Immediate (P0)
1. ✅ **Fix BlocksPlugin I/O violations**
   - Move chunk loading to core-systems
   - Move chunk saving to core-systems
   - Move world generation calls to core
   - Use BlockAPI for all operations

### High Priority (P1)
2. ✅ **Add priority() to all plugins**
   - Animations: 50
   - Blocks: 40
   - Inventory: 40
   - Chat: 30
   - Commands: 10
   - Hello: 0

3. ✅ **Verify API system parameters work**
   - Check trait implementations
   - Ensure Bevy ECS compatibility
   - Update if needed

### Medium Priority (P2)
4. ⚠️ **Add dependencies() to all plugins**
   - Document dependency relationships
   - Ensure proper load order

5. ⚠️ **Fix InventoryPlugin state access**
   - Create InventoryAPI methods
   - Remove direct state access

### Low Priority (P3)
6. 🟢 **Clean up InventoryPlugin test code**
7. 🟢 **Update documentation examples** to match actual implementations

---

## Pattern Observations

### Good Patterns Being Used ✅
1. **Clear plugin structure** - All plugins follow same layout
2. **Good documentation** - Most have clear header comments
3. **Consistent naming** - `handle_*` for system functions
4. **Event registration** - Using `register_events!` macro correctly
5. **Separation in intent** - Plugins *try* to separate concerns

### Anti-Patterns to Avoid ❌
1. **Direct state access** - Blocks and Inventory plugins
2. **Missing priority** - All plugins
3. **I/O in plugins** - BlocksPlugin chunk operations
4. **Undocumented dependencies** - No plugins declare dependencies

---

## Conclusion

The plugins are **structurally sound** but need refinement to fully match the documented architecture. The BlocksPlugin is the primary concern as it sets a bad precedent for future plugin development.

**Overall Grade: B-** (80/100)

**Next Steps:**
1. Fix BlocksPlugin I/O violations (critical)
2. Add priority to all plugins (high)
3. Verify API system parameters work (high)
4. Document dependencies (medium)
5. Clean up minor issues (low)

Once these issues are addressed, the plugin system will be a **strong foundation** for future development.
