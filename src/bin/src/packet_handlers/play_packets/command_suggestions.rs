use std::sync::Arc;

use bevy_ecs::prelude::*;
use ferrumc_commands::{Command, CommandContext, CommandInput, Sender, ROOT_COMMAND};
use ferrumc_net::{
    connection::StreamWriter,
    packets::outgoing::command_suggestions::{CommandSuggestionsPacket, Match},
    CommandSuggestionRequestReceiver,
};
use ferrumc_net_codec::net_types::{
    length_prefixed_vec::LengthPrefixedVec, prefixed_optional::PrefixedOptional, var_int::VarInt,
};
use ferrumc_state::GlobalStateResource;
use tracing::error;

fn find_command(input: String) -> Option<Arc<Command>> {
    let mut input = input;
    if input.starts_with("/") {
        input.remove(0);
    }

    if let Some(command) = ferrumc_commands::infrastructure::get_command_by_name(&input) {
        return Some(command);
    }

    if let Some(command) = ferrumc_commands::infrastructure::find_command(&input) {
        return Some(command);
    }

    while !input.is_empty() {
        // remove the last word and retry
        if let Some(pos) = input.rfind(char::is_whitespace) {
            input.truncate(pos);

            if let Some(command) = ferrumc_commands::infrastructure::get_command_by_name(&input) {
                return Some(command);
            }

            if let Some(command) = ferrumc_commands::infrastructure::find_command(&input) {
                return Some(command);
            }
        } else {
            break; // string does not have any further words, meaning it's just whitespace?
        }
    }

    None
}

pub fn handle(world: &mut bevy_ecs::world::World) {
    let mut system_state: bevy_ecs::system::SystemState<(
        Res<CommandSuggestionRequestReceiver>,
        Query<&StreamWriter>,
        Res<GlobalStateResource>,
    )> = bevy_ecs::system::SystemState::new(world);

    // Collect all pending suggestions first
    let suggestions_to_process: Vec<_> = {
        let (events, _, state) = system_state.get_mut(world);
        events.0.try_iter()
            .filter(|(_, entity)| state.0.players.is_connected(*entity))
            .collect()
    };
    system_state.apply(world);

    for (request, entity) in suggestions_to_process {
        let input = request.input;

        let command = find_command(input.clone());
        let command_arg = input
            .clone()
            .strip_prefix(&format!(
                "/{} ",
                command.clone().map(|c| c.name).unwrap_or_default()
            ))
            .unwrap_or(&input)
            .to_string();
        let stripped_input = command_arg
            .clone()
            .strip_prefix(command.clone().map(|c| c.name).unwrap_or_default())
            .unwrap_or(&command_arg)
            .trim_start()
            .to_string();
        
        let command_input = CommandInput::of(stripped_input);
        let mut ctx = CommandContext {
            input: command_input,
            command: command.clone().unwrap_or(ROOT_COMMAND.clone()),
            sender: Sender::Player(entity),
            world,
        };
        let command_arg = command_arg.clone(); // ok borrow checker
        let tokens = command_arg.split(" ").collect::<Vec<&str>>();
        let Some(current_token) = tokens.last() else {
            return; // whitespace
        };

        let mut suggestions = Vec::new();

        if let Some(ref command) = command {
            for arg in command.args.clone() {
                let arg_suggestions = (arg.suggester)(&mut ctx);
                ctx.input.skip_whitespace(u32::MAX, true);
                if !ctx.input.has_remaining_input() {
                    suggestions = arg_suggestions;
                    break;
                }
            }
        }

        let length = input.len();
        let start = length - current_token.len();

        let packet = CommandSuggestionsPacket {
                transaction_id: request.transaction_id,
                matches: LengthPrefixedVec::new(
                    suggestions
                        .into_iter()
                        .filter(|sug| sug.content.starts_with(current_token))
                        .map(|sug| Match {
                            content: sug.content,
                            tooltip: PrefixedOptional::new(sug.tooltip),
                        })
                        .collect(),
                ),
                length: VarInt::new(length as i32),
                start: VarInt::new(start as i32),
        };

        // Send the packet using a fresh system state access
        let mut send_state: bevy_ecs::system::SystemState<Query<&StreamWriter>> = 
            bevy_ecs::system::SystemState::new(world);
        let writers = send_state.get(world);
        
        if let Ok(writer) = writers.get(entity) {
            if let Err(e) = writer.send_packet(packet) {
                error!("failed sending command suggestions to player: {e}");
            }
        }
        send_state.apply(world);
    }
}
