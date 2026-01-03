//! Dynamic command dispatch system.
//!
//! This system handles commands registered via the mod API's builder pattern.
//! It runs before the macro-generated command systems.

use bevy_ecs::prelude::*;
use ferrumc_text::{NamedColor, TextComponentBuilder};

use crate::{
    infrastructure::get_dynamic_command,
    messages::ResolvedCommandDispatched,
    sender::Sender,
};

/// System that dispatches dynamic commands registered via the mod API.
///
/// This system:
/// 1. Reads `ResolvedCommandDispatched` messages
/// 2. Checks if the command is a dynamic command (registered via builder API)
/// 3. If so, executes the handler
/// 4. If not, leaves the message for macro-generated handlers
///
/// Note: Dynamic commands are processed first. The macro-generated systems
/// will also see the same messages, but since we handle dynamic ones here,
/// they'll just skip over them (no matching command name).
pub fn dispatch_dynamic_commands(
    mut messages: MessageMutator<ResolvedCommandDispatched>,
) {
    for ResolvedCommandDispatched { command, ref mut ctx, sender } in messages.read() {
        let command_name = command.name;

        // Check if this is a dynamic command
        let Some(dynamic) = get_dynamic_command(command_name) else {
            // Not a dynamic command - macro-generated systems will handle it
            continue;
        };

        // Check requires_player
        if dynamic.requires_player {
            if let Sender::Server = sender {
                sender.send_message(
                    TextComponentBuilder::new("This command can only be run by a player")
                        .color(NamedColor::Red)
                        .build(),
                    false,
                );
                continue;
            }
        }

        // TODO: Check permission when permission system is implemented
        // if let Some(ref perm) = dynamic.permission {
        //     if !has_permission(&sender, perm) {
        //         sender.send_message(...);
        //         continue;
        //     }
        // }

        // Execute the handler
        if let Err(err) = dynamic.handler.execute(ctx) {
            sender.send_message(*err, false);
        }
    }
}
