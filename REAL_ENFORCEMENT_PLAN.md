# Real Plugin API Enforcement - Implementation Plan

**Goal:** Actual compile-time enforcement, not just a nicer API

---

## Problems to Fix

### 1. Plugins Can Import Anything
```rust
// ❌ Currently possible:
use bevy_ecs::prelude::Query;              // Can query anything
use ferrumc_net::connection::StreamWriter; // Can access network
```

### 2. Plugins Use Raw Query
```rust
// ❌ Blocks plugin does this:
fn system(query: Query<(&Inventory, &Hotbar)>) {
    // Nothing stops: Query<&StreamWriter>
}
```

### 3. EventRegistry Exposes World
```rust
pub fn insert_resource<R: Resource>(&mut self, resource: R) {
    self.world.insert_resource(resource);  // ❌ Can insert ANYTHING
}
```

---

## Solution: Real Enforcement

### Step 1: Create Plugin Prelude (Safe Re-exports)
Create `plugin-api/src/prelude.rs` that re-exports ONLY safe types:
- EventReader, EventWriter (for specific events)
- Safe SystemParams (EntityQueries, WorldQueries, domain APIs)
- NO Query, NO Res, NO ResMut, NO World

### Step 2: Remove bevy_ecs from Plugin Dependencies
Plugins should NOT depend on bevy_ecs directly. Only plugin-api.

### Step 3: Create Safe Component Access
Instead of raw Query, provide typed accessors:
- `EntityQueries` already does this
- Need similar for Inventory, Hotbar, etc.

### Step 4: Restrict EventRegistry
Remove `insert_resource` and `register_component` - only allow events.

### Step 5: Hide Sensitive Types
Make StreamWriter, database types `pub(crate)` or module-private.

### Step 6: Validate Plugin System Params
Systems can only use:
- EventReader<T> (from domain APIs)
- EventWriter<T> (from domain APIs)
- Domain API SystemParams (AnimationAPI, BlockAPI, etc.)
- EntityQueries, WorldQueries
- NO raw Query, NO Res<GlobalState>, NO World

---

## Implementation Steps

1. Create plugin prelude with safe re-exports
2. Create InventoryQueries for safe inventory access
3. Remove bevy_ecs from plugin Cargo.toml files
4. Update plugins to use only safe APIs
5. Remove insert_resource from EventRegistry
6. Make sensitive types non-public
7. Test that violations don't compile

---

## Expected Result

```rust
// ❌ Won't compile - not in scope
use bevy_ecs::prelude::Query;
use ferrumc_net::connection::StreamWriter;

// ✅ Only safe APIs available
use ferrumc_plugin_api::prelude::*;

fn my_system(
    entities: EntityQueries,  // ✅ Safe
    api: AnimationAPI,        // ✅ Safe
    query: Query<&Inventory>, // ❌ Query not in scope!
) { }
```
