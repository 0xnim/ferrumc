# Real Enforcement - Implementation Results

## Summary

✅ **REAL enforcement is now implemented.** Plugins literally CANNOT access I/O because the types aren't in scope.

## What We Implemented

### 1. Safe Plugin Prelude (`src/lib/plugin-api/src/prelude.rs`)
- Only re-exports safe types: `Entity`, `EventReader`, `EventWriter`
- Does NOT re-export: `Query`, `Res`, `ResMut`, `World`, `App`, `Commands`
- Plugins must use `use ferrumc_plugin_api::prelude::*` and nothing else

### 2. Safe Query APIs
- `EntityQueries`: Read-only access to Position, Rotation, PlayerIdentity
- `InventoryQueries`: Read access to inventory/hotbar
- `InventoryQueriesMut`: Write access to inventory/hotbar (requires capability)
- `WorldQueries`: Safe world data queries (future)

### 3. Removed bevy_ecs Dependency from ALL Plugins
Plugins that were updated:
- `ferrumc-plugin-animations`
- `ferrumc-plugin-blocks`
- `ferrumc-plugin-chat`
- `ferrumc-plugin-inventory`
- `ferrumc-default-commands`
- `ferrumc-plugin-hello`

### 4. Restricted EventRegistry
Removed unsafe methods:
- ❌ `insert_resource()` - allowed arbitrary resource injection
- ❌ `register_component()` - allowed arbitrary component registration
- ✅ `register::<E: Event>()` - only allows event registration (safe)

### 5. Dependency Boundaries
**Plugins can only depend on:**
- `ferrumc-plugin-api` (safe APIs only)
- Domain APIs (`ferrumc-animation-api`, `ferrumc-block-api`, etc.)
- Standard Rust crates

**Plugins CANNOT depend on:**
- ❌ `bevy_ecs` (not in their Cargo.toml)
- ❌ `ferrumc-net` (not in their Cargo.toml)
- ❌ `ferrumc-storage` (not in their Cargo.toml)
- ❌ `ferrumc-state` (not in their Cargo.toml)

## Enforcement Tests

Created `src/lib/plugins/test_violations/` to verify enforcement.

### Test 1: Cannot Import StreamWriter
```rust
use ferrumc_net::connection::StreamWriter;  // ❌ FAILS
```

**Result:**
```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `ferrumc_net`
```

✅ **PASS** - Plugins literally cannot import ferrumc_net

### Test 2: Cannot Import bevy_ecs Query
```rust
use bevy_ecs::system::Query;  // ❌ FAILS
```

**Result:**
```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `bevy_ecs`
```

✅ **PASS** - Plugins literally cannot import bevy_ecs

### Test 3: Safe Plugin Compiles Fine
```rust
use ferrumc_plugin_api::prelude::*;

fn my_system(entities: EntityQueries, inventories: InventoryQueries) {
    // This compiles fine - using only safe APIs
}
```

✅ **PASS** - Safe APIs work perfectly

## How It Works

### Compile-Time Enforcement

1. **Missing Dependencies**: Plugins don't have `bevy_ecs`, `ferrumc-net`, or `ferrumc-storage` in Cargo.toml
2. **Limited Prelude**: Only safe types are re-exported in prelude
3. **Type System**: Rust's type checker prevents accessing types that aren't in scope

### Example: Why Plugins Can't Access Network

```rust
// ❌ Plugin tries to do I/O:
use ferrumc_net::connection::StreamWriter;  // Compile error: unresolved crate

fn my_system(query: Query<&StreamWriter>) {  // Compile error: Query not in scope
    // Can't even get here!
}
```

**Compile errors:**
1. `ferrumc_net` is not a dependency
2. `Query` is not re-exported in prelude
3. Even if they hacked around (1) and (2), `StreamWriter` is a component they can't query

### Example: What Plugins CAN Do

```rust
use ferrumc_plugin_api::prelude::*;
use ferrumc_block_api::BlockAPI;

fn my_system(
    entities: EntityQueries,        // ✅ Safe query
    inventories: InventoryQueries,  // ✅ Safe query
    mut blocks: BlockAPI,           // ✅ Domain API
) {
    // Access safe data
    let pos = entities.position(player);
    let item = inventories.selected_item(player);
    
    // Request actions via API (core handles I/O)
    blocks.place_block(player, pos, block_id, sequence);
}
```

## What Core Can Still Do

Core modules (src/bin, src/lib/core-systems, src/lib/net) still have full access:
- ✅ Can import `StreamWriter`
- ✅ Can use raw `Query<&StreamWriter>`
- ✅ Can access database
- ✅ Can do network I/O

This is correct - core needs I/O access, plugins don't.

## Migration Status

All existing plugins have been migrated:

| Plugin | Status | Safe APIs Used |
|--------|--------|----------------|
| animations | ✅ Migrated | AnimationAPI |
| blocks | ✅ Migrated | BlockAPI, EntityQueries, InventoryQueries |
| chat | ✅ Migrated | ChatAPI |
| inventory | ✅ Migrated | InventoryQueriesMut |
| default-commands | ✅ Migrated | CommandsAPI |
| hello (example) | ✅ Migrated | Prelude only |

## Conclusion

This is NOT just a nicer API - this is REAL enforcement:

1. ✅ Plugins cannot import I/O types (compile error)
2. ✅ Plugins cannot use raw Query (not in scope)
3. ✅ Plugins cannot inject arbitrary resources (method removed)
4. ✅ Plugins can only use safe domain APIs
5. ✅ Violation attempts fail at compile time

The architecture is now **safe by construction** - it's impossible for plugins to violate boundaries without modifying their Cargo.toml to add forbidden dependencies (which we can detect in code review).
