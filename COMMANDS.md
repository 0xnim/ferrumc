# Minecraft Vanilla Commands Reference

This document contains a comprehensive list of all vanilla Minecraft commands extracted from [minecraft.wiki](https://minecraft.wiki/w/Commands).

## Java Edition Commands

### Gameplay Commands

| Command | Description | OP Level |
|---------|-------------|----------|
| `/advancement` | Gives, removes, or checks player advancements | 2 |
| `/attribute` | Queries, adds, removes or sets an entity attribute | 2 |
| `/clear` | Clears items from player inventory | 2 |
| `/damage` | Applies damage to the specified entities | 2 |
| `/effect` | Adds or removes status effects | 2 |
| `/enchant` | Adds an enchantment to a player's selected item | 2 |
| `/experience` (alias: `/xp`) | Adds or removes player experience | 2 |
| `/give` | Gives an item to a player | 2 |
| `/item` | Manipulates items in inventories | 2 |
| `/kill` | Kills entities (players, mobs, items, etc.) | 2 |
| `/loot` | Drops items from an inventory slot onto the ground | 2 |
| `/recipe` | Gives or takes player recipes | 2 |
| `/ride` | Used to make entities ride other entities, or stop entities from riding | 2 |
| `/rotate` | Changes the rotation of an entity | 2 |
| `/scoreboard` | Manages scoreboard objectives and players | 2 |
| `/summon` | Summons an entity | 2 |
| `/tag` | Controls entity tags | 2 |
| `/team` | Controls teams | 2 |
| `/teleport` (alias: `/tp`) | Teleports entities | 2 |
| `/trigger` | Sets a trigger to be activated | 0 |

### World Manipulation

| Command | Description | OP Level |
|---------|-------------|----------|
| `/clone` | Copies blocks from one place to another | 2 |
| `/fill` | Fills a region with a specific block | 2 |
| `/fillbiome` | Fills a region with a specific biome | 2 |
| `/locate` | Locates closest structure, biome, or point of interest | 2 |
| `/particle` | Creates particles | 2 |
| `/place` | Used to place a configured feature, jigsaw, template, or structure at a given location | 2 |
| `/playsound` | Plays a sound | 2 |
| `/setblock` | Changes a block to another block | 2 |
| `/setworldspawn` | Sets the world spawn | 2 |
| `/spawnpoint` | Sets the spawn point for a player | 2 |
| `/spreadplayers` | Teleports entities to random locations | 2 |
| `/stopsound` | Stops a sound | 2 |
| `/weather` | Sets the weather | 2 |
| `/worldborder` | Manages the world border | 2 |

### Game Management

| Command | Description | OP Level |
|---------|-------------|----------|
| `/defaultgamemode` | Sets the default game mode | 2 |
| `/difficulty` | Sets the difficulty level | 2 |
| `/forceload` | Forces chunks to constantly be loaded or not | 2 |
| `/gamemode` | Sets a player's game mode | 2 |
| `/gamerule` | Sets or queries a game rule value | 2 |
| `/reload` | Reloads loot tables, advancements, and functions from disk | 2 |
| `/seed` | Displays the world seed | 0 (SP) / 2 (MP) |
| `/time` | Changes or queries the world's game time | 2 |
| `/tick` | Controls the tick rate of the game | 3 |

### Multiplayer Commands

| Command | Description | OP Level |
|---------|-------------|----------|
| `/ban` | Adds player to banlist | 3 |
| `/ban-ip` | Adds IP address to banlist | 3 |
| `/banlist` | Displays banlist | 3 |
| `/deop` | Revokes operator status from a player | 3 |
| `/kick` | Kicks a player off a server | 3 |
| `/list` | Lists players on the server | 0 |
| `/op` | Grants operator status to a player | 3 |
| `/pardon` | Removes entries from the banlist | 3 |
| `/pardon-ip` | Removes entries from the banlist | 3 |
| `/publish` | Opens single-player world to local network | 4 |
| `/save-all` | Saves the server to disk | 4 |
| `/save-off` | Disables automatic server saves | 4 |
| `/save-on` | Enables automatic server saves | 4 |
| `/setidletimeout` | Sets the time before idle players are kicked | 3 |
| `/stop` | Stops a server | 4 |
| `/transfer` | Triggers a transfer of a player to another server | 3 |
| `/whitelist` | Manages server whitelist | 3 |

### Communication Commands

| Command | Description | OP Level |
|---------|-------------|----------|
| `/help` | Provides help for commands | 0 |
| `/me` | Displays a message about the sender | 0 |
| `/msg` (aliases: `/tell`, `/w`) | Displays a private message to other players | 0 |
| `/say` | Displays a message to multiple players | 2 |
| `/teammsg` (alias: `/tm`) | Specifies the message to send to team | 0 |
| `/tellraw` | Displays a JSON message to players | 2 |
| `/title` | Manages screen titles | 2 |

### Advanced Commands

| Command | Description | OP Level |
|---------|-------------|----------|
| `/bossbar` | Creates and modifies bossbars | 2 |
| `/data` | Gets, merges, modifies, and removes block entity, entity, and command storage NBT data | 2 |
| `/datapack` | Controls loaded data packs | 2 |
| `/execute` | Executes another command | 2 |
| `/fetchprofile` | Fetches a player profile | 2 |
| `/function` | Runs a function | 2 |
| `/random` | Draw a random value or control the random number sequence | 0 (without sequence) / 2 |
| `/return` | Control execution flow inside functions and change their return value | 2 |
| `/schedule` | Delays the execution of a function | 2 |
| `/spectate` | Make one player in spectator mode spectate an entity | 2 |
| `/waypoint` | Manages waypoints displayed on the locator bar | 2 |

### Debugging Commands

These commands require debug properties to be enabled and are intended for debugging purposes only.

| Command | Description | OP Level | Debug Property |
|---------|-------------|----------|----------------|
| `/warden_spawn_tracker` | Shows how many warnings a sculk shrieker has before spawning a warden | 2 | DEV_COMMANDS |
| `/debugpath` | Shows the calculated path for entities | 2 | - |
| `/debugmobspawning` | Displays mob spawning information | 2 | - |
| `/raid` | Starts a raid | 3 | - |
| `/serverpack` | Configures a server resource pack | 2 | - |
| `/spawn_armor_trims` | Spawns all possible combinations of designs, materials, and armor types of armor trims | 2 | - |
| `/debugconfig` | Configuration options for debugging | 3 | - |
| `/chase` | Sync the movement of two or more clients | 0 | CHASE_COMMAND |
| `/debug` | Starts or stops a debugging session | 3 | - |
| `/jfr` | Starts or stops a JFR profiling | 4 | - |
| `/perf` | Captures info and metrics about the game for 10 seconds | 4 | - |
| `/test` | Manage and execute GameTests | 2 | - |

## Bedrock Edition Commands

### Additional Bedrock-Specific Commands

| Command | Description | OP Level |
|---------|-------------|----------|
| `/ability` | Grants or revokes a player ability | 1 |
| `/aimassist` | Modifies player's aim assist | 1 |
| `/allowlist` (alias: `/whitelist`) | Manages server allowlist | 4 |
| `/alwaysday` (alias: `/daylock`) | Locks and unlocks the day-night cycle | 1 |
| `/camera` | Switch to custom camera perspectives and fade the screen | 1 |
| `/camerashake` | Used to enable a camera shaking effect | 1 |
| `/changesetting` | Changes a setting on the dedicated server while it's running | 4 |
| `/clearspawnpoint` | Removes the spawn point for a player | 1 |
| `/connect` (alias: `/wsserver`) | Attempts to connect to the websocket server | 0 (edu) / 2 (BE) |
| `/controlscheme` | Switching control schemes when the Experimental Creator Camera toggle is enabled | ? |
| `/daylock` (alias: `/alwaysday`) | Locks and unlocks the day-night cycle | 1 |
| `/dedicatedwsserver` | Attempts to connect to a websocket server | 0 |
| `/dialogue` | Opens NPC dialogue for a player | 1 |
| `/event` | Used to trigger an event on an entity | 1 |
| `/fog` | Used for managing active fog settings for players | 1 |
| `/gametest` | To test the GameTest features | 1 |
| `/gametips` | Enable or disable the game tips on this device | 0 |
| `/hud` | Changes the visibility of a HUD element | 1 |
| `/immutableworld` | Allows setting immutable state of a world | 1 |
| `/inputpermission` | Specify the operation for the player's permission status | 1 |
| `/mobevent` | Enables/disables a specified mob event | 1 |
| `/music` | Allows the player to control playing music tracks | 1 |
| `/ops` (alias: `/permission`) | Reloads and applies permissions | 4 |
| `/permission` (alias: `/ops`) | Reloads and applies permissions | 4 |
| `/playanimation` | Used to run a one-off animation | 1 |
| `/project` | Contains tools for managing Editor projects | 1 |
| `/reloadconfig` | Reloads configuration files relating to variables, permissions, etc. | 4 |
| `/reloadpacketlimitconfig` | Reloads packetlimitconfig.json configuration file | 4 |
| `/remove` | Removing agent | 2 |
| `/replaceitem` | Replaces items in inventories | 1 |
| `/save` | Prepares a backup, queries its status, or resumes | 4 |
| `/script` | Debugging options for GameTest Framework | ? |
| `/scriptevent` | Triggers a script event with an ID and message | ? |
| `/sendshowstoreoffer` | Send a request to show players the store offer | 4 |
| `/set_movement_authority` | Control the client or server to check a player movement | 1 |
| `/setmaxplayers` | Sets the maximum number of players allowed to join | 3 |
| `/structure` | Used to save and load structures without having to use structure blocks items in inventories | 1 |
| `/testfor` | Counts entities matching specified conditions | 1 |
| `/testforblock` | Tests whether a block is in a location | 1 |
| `/testforblocks` | Tests whether the blocks in two regions match | 1 |
| `/tickingarea` | Add, remove, or list ticking areas | 1 |
| `/titleraw` | Controls screen titles with JSON messages | 1 |
| `/toggledownfall` | Toggles the weather | 1 |
| `/wb` (alias: `/worldbuilder`) | Ability to edit restricted blocks | 1 |
| `/worldbuilder` (alias: `/wb`) | Ability to edit restricted blocks | 1 |

### Education Edition Exclusive Commands

| Command | Description | OP Level |
|---------|-------------|----------|
| `/agent` | Controls the agent | 0 (edu) / 1 (BE) |
| `/codebuilder` | Sets Code Builder state for a player | 1 |
| `/classroommode` | Ability to edit restricted blocks | 0 |
| `/code` | Launches Code Builder | 0 |

## Hidden Commands (Bedrock)

These commands are unavailable in general cases and can only be accessed with a Websocket Server, NPC, or Scripting API.

| Command | Description | OP Level |
|---------|-------------|----------|
| `/closechat` | Closes the chat window of the local player if it is open | 0 |
| `/closewebsocket` | Closes websocket connection if there is one | 0 |
| `/enableencryption` | Enable encryption for the current websocket connection | 0 |
| `/getchunkdata` | Gets pixels for a specific chunk | 3 |
| `/getchunks` | Gets list of chunks that are loaded | 3 |
| `/geteduclientinfo` | Shows the information about the client | 0 |
| `/geteduserverinfo` | Shows the information about the server | 0 |
| `/getlocalplayername` | Shows the name of the local player | 0 |
| `/getspawnpoint` | Gets the spawn position of the specified player(s) | 3 |
| `/gettopsolidblock` | Gets the position of the top non-air block below the specified position | 0 (edu) / 1 (BE) |
| `/globalpause` | Sets or gets the paused state of the game for all players | 3 |
| `/lesson` | Handle Educational Lesson reporting | 0 |
| `/listd` | Lists the information about players on the server | 3 |
| `/querytarget` | Gets transform, name, and id information about the given target entity or entities | 0 (edu) / 2 (BE) |
| `/spawnitem` | Spawns an item entity at position | 0 |
| `/takepicture` | Takes a photo of a player or from a player's point of view | 1 |

## Command Syntax

### Java Edition Syntax

- `plain text` - Enter this literally, exactly as shown
- `<argumentName>` - An argument that should be replaced with an appropriate value
- `[entry]` - This entry is optional
- `(entry|entry)` - Required. Pick one of these entries
- `[entry|entry]` - Optional. Pick one of these entries

### Bedrock Edition Syntax

- `plain text` - Enter this literally, exactly as shown
- `name: type` - An argument that should be replaced with an appropriate value
- `<entry>` - This entry is required
- `[entry]` - This entry is optional
- `plain text|plain text` - Enter one of these texts literally

## Permission Levels

| Level | Description |
|-------|-------------|
| 0 | All players can use the command |
| 1 | Command blocks and players with "cheat" access |
| 2 | Players with "operator" status (default level) |
| 3 | Players with higher operator privileges |
| 4 | Server console and commands run by the server |

## Notes

- Commands are prefixed with `/` when entered in chat or command blocks
- Many commands support tab completion
- Some commands have aliases (e.g., `/tp` is an alias for `/teleport`)
- Bedrock Edition and Java Edition have different command syntaxes and availability
- Some commands require cheats to be enabled
- Debug commands should not be used during normal gameplay

## References

- Source: [Minecraft Wiki - Commands](https://minecraft.wiki/w/Commands)
- Last updated: Based on Minecraft 1.21+ (Java Edition) and latest Bedrock Edition
