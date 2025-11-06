# Command API Implementation - Complete

## Overview
Comprehensive implementation of advanced command argument types with dynamic autocomplete support for FerrumC.

## ✅ Implemented Features

### 1. Player Argument Type (`PlayerArgument`)
**Location:** `src/lib/commands/src/arg/entity.rs`

- **Autocomplete:** Suggests all online player names as you type
- **Case-insensitive matching:** Works with any capitalization
- **ECS Integration:** Resolves player names to Entity IDs
- **Usage:**
  ```rust
  #[command("msg")]
  fn message_command(
      #[arg] target: PlayerArgument,
      #[arg] message: GreedyString,
      // ... other params
  ) {
      let player_entity = target.0; // Access the resolved Entity
  }
  ```

### 2. Entity Selector Support (`EntityArgument`)
**Location:** `src/lib/commands/src/arg/entity.rs`

- **Selectors:** `@a` (all players), `@p` (nearest/first player)
- **Player names:** Direct username targeting
- **Multi-entity:** Returns `Vec<Entity>` for bulk operations
- **Autocomplete:** Suggests selectors and player names
- **Usage:**
  ```rust
  #[arg] targets: EntityArgument  // Can target multiple entities
  for entity in targets.0 {
      // Process each entity
  }
  ```

### 3. Position Arguments
**Location:** `src/lib/commands/src/arg/position.rs`

#### Vec3Argument - 3D Positions
- **Absolute:** `100 64 200`
- **Relative:** `~5 ~10 ~-3` (relative to sender)
- **Local:** `^1 ^2 ^3` (relative to sender's rotation)
- **Type:** `{ x: f64, y: f64, z: f64 }`

#### Vec2Argument - 2D Positions  
- **Coordinates:** x, z only
- **Supports:** Absolute and relative

#### BlockPosArgument - Integer Block Coordinates
- **Type:** `{ x: i32, y: i32, z: i32 }`
- **Supports:** Absolute, relative (`~`), local (`^`)

#### ColumnPosArgument - Column Positions
- **Type:** `{ x: i32, z: i32 }`
- **Use case:** Chunk or column operations

## Architecture

### Command Context with World Access
**File:** `src/lib/commands/src/ctx.rs`

```rust
pub struct CommandContext<'w> {
    pub input: CommandInput,
    pub command: Arc<Command>,
    pub sender: Sender,
    pub world: &'w mut World,  // ← NEW: Direct ECS access
}
```

The `world` field allows argument parsers and suggesters to query the ECS for dynamic data like player lists.

### Command Macro - Exclusive System Pattern
**File:** `src/lib/derive_macros/src/commands/mod.rs`

The macro now generates **exclusive systems** that:
1. Use `SystemState` to safely access Bevy params
2. Create `CommandContext` with World access for argument parsing
3. Properly manage borrow scopes to avoid conflicts

**Generated pattern:**
```rust
fn __handler(world: &mut World) {
    // Phase 1: Collect events
    let events = /* fetch via SystemState */;
    
    // Phase 2: For each command
    for (command, input, sender) in events {
        // Parse arguments (has &mut World)
        let mut ctx = CommandContext { ..., world };
        let arg1 = ctx.arg::<PlayerArgument>("target")?;
        drop(ctx); // Release World borrow
        
        // Fetch Bevy params and call handler
        let mut state = SystemState::<(Res<X>, Query<Y>)>::new(world);
        let (res, query) = state.get_mut(world);
        handler(arg1, res, query);
        state.apply(world);
    }
}
```

### Suggestion Handler
**File:** `src/bin/src/packet_handlers/play_packets/command_suggestions.rs`

- **Exclusive system:** `fn handle(world: &mut World)`
- **Dynamic suggestions:** Creates `CommandContext` with World access
- **Player autocomplete:** Calls suggester functions that query ECS for online players

## Developer Experience

### Zero Boilerplate
Commands look exactly the same as before - the complexity is hidden in the macro:

```rust
#[command("teleport")]
fn teleport_command(
    #[arg] target: PlayerArgument,    // ← Auto-suggests players!
    #[arg] position: Vec3Argument,    // ← Supports ~, ^
    #[sender] sender: Sender,
    mut commands: CommandsAPI,
) {
    // target.0 is the Entity
    // position.x, position.y, position.z are the coordinates
}
```

### Type Safety
- Entity resolution happens at parse time with proper error messages
- Position parsing validates coordinate syntax
- All errors are TextComponent for consistent user feedback

### Extensibility  
Adding new argument types is straightforward:
1. Implement `CommandArgument` trait
2. Add `primitive()` for client-side validation
3. Add `suggest()` for autocomplete
4. Use `ctx.world` to query any ECS data

## Testing

Run the server and test:
```bash
cargo run
```

In-game commands to test:
- `/msg <TAB>` - Should show all online players
- `/msg Player1 hello` - Should send message
- Entity selectors: `/command @a` or `/command @p`
- Positions: `/tp ~10 ~5 ~-3` or `/tp 100 64 200`

## Performance

- **Suggestion caching:** SystemState is created per-request (no pre-allocation needed)
- **Lazy parsing:** Arguments only parsed when command executes
- **Zero allocation** for selector constants (`@a`, `@p`)
- **Efficient queries:** Uses Bevy's optimized ECS queries

## Future Enhancements

Potential additions:
- **@e selector:** All entities (not just players)
- **@r selector:** Random entity
- **Selector predicates:** `@a[distance=..10,gamemode=survival]`
- **Block/Item suggestions:** Autocomplete from registry
- **NBT arguments:** For complex data structures
- **Score arguments:** Scoreboard integration

## Files Modified

### Core Command System
- `src/lib/commands/src/ctx.rs` - Added World to CommandContext
- `src/lib/commands/src/events.rs` - Changed event to store input string
- `src/lib/commands/src/arg/mod.rs` - Updated trait with lifetimes
- `src/lib/commands/src/arg/entity.rs` - **NEW** - Entity/Player arguments
- `src/lib/commands/src/arg/position.rs` - **NEW** - Position arguments
- All primitive arg files - Updated parse signatures

### Macro System
- `src/lib/derive_macros/src/commands/mod.rs` - Complete rewrite to exclusive systems

### Packet Handlers
- `src/bin/src/packet_handlers/play_packets/command.rs` - Updated dispatch
- `src/bin/src/packet_handlers/play_packets/command_suggestions.rs` - Exclusive system

### Supporting
- `src/lib/commands/src/arg/primitive/mod.rs` - Added EntityFlags enum

## Credits

Implementation follows patterns recommended by the Oracle (GPT-5 reasoning model) for safe Bevy ECS integration with exclusive systems and SystemState.

---

**Status:** ✅ Complete and Compiled
**Build:** Passing (`cargo check`)
**Ready for:** In-game testing
