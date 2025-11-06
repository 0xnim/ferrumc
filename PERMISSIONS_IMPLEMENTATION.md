# LuckPerms-style Permission System Implementation

## Overview

Implemented a full LuckPerms-style permission system for FerrumC with groups, inheritance, and permission nodes.

## Architecture

### Layer 1: Core Components
**Location:** `src/lib/core/src/permissions/`

- **`PlayerPermissions`** - Component attached to player entities
  - `groups: HashSet<String>` - Groups the player belongs to
  - `permissions: HashMap<String, bool>` - Permission overrides (individual player permissions)
  
- **`PermissionGroup`** - Definition of a permission group
  - `name: String` - Group name (e.g., `"minecraft.op.level.2"`)
  - `permissions: HashMap<String, bool>` - Permissions granted by this group
  - `inherits: Vec<String>` - Parent groups (for inheritance)

### Layer 2: Permissions API
**Location:** `src/lib/apis/permissions-api/`

**Events:**
- `PermissionCheckEvent` - Permission check requested
- `PermissionGrantedEvent` - Permission granted to player
- `PermissionRevokedEvent` - Permission revoked from player
- `GroupAddedEvent` - Player added to group
- `GroupRemovedEvent` - Player removed from group
- `PermissionsReloadEvent` - Reload all permissions

**PermissionsAPI (SystemParam):**
```rust
// Check if player has permission
perms.has_permission(entity, "minecraft.command.gamemode") -> bool

// Group management
perms.add_group(entity, "minecraft.op.level.2") -> bool
perms.remove_group(entity, "minecraft.op.level.2") -> bool
perms.get_player_groups(entity) -> Option<Vec<String>>

// Individual permission management
perms.set_permission(entity, "ferrumc.build.spawn", true) -> bool
perms.unset_permission(entity, "ferrumc.build.spawn") -> bool

// List all available groups
perms.list_groups() -> Vec<String>
```

**PermissionGroups (Resource):**
- Stores all registered permission groups
- Handles permission inheritance resolution
- Supports wildcard permissions (`minecraft.command.*`)

### Layer 3: Permissions Plugin
**Location:** `src/lib/plugins/core/permissions/`

**Default Minecraft OP Groups:**

1. **`minecraft.op.level.0`** - Default player permissions
   - `/help`, `/me`, `/trigger`, `/list`, `/teammsg`, `/msg`, `/tell`

2. **`minecraft.op.level.1`** - Inherits level.0
   - `minecraft.bypass.spawn_protection`

3. **`minecraft.op.level.2`** - Inherits level.1 (standard `/op`)
   - All gameplay commands: `/gamemode`, `/give`, `/teleport`, `/tp`, `/kill`, `/time`, `/weather`, etc.
   - Wildcard: `minecraft.command.*`

4. **`minecraft.op.level.3`** - Inherits level.2
   - Admin commands: `/ban`, `/kick`, `/op`, `/deop`, `/whitelist`, etc.

5. **`minecraft.op.level.4`** - Inherits level.3
   - Server management: `/stop`, `/save-all`, `/save-on`, `/save-off`, `/publish`

## Integration

### Command Permission Checks
**Location:** `src/bin/src/packet_handlers/play_packets/command.rs`

Commands now check permissions before execution:
```rust
if let Some(required_permission) = command.permission {
    let has_perm = match sender {
        Sender::Player(player_entity) => {
            permissions.has_permission(player_entity, required_permission)
        }
        Sender::Server => true, // Server always has permission
    };

    if !has_perm {
        // Send permission denied error
        return;
    }
}
```

### Player Join
**Location:** `src/bin/src/systems/new_connections.rs`

`PlayerPermissions::default()` component is added to all new player entities (no groups by default).

## Commands

### `/op <player>`
- **Permission Required:** `minecraft.command.op` (level 3)
- **Action:** Adds player to `minecraft.op.level.2` group
- **Effect:** Player gains all standard operator permissions

### `/deop <player>`
- **Permission Required:** `minecraft.command.deop` (level 3)
- **Action:** Removes player from `minecraft.op.level.2` group
- **Effect:** Player loses operator permissions

## Permission Node Format

### Vanilla Minecraft
- `minecraft.command.<command_name>` - Execute specific command
- `minecraft.op.level.<0-4>` - Operator level group
- `minecraft.bypass.<feature>` - Bypass specific restrictions

### Custom Permissions (Examples)
- `ferrumc.build.spawn` - Build in spawn area
- `ferrumc.command.fly` - Use custom fly command
- `ferrumc.bypass.limit.*` - Bypass all limits

### Wildcards
- `minecraft.command.*` - All Minecraft commands
- `ferrumc.*` - All custom permissions

## Permission Inheritance

Groups inherit permissions from parent groups:
```
level.4 → level.3 → level.2 → level.1 → level.0
```

Player permission overrides take precedence over group permissions.

## Next Steps (Not Implemented)

1. **Storage Integration** - Persist permissions to database (currently in-memory only)
2. **Advanced Commands** - `/permission`, `/group`, `/rank` commands for fine-grained control
3. **Per-World Permissions** - Context-aware permissions based on world/dimension
4. **Timed Permissions** - Temporary permissions that expire
5. **Meta/Prefixes/Suffixes** - Chat prefixes, tab list colors, etc.

## Files Created/Modified

### Created:
- `src/lib/core/src/permissions/mod.rs`
- `src/lib/core/src/permissions/player_permissions.rs`
- `src/lib/apis/permissions-api/Cargo.toml`
- `src/lib/apis/permissions-api/src/lib.rs`
- `src/lib/apis/permissions-api/src/events.rs`
- `src/lib/apis/permissions-api/src/traits.rs`
- `src/lib/plugins/core/permissions/Cargo.toml`
- `src/lib/plugins/core/permissions/src/lib.rs`
- `src/lib/plugins/vanilla/commands/src/commands/permissions.rs`

### Modified:
- `Cargo.toml` - Added workspace members and dependencies
- `src/lib/core/src/lib.rs` - Export permissions module
- `src/lib/commands/src/lib.rs` - Added `permission` field to `Command`
- `src/lib/derive_macros/src/commands/mod.rs` - Parse permission attribute
- `src/bin/Cargo.toml` - Added dependencies
- `src/bin/src/plugin_loader.rs` - Register permissions plugin
- `src/bin/src/packet_handlers/play_packets/command.rs` - Permission checking
- `src/bin/src/systems/new_connections.rs` - Add PlayerPermissions component
- `src/lib/plugin-api/src/build_context.rs` - Add `insert_resource` method
- `src/lib/plugins/vanilla/commands/src/commands/mod.rs` - Import permissions module
- `src/lib/plugins/vanilla/commands/Cargo.toml` - Add permissions-api dependency

## Usage Example

```rust
// In a command handler
#[command("fly")]
fn fly_command(
    #[sender] sender: Sender,
    permissions: PermissionsAPI,
) {
    if let Sender::Player(player) = sender {
        if permissions.has_permission(player, "ferrumc.command.fly") {
            // Enable flight
        }
    }
}

// In a system
fn check_build_permission(
    query: Query<(Entity, &Position)>,
    permissions: PermissionsAPI,
) {
    for (entity, pos) in query.iter() {
        if is_spawn_area(pos) {
            if !permissions.has_permission(entity, "ferrumc.build.spawn") {
                // Prevent building
            }
        }
    }
}
```

## Build Status

✅ All checks pass
✅ No compilation errors
✅ Permission system functional
⚠️ Storage persistence not yet implemented
