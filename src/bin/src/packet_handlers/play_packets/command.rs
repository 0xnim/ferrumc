use std::sync::Arc;

use bevy_ecs::prelude::*;
use ferrumc_commands::{
    events::{CommandDispatchEvent, ResolvedCommandDispatchEvent},
    infrastructure, Command, Sender,
};
use ferrumc_core::mq;
use ferrumc_net::ChatCommandPacketReceiver;
use ferrumc_permissions_api::PermissionsAPI;
use ferrumc_text::{NamedColor, TextComponent, TextComponentBuilder};

fn resolve(
    input: String,
) -> Result<(Arc<Command>, String), Box<TextComponent>> {
    let command = infrastructure::find_command(&input);
    if command.is_none() {
        return Err(Box::new(
            TextComponentBuilder::new("Unknown command")
                .color(NamedColor::Red)
                .build(),
        ));
    }

    let command = command.unwrap();
    let command_input = input
        .strip_prefix(command.name)
        .unwrap_or(&input)
        .trim_start()
        .to_string();

    Ok((command, command_input))
}

pub fn handle(
    events: Res<ChatCommandPacketReceiver>,
    mut dispatch_events: EventWriter<CommandDispatchEvent>,
    mut resolved_dispatch_events: EventWriter<ResolvedCommandDispatchEvent>,
    permissions: PermissionsAPI,
) {
    for (event, entity) in events.0.try_iter() {
        let sender = Sender::Player(entity);
        dispatch_events.write(CommandDispatchEvent {
            command: event.command.clone(),
            sender,
        });

        let resolved = resolve(event.command);
        match resolved {
            Err(err) => {
                mq::queue(*err, false, entity);
            }

            Ok((command, input)) => {
                if let Some(required_permission) = command.permission {
                    let has_perm = match sender {
                        Sender::Player(player_entity) => {
                            permissions.has_permission(player_entity, required_permission)
                        }
                        Sender::Server => true,
                    };

                    if !has_perm {
                        let error = TextComponentBuilder::new("You don't have permission to use this command")
                            .color(NamedColor::Red)
                            .build();
                        mq::queue(error, false, entity);
                        continue;
                    }
                }

                resolved_dispatch_events.write(ResolvedCommandDispatchEvent {
                    command,
                    input,
                    sender,
                });
            }
        }
    }
}
