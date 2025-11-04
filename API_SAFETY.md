# API Safety - Event Registration

## The Problem

When using Bevy ECS `SystemParam` APIs like `BlockAPI` or `ChatAPI`, the underlying events must be registered in the World. If they're not registered, you get a **runtime panic**:

```
Parameter `BlockAPI<'_>::update_events` failed validation: Event not initialized
```

This happens at **runtime**, not compile time, making it easy to miss during development.

## The Solution

We provide registration macros for each API that automatically register all required events:

### Block API

```rust
use ferrumc_block_api::register_block_api_events;
use ferrumc_plugin_api::{Plugin, PluginContext};

impl Plugin for MyPlugin {
    fn build(&self, ctx: &mut PluginContext) {
        // Register all events required by BlockAPI
        register_block_api_events!(ctx);
        
        // Now safe to use BlockAPI in systems
        ctx.add_tick_system(my_system);
    }
}

fn my_system(mut blocks: BlockAPI) {
    // This will work without panicking
    blocks.broadcast_block_update(position, block_state);
}
```

### Chat API

```rust
use ferrumc_chat_api::register_chat_api_events;

impl Plugin for MyPlugin {
    fn build(&self, ctx: &mut PluginContext) {
        register_chat_api_events!(ctx);
        ctx.add_tick_system(my_system);
    }
}
```

### Animation API

```rust
use ferrumc_animation_api::register_animation_api_events;

impl Plugin for MyPlugin {
    fn build(&self, ctx: &mut PluginContext) {
        register_animation_api_events!(ctx);
        ctx.add_tick_system(my_system);
    }
}
```

## What Gets Registered

Each macro registers all events required by that API:

| API | Events Registered |
|-----|-------------------|
| `register_block_api_events!` | `BlockPlaceAttemptEvent`, `BlockBreakAttemptEvent`, `SendBlockUpdateRequest`, `SendBlockChangeAckRequest` |
| `register_chat_api_events!` | `ChatMessageEvent`, `SendChatMessageRequest` |
| `register_animation_api_events!` | `PlayerSwingArmEvent`, `PlayerCommandEvent`, `PlayAnimationRequest`, `SetEntityPoseRequest` |

## Why This Helps

1. **Single Source of Truth** - The API crate defines which events it needs
2. **Harder to Forget** - One macro call instead of listing 4+ events
3. **Easier to Maintain** - When the API adds new events, the macro is updated automatically
4. **Self-Documenting** - Clear what you need to call to use each API
5. **Catches Errors Earlier** - While still runtime, it fails during plugin init, not during gameplay

## Can We Make It Compile-Time?

Unfortunately, no. Bevy ECS event registration is inherently runtime because:

1. The World is constructed at runtime
2. Events are registered via `world.init_resource::<Events<T>>()`
3. Rust's type system can't enforce "resource X must exist in world Y"

The best we can do is:
- ✅ Make it easy to register correctly (via macros)
- ✅ Fail fast (during plugin init, not during gameplay)
- ✅ Clear error messages
- ✅ Good documentation

## Migration Guide

If you have code like this:

```rust
// OLD - Manual registration
register_events!(
    ctx,
    BlockPlaceAttemptEvent,
    BlockBreakAttemptEvent,
    SendBlockUpdateRequest,
    SendBlockChangeAckRequest
);
```

Change to:

```rust
// NEW - Use API macro
register_block_api_events!(ctx);
```

Benefits:
- Fewer imports needed
- Can't forget an event
- Easier to read
- Automatically stays up-to-date with API changes
