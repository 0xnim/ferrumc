# FerrumC Plugin System - Quick Start Guide

**Learn how to create plugins for FerrumC using the domain API architecture**

**See also:**
- [PLUGIN_ARCHITECTURE.md](PLUGIN_ARCHITECTURE.md) - Comprehensive architecture guide
- [PLUGIN_VS_CORE.md](PLUGIN_VS_CORE.md) - Understanding core vs plugins vs APIs

---

## Quick Overview

FerrumC uses a three-layer plugin architecture:

1. **Core** - Handles all I/O (packets, network, database)
2. **Domain APIs** - Define events, traits, and types for gameplay features
3. **Plugins** - Implement game logic using domain APIs

**Plugins never directly access packets, network, or database!**

---

## Creating Your First Plugin

### Step 1: Choose Your Domain

First, identify which domain API your plugin will use:

- **animation-api** - Play entity animations
- **block-api** - Block placement/breaking
- **chat-api** - Chat messaging
- **entity-api** - Entity tracking and management
- **inventory-api** - Inventory operations
- **movement-api** - Player movement

For this guide, we'll create an **animations** plugin.

### Step 2: Create Plugin Crate

```bash
cd src/lib/plugins/core/
mkdir -p animations/src
cd animations
```

**Create `Cargo.toml`:**
```toml
[package]
name = "ferrumc-plugin-animations"
version = "0.1.0"
edition = "2021"

[dependencies]
ferrumc-plugin-api = { path = "../../../plugin-api" }
ferrumc-animation-api = { path = "../../../apis/animation-api" }
ferrumc-core = { workspace = true }
bevy_ecs = { workspace = true }
tracing = { workspace = true }

[lints]
workspace = true
```

### Step 3: Implement Plugin Trait

**Create `src/lib.rs`:**
```rust
use ferrumc_plugin_api::*;
use ferrumc_animation_api::*;
use bevy_ecs::prelude::*;
use tracing::info;

#[derive(Default)]
pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn name(&self) -> &'static str {
        "animations"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Handles entity animations like arm swings and damage effects"
    }

    fn build(&self, ctx: &mut PluginContext<'_>) {
        info!("Loading animations plugin");

        // Register events from the domain API
        ctx.register_event::<PlayerSwingArmEvent>();
        ctx.register_event::<PlayAnimationRequest>();

        // Register our gameplay logic systems
        ctx.add_tick_system(handle_player_swings);

        info!("Animations plugin loaded successfully");
    }
}

/// System: When player swings arm, trigger animation
fn handle_player_swings(
    mut events: EventReader<PlayerSwingArmEvent>,
    world: &mut World,
) {
    for event in events.read() {
        // Game logic: Which animation should play?
        let animation = match event.hand {
            Hand::Main => AnimationType::SwingMainArm,
            Hand::Off => AnimationType::SwingOffhand,
        };

        // Use the domain API to request animation
        world.play_animation(event.player, animation);
    }
}
```

### Step 4: Register Plugin

**Update `src/bin/src/plugin_loader.rs`:**
```rust
pub fn create_plugin_registry() -> Result<PluginRegistry, PluginError> {
    let mut registry = PluginRegistry::new();
    registry.load_config("plugins.toml")?;

    // Register your plugin here
    registry.register::<ferrumc_plugin_animations::AnimationsPlugin>();

    // Other plugins...
    registry.register::<ferrumc_plugin_hello::HelloPlugin>();

    registry.sort_by_dependencies()?;
    Ok(registry)
}
```

**Update `src/bin/Cargo.toml`:**
```toml
[dependencies]
# ... other dependencies ...
ferrumc-plugin-animations = { path = "../lib/plugins/core/animations" }
```

### Step 5: Configure Plugin (Optional)

**Add to `plugins.toml`:**
```toml
[animations]
enabled = true
# Plugin-specific config here
```

Access config in your plugin:
```rust
fn build(&self, ctx: &mut PluginContext<'_>) {
    let enabled = ctx.get_config::<bool>("enabled").unwrap_or(true);
    if !enabled {
        return; // Don't register systems
    }
    
    // ... register systems ...
}
```

### Step 6: Build and Test

```bash
# Build the server
cargo build

# Run the server
./target/debug/ferrumc

# Check logs for:
# [INFO] Loading plugin: animations v1.0.0 by FerrumC Team
# [INFO] Loading animations plugin
# [INFO] Animations plugin loaded successfully
```

---

## Understanding Domain APIs

### What is a Domain API?

A domain API is a crate that defines the **contract** between core infrastructure and plugins for a specific gameplay domain.

**Example: `ferrumc-animation-api`**

```
src/lib/apis/animation-api/
├── src/
│   ├── lib.rs       # Re-exports
│   ├── types.rs     # AnimationType, Hand
│   ├── events.rs    # PlayerSwingArmEvent, PlayAnimationRequest
│   └── traits.rs    # AnimationAPI trait
└── Cargo.toml
```

### Domain API Components

#### 1. Types (Domain Models)

```rust
// src/lib/apis/animation-api/src/types.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    SwingMainArm,
    SwingOffhand,
    TakeDamage,
    LeaveBed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Main,
    Off,
}
```

#### 2. Events (High-Level Communication)

```rust
// src/lib/apis/animation-api/src/events.rs

use bevy_ecs::prelude::*;
use super::types::*;

/// Emitted when player swings their arm (from packet handler in core)
#[derive(Event, Clone)]
pub struct PlayerSwingArmEvent {
    pub player: Entity,
    pub hand: Hand,
}

/// Request to play an animation (sent by plugin, handled by core)
#[derive(Event, Clone)]
pub struct PlayAnimationRequest {
    pub entity: Entity,
    pub animation: AnimationType,
    pub broadcast_to_all: bool,
}
```

#### 3. Traits (Plugin APIs)

```rust
// src/lib/apis/animation-api/src/traits.rs

use bevy_ecs::prelude::*;
use super::events::*;
use super::types::*;

/// API for triggering animations
pub trait AnimationAPI {
    /// Play animation visible to nearby players
    fn play_animation(&mut self, entity: Entity, animation: AnimationType);
    
    /// Play animation visible to all players
    fn play_animation_global(&mut self, entity: Entity, animation: AnimationType);
}

impl AnimationAPI for World {
    fn play_animation(&mut self, entity: Entity, animation: AnimationType) {
        self.send_event(PlayAnimationRequest {
            entity,
            animation,
            broadcast_to_all: false,
        });
    }
    
    fn play_animation_global(&mut self, entity: Entity, animation: AnimationType) {
        self.send_event(PlayAnimationRequest {
            entity,
            animation,
            broadcast_to_all: true,
        });
    }
}
```

---

## Event Flow

Understanding how events flow through the system is crucial:

```
1. Player Action
   ↓
2. Network Layer (Core)
   Receives SwingArmPacket
   ↓
3. Packet Handler (Core)
   pub fn handle_swing_arm_packets(
       packets: Res<SwingArmPacketReceiver>,
       mut events: EventWriter<PlayerSwingArmEvent>,
   )
   Converts packet → PlayerSwingArmEvent
   ↓
4. Plugin System
   fn handle_player_swings(
       mut events: EventReader<PlayerSwingArmEvent>,
       world: &mut World,
   )
   Reads event, applies game logic
   Emits PlayAnimationRequest via world.play_animation()
   ↓
5. Broadcast System (Core)
   pub fn broadcast_animations(
       mut requests: EventReader<PlayAnimationRequest>,
       conn_query: Query<&StreamWriter>,
   )
   Reads PlayAnimationRequest
   Sends EntityAnimationPacket to clients
   ↓
6. Network Layer (Core)
   Sends packets to players
```

**Key principle:** Plugins sit in the middle, reading high-level events and emitting high-level requests. Core handles all I/O.

---

## Common Patterns

### Pattern 1: Event Listener

Listen to events from core and react:

```rust
fn my_system(
    mut events: EventReader<SomeEvent>,
) {
    for event in events.read() {
        // React to event
    }
}
```

### Pattern 2: API User

Use domain API traits to request actions:

```rust
fn my_system(
    world: &mut World,
    events: EventReader<PlayerSwingArmEvent>,
) {
    for event in events.read() {
        // Use API
        world.play_animation(event.player, AnimationType::SwingMainArm);
    }
}
```

### Pattern 3: Cancellable Events

Some events can be cancelled by plugins:

```rust
fn validate_placement(
    mut events: EventReader<BlockPlaceAttempt>,
) {
    for event in events.read() {
        if !is_valid(event) {
            event.cancel("Invalid placement");
        }
    }
}

fn handle_placement(
    mut events: EventReader<BlockPlaceAttempt>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        if event.is_cancelled() {
            continue;  // Skip cancelled events
        }
        
        // Process event
        state.0.set_block(event.position, event.block);
    }
}
```

### Pattern 4: Multiple Event Listeners

Plugins can listen to multiple event types:

```rust
impl Plugin for MyPlugin {
    fn build(&self, ctx: &mut PluginContext<'_>) {
        ctx.register_event::<EventA>();
        ctx.register_event::<EventB>();
        ctx.register_event::<EventC>();
        
        ctx.add_tick_system(handle_event_a);
        ctx.add_tick_system(handle_event_b);
        ctx.add_tick_system(handle_event_c);
    }
}
```

### Pattern 5: Cross-Plugin Communication

Plugins communicate via events:

```rust
// Plugin A emits event
fn plugin_a_system(world: &mut World) {
    world.send_event(CustomEvent { data: 42 });
}

// Plugin B listens to event
fn plugin_b_system(mut events: EventReader<CustomEvent>) {
    for event in events.read() {
        // React to Plugin A's event
    }
}
```

---

## Plugin Lifecycle

### Lifecycle Hooks

```rust
impl Plugin for MyPlugin {
    /// Called when plugin is loaded (before build)
    fn on_load(&mut self) {
        // Initialize plugin state
    }
    
    /// Build - register systems, events, resources
    fn build(&self, ctx: &mut PluginContext<'_>) {
        // Register everything
    }
    
    /// Called when server is ready (after all plugins built)
    fn on_enable(&mut self) {
        // Server is ready, do final setup
    }
    
    /// Called when server is shutting down
    fn on_disable(&mut self) {
        // Cleanup
    }
    
    /// Called when configuration changes
    fn on_config_reload(&mut self, new_config: &PluginConfig) {
        // Update plugin based on new config
    }
}
```

### Execution Order

Control when your plugin's systems run:

```rust
impl Plugin for AntiCheatPlugin {
    fn priority(&self) -> i32 {
        100  // High priority = runs first
    }
}

impl Plugin for LoggingPlugin {
    fn priority(&self) -> i32 {
        -100  // Low priority = runs last
    }
}
```

### Plugin Dependencies

Declare dependencies on other plugins:

```rust
impl Plugin for CombatPlugin {
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["entity_tracking", "health", "animations"]
    }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        // These plugins are guaranteed to be loaded first
    }
}
```

---

## Available APIs

### Currently Implemented

✅ **plugin-api** - Base plugin system (`Plugin` trait, `PluginContext`)

### Planned Domain APIs

🚧 **animation-api** - Entity animations
🚧 **block-api** - Block placement/breaking
🚧 **chat-api** - Chat messaging
🚧 **entity-api** - Entity tracking
🚧 **inventory-api** - Inventory management
🚧 **movement-api** - Player movement

---

## Helper Macros

### `register_events!`

Register multiple events at once:

```rust
use ferrumc_plugin_api::register_events;

fn build(&self, ctx: &mut PluginContext<'_>) {
    register_events!(ctx,
        EventA,
        EventB,
        EventC,
    );
}

// Instead of:
// ctx.register_event::<EventA>();
// ctx.register_event::<EventB>();
// ctx.register_event::<EventC>();
```

---

## Best Practices

### ✅ Do

- Use domain APIs for all interactions
- Listen to events, don't poll
- Keep plugins focused on one feature
- Document your plugin's purpose
- Use descriptive event names
- Handle errors gracefully
- Write tests for your plugin logic

### ❌ Don't

- Access raw packets or network directly
- Directly manipulate database
- Create global state outside ECS
- Hardcode values (use config instead)
- Ignore cancellation flags on events
- Depend on plugin load order (use dependencies instead)

---

## Example: Complete Block Plugin

```rust
use ferrumc_plugin_api::*;
use ferrumc_block_api::*;
use bevy_ecs::prelude::*;
use tracing::{info, error};

#[derive(Default)]
pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn name(&self) -> &'static str { "blocks" }
    fn version(&self) -> &'static str { "1.0.0" }
    fn author(&self) -> &'static str { "FerrumC Team" }
    fn description(&self) -> &'static str {
        "Block placement and breaking mechanics"
    }
    
    fn build(&self, ctx: &mut PluginContext<'_>) {
        info!("Loading blocks plugin");
        
        // Register events
        register_events!(ctx,
            BlockPlaceAttempt,
            BlockBreakAttempt,
            BlockChanged,
        );
        
        // Register systems
        ctx.add_tick_system(validate_placement);
        ctx.add_tick_system(handle_placement);
        ctx.add_tick_system(handle_breaking);
        
        info!("Blocks plugin loaded");
    }
}

// Validation system (runs first)
fn validate_placement(
    mut events: EventReader<BlockPlaceAttempt>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        // Check collision
        if !state.0.can_place_block(event.position, event.player) {
            event.cancel("Collision detected");
            continue;
        }
        
        // Check permissions
        if !has_permission(event.player, event.position) {
            event.cancel("No permission");
            continue;
        }
    }
}

// Placement system (runs after validation)
fn handle_placement(
    mut events: EventReader<BlockPlaceAttempt>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        if event.is_cancelled() {
            continue;
        }
        
        // Use BlockAPI
        if let Err(e) = state.0.set_block(event.position, event.block) {
            error!("Failed to place block: {}", e);
        }
    }
}

// Breaking system
fn handle_breaking(
    mut events: EventReader<BlockBreakAttempt>,
    state: Res<GlobalStateResource>,
) {
    for event in events.read() {
        if event.is_cancelled() {
            continue;
        }
        
        // Use BlockAPI
        if let Err(e) = state.0.break_block(event.position) {
            error!("Failed to break block: {}", e);
        }
    }
}

fn has_permission(player: Entity, position: BlockPos) -> bool {
    // Permission check logic
    true
}
```

---

## Testing Your Plugin

```bash
# Build with your plugin
cargo build

# Run server
./target/debug/ferrumc

# Check logs for plugin initialization
# You should see:
# [INFO] Loading plugin: your_plugin v1.0.0 by Your Name
# [INFO] Loading your plugin
# [INFO] Your plugin loaded successfully
```

---

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=debug ./target/debug/ferrumc
```

### Add Logging to Your Plugin

```rust
use tracing::{debug, info, warn, error};

fn my_system(mut events: EventReader<MyEvent>) {
    debug!("System starting");
    
    for event in events.read() {
        info!("Processing event: {:?}", event);
        
        if some_condition {
            warn!("Warning: something unusual");
        }
        
        if let Err(e) = do_something() {
            error!("Error occurred: {}", e);
        }
    }
}
```

---

## Next Steps

1. Read [PLUGIN_ARCHITECTURE.md](PLUGIN_ARCHITECTURE.md) for in-depth architecture details
2. Read [PLUGIN_VS_CORE.md](PLUGIN_VS_CORE.md) to understand what belongs where
3. Look at example plugins in `src/lib/plugins/examples/`
4. Check [FEATURES.md](FEATURES.md) for features that need implementing

---

## Getting Help

- **Documentation:** Read PLUGIN_ARCHITECTURE.md for detailed architecture
- **Discord:** https://discord.gg/qT5J8EMjwk
- **GitHub Issues:** Open an issue for bugs or questions

---

**Happy plugin development! 🎉**
