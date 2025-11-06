# Bad Code Examples - Plugin System Issues

This document shows **actual bad code** found in the FerrumC plugin system during the comprehensive review.

---

## 🔴 CRITICAL ISSUE #1: EventWriter Exposed to Plugins

**File:** [src/lib/plugin-api/src/prelude.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/src/prelude.rs#L28)

```rust
// Re-export safe ECS types only
pub use bevy_ecs::entity::Entity;
pub use bevy_ecs::event::{EventReader, EventWriter};  // ❌ BAD: EventWriter allows bypassing APIs
```

**Why This Is Bad:**
- Plugins can bypass domain APIs by directly writing to request events
- Undermines the entire capability system
- No enforcement of separation of concerns

**Example of How This Gets Abused:**
```rust
// Plugin can do this instead of using AnimationAPI:
fn bad_plugin_system(mut events: EventWriter<PlayAnimationRequest>) {
    events.send(PlayAnimationRequest { ... });  // Bypasses AnimationAPI
}
```

**Fix:**
```rust
// Only re-export EventReader, not EventWriter
pub use bevy_ecs::entity::Entity;
pub use bevy_ecs::event::EventReader;  // ✅ GOOD: Plugins can only read events
```

---

## 🔴 CRITICAL ISSUE #2: Direct Network Access via EntityExt

**File:** [src/lib/plugin-api/src/entity.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/src/entity.rs#L5-L6)

```rust
use ferrumc_net::connection::StreamWriter;  // ❌ BAD: Network types in plugin API
use ferrumc_net_codec::encode::NetEncode;   // ❌ BAD: Protocol details exposed

pub trait EntityExt {
    /// Broadcast a packet to all connected players.
    fn broadcast_packet<P: NetEncode + Send>(&self, packet: &P);  // ❌ BAD
    
    fn broadcast_packet_in_range<P: NetEncode + Send>(...);       // ❌ BAD
}

impl EntityExt for World {
    fn broadcast_packet<P: NetEncode + Send>(&self, packet: &P) {
        for entity_ref in self.iter_entities() {
            if let Some(writer) = entity_ref.get::<StreamWriter>() {  // ❌ Accessing network I/O!
                let _ = writer.send_packet_ref(packet)...
            }
        }
    }
}
```

**Why This Is Bad:**
- Plugins can send **raw network packets** directly
- Violates "Core handles I/O" architecture principle
- Plugins could send malformed packets, crash clients, or bypass validation

**Currently Exposed in Prelude:**
```rust
// src/lib/plugin-api/src/lib.rs
pub use entity::EntityExt;  // ❌ BAD: Exported to all plugins
```

**Fix:** Make this `pub(crate)` and remove from exports. Core systems only.

---

## 🔴 CRITICAL ISSUE #3: Direct Database/World I/O via WorldExt

**File:** [src/lib/plugin-api/src/world.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/src/world.rs#L64-L152)

```rust
use ferrumc_state::GlobalState;  // ❌ BAD: Full state access

pub trait WorldExt {
    fn get_chunk(&self, x: i32, z: i32, dimension: &str) -> Result<Arc<Chunk>, WorldError>;
    fn set_block_at(&self, x: i32, y: i32, z: i32, block: BlockStateId) -> Result<(), WorldError>;
    fn get_block_at(&self, x: i32, y: i32, z: i32) -> Result<BlockStateId, WorldError>;
}

impl WorldExt for GlobalState {
    fn set_block_at(&self, x: i32, y: i32, z: i32, block: BlockStateId) -> Result<(), WorldError> {
        // ❌ Plugin can load chunks from database
        let mut chunk = self.world.load_chunk_owned(chunk_x, chunk_z, "overworld")
            .or_else(|_| {
                // ❌ Plugin can generate terrain
                self.terrain_generator.generate_chunk(chunk_x, chunk_z)
            })?;

        chunk.set_block(relative_x, y, relative_z, block)?;

        // ❌ Plugin can save chunks to database
        self.world.save_chunk(Arc::new(chunk))?;
        
        Ok(())
    }
}
```

**Why This Is Bad:**
- Plugins can **load/save chunks** directly from database
- Plugins can **generate terrain**
- Complete violation of "Core handles I/O, Plugins handle logic" separation
- Could corrupt world data or cause performance issues

**Currently Exposed:**
```rust
// src/lib/plugin-api/src/lib.rs
pub use world::WorldExt;  // ❌ BAD
```

**Fix:** Remove from plugin API entirely. Core systems only.

---

## 🔴 CRITICAL ISSUE #4: No Resource Capability Enforcement

**File:** [src/lib/plugin-api/src/build_context.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/src/build_context.rs#L110-L112)

```rust
/// Insert a resource into the world
pub fn insert_resource<R: Resource>(&mut self, resource: R) {
    self.events.world.insert_resource(resource);  // ❌ No capability check!
}
```

**Why This Is Bad:**
- Plugins can insert **any resource** regardless of declared capabilities
- Makes the entire `PluginCapabilities::resources` field pointless

**How It Should Work:**
```rust
pub fn insert_resource<R: Resource>(&mut self, resource: R) {
    let type_id = TypeId::of::<R>();
    
    // ✅ Check if plugin declared this resource capability
    if !self.capabilities.resources.iter().any(|cap| cap.type_id == type_id && !cap.read_only) {
        panic!(
            "Plugin tried to insert resource {} without declaring capability. \
             Add .with_resource::<{}>() to capabilities()",
            std::any::type_name::<R>(),
            std::any::type_name::<R>()
        );
    }
    
    self.events.world.insert_resource(resource);
}
```

---

## ✅ FIXED: Duplicate Events (Two Different Types!)

**File 1 (REMOVED):** [src/lib/plugin-api/src/events.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/src/events.rs#L9-L27)

```rust
/// Event emitted when a player joins the server
#[derive(Event, Debug, Clone)]
pub struct PlayerJoinEvent {
    pub player: Entity,
    pub username: String,  // ❌ Only has username
}

#[derive(Event, Debug, Clone)]
pub struct PlayerLeaveEvent {
    pub player: Entity,
    pub username: String,
    pub reason: Option<String>,
}
```

**File 2:** [src/lib/apis/join-leave-api/src/lib.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/apis/join-leave-api/src/lib.rs#L39-L62)

```rust
/// Event fired when a player joins the server
#[derive(Event, Clone)]
pub struct PlayerJoinEvent {
    pub joining_player: Entity,  // ❌ Different field name!
    pub identity: PlayerIdentity,  // ❌ Different type (full identity, not just username)!
}

#[derive(Event, Clone)]
pub struct PlayerLeaveEvent {
    pub leaving_player: Entity,  // ❌ Different field name!
    pub identity: PlayerIdentity,
    pub reason: Option<String>,
}
```

**Why This Is Bad:**
- **TWO DIFFERENT TYPES** with the same name
- Importing the wrong one causes silent bugs
- Different field names (`player` vs `joining_player`)
- Different data types (`String` vs `PlayerIdentity`)
- Massive source of confusion

**Fix Applied:**
- ✅ Removed duplicate event definitions from `plugin-api/src/events.rs`
- ✅ Updated `prelude.rs` to re-export from `ferrumc_join_leave_api`
- ✅ All plugins now use the single authoritative source

**Files Changed:**
- [src/lib/plugin-api/src/events.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/src/events.rs) - Events removed
- [src/lib/plugin-api/src/prelude.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/src/prelude.rs) - Now re-exports from join-leave-api
- [src/lib/plugin-api/Cargo.toml](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/Cargo.toml) - Added join-leave-api dependency

---

## 🔴 HIGH ISSUE #6: Dependency Ordering Bug

**File:** [src/bin/src/plugin_loader.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/bin/src/plugin_loader.rs#L61-L100)

```rust
fn sort_by_dependencies(&mut self) -> Result<(), PluginError> {
    // Create dependency graph
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut in_degree: HashMap<&str, usize> = HashMap::new();

    for plugin in &self.plugins {
        let name = plugin.name();
        graph.insert(name, plugin.dependencies());  // ❌ Wrong direction!
        in_degree.insert(name, 0);
    }

    // Calculate in-degrees
    for plugin in &self.plugins {
        for dep in plugin.dependencies() {
            *in_degree.get_mut(dep).unwrap_or(&mut 0) += 1;  // ❌ Wrong logic!
        }
    }
```

**Why This Is Bad:**

The graph is built **backwards**! Here's what happens:

```
Plugin A depends on B
Current code: graph[A] = [B]  ❌ WRONG

This means:
- in_degree[B] gets incremented (wrong!)
- B appears to depend on A
- B loads first (correct by accident)
- But the logic is inverted

Correct approach:
- graph[B] = [A] (B enables A)
- in_degree[A] += 1 (A depends on B)
- Process B first (in_degree = 0)
```

**Fix:**
```rust
// Build graph: dependency -> dependent
for plugin in &self.plugins {
    in_degree.insert(plugin.name(), plugin.dependencies().len());
    for dep in plugin.dependencies() {
        graph.entry(dep).or_default().push(plugin.name());
    }
}
```

---

## 🟡 MEDIUM ISSUE #7: Network Types in Plugin Events

**File:** [src/lib/apis/animation-api/src/events.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/apis/animation-api/src/events.rs#L2)

```rust
use ferrumc_net_codec::net_types::var_int::VarInt;  // ❌ Protocol type in plugin API

#[derive(Event, Clone, Debug)]
pub struct PlayerCommandEvent {
    pub player: Entity,
    pub command: PlayerCommand,
    pub entity_id: VarInt,      // ❌ Why do plugins need this?
    pub jump_boost: VarInt,     // ❌ Network detail leaked
}

#[derive(Event, Clone, Debug)]
pub struct SetEntityPoseRequest {
    pub entity: Entity,
    pub entity_id: VarInt,      // ❌ Plugins shouldn't know about network IDs
    pub pose: EntityPose,
    pub exclude_player: Option<Entity>,
}
```

**Why This Is Bad:**
- `VarInt` is a **network protocol detail**
- Plugins work with game logic, not protocol encoding
- Makes testing harder (need to construct protocol types)
- Coupling to Minecraft protocol version

**Fix:**
```rust
// Remove entity_id entirely, or use a domain newtype
pub struct EntityNetId(i32);  // If really needed

#[derive(Event, Clone, Debug)]
pub struct SetEntityPoseRequest {
    pub entity: Entity,
    // pub entity_id: VarInt,  // ❌ Remove this
    pub pose: EntityPose,
    pub exclude_player: Option<Entity>,
}

// Core resolves Entity -> network ID internally
```

---

## 🟡 MEDIUM ISSUE #8: Inconsistent API Naming

**Different APIs use completely different patterns:**

### Animation API
```rust
pub fn play_animation(&mut self, entity: Entity, animation: AnimationType);
pub fn play_animation_except(&mut self, entity: Entity, animation: AnimationType, exclude: Entity);
pub fn set_pose(&mut self, entity: Entity, entity_id: VarInt, pose: EntityPose);
```

### Movement API
```rust
pub fn broadcast_movement_all(&mut self, entity: Entity, position: Position, rotation: Rotation);
pub fn broadcast_movement_to(&mut self, entity: Entity, target: Entity, position: Position, rotation: Rotation);
```

### Chat API
```rust
pub fn send(&mut self, player: Entity, message: TextComponent);
pub fn broadcast(&mut self, message: TextComponent);
```

### Block API
```rust
// Split into two types, then aliased
pub type BlockAPI<'w> = (BlockRequests<'w>, BlockBroadcasts<'w>);
```

**Why This Is Bad:**
- No consistency
- Different methods for same concept (`*_except`, `*_to`, `*_all`)
- Harder to learn and remember

**Fix:**
```rust
// Unified targeting system
pub enum Target {
    All,
    One(Entity),
    Except(Entity),
}

// All APIs use same pattern
pub fn play_animation(&mut self, entity: Entity, animation: AnimationType, target: Target);
pub fn send_message(&mut self, message: TextComponent, target: Target);
pub fn broadcast_movement(&mut self, entity: Entity, position: Position, target: Target);
```

---

## 🟡 MEDIUM ISSUE #9: Default Capabilities = All

**File:** [src/lib/plugin-api/src/lib.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/src/lib.rs#L137-L140)

```rust
#[allow(deprecated)]
fn capabilities(&self) -> PluginCapabilities {
    PluginCapabilities::all()  // ❌ BAD: Default undermines entire system
}
```

**File:** [src/lib/plugin-api/src/capabilities.rs](file:///Users/niklaswoj/RustroverProjects/ferrumc/src/lib/plugin-api/src/capabilities.rs#L84-L97)

```rust
#[deprecated(note = "Declare specific capabilities instead of using all()")]
pub fn all() -> Self {
    Self {
        animation_api: true,
        block_api: true,
        chat_api: true,
        // ... everything enabled  ❌ Defeats the purpose
    }
}
```

**Why This Is Bad:**
- Every plugin without explicit `capabilities()` gets **all access**
- Makes the capability system pointless during migration
- Encourages lazy "I'll fix it later" approach

**Fix:**
```rust
// Default to NONE, force explicit declaration
fn capabilities(&self) -> PluginCapabilities {
    PluginCapabilities::none()  // ✅ Secure by default
}

// Or emit warnings:
fn capabilities(&self) -> PluginCapabilities {
    let caps = PluginCapabilities::all();
    warn!("Plugin {} uses all() capabilities - this is deprecated!", self.name());
    caps
}
```

---

## Summary of Bad Code Found

| Issue | Severity | Files Affected | Impact |
|-------|----------|----------------|--------|
| EventWriter exposed | 🔴 Critical | prelude.rs | Bypasses all APIs |
| EntityExt network access | 🔴 Critical | entity.rs, lib.rs | Direct I/O from plugins |
| WorldExt database access | 🔴 Critical | world.rs, lib.rs | Direct I/O from plugins |
| No resource enforcement | 🔴 Critical | build_context.rs | Capabilities ignored |
| Duplicate events | 🔴 High | events.rs, join-leave-api | Silent bugs |
| Dependency ordering bug | 🔴 High | plugin_loader.rs | Wrong load order |
| VarInt in events | 🟡 Medium | animation-api/events.rs | Protocol coupling |
| Inconsistent naming | 🟡 Medium | All APIs | Poor DX |
| Default = all() | 🟡 Medium | lib.rs, capabilities.rs | No enforcement |

---

## Next Steps

1. **Immediate (can do today):**
   - Remove `EventWriter` from prelude
   - Remove `EntityExt` and `WorldExt` from exports
   - Add resource capability enforcement
   - Fix dependency ordering

2. **Near-term (this week):**
   - Deduplicate join/leave events
   - Add missing capability flags
   - Unify API naming patterns
   - Remove VarInt from plugin-facing events

3. **Future:**
   - Compile-time system param validation
   - CI enforcement (cargo-deny)
   - Third-party plugin SDK
