//! Communication commands
//!
//! Implements:
//! - `/help` - Provides help for commands
//! - `/me <message>` - Displays a message about the sender
//! - `/list` - Lists online players
//! - `/msg <player> <message>` - Private message (aliases: /tell, /w)

use ferrumc_commands::{arg::primitive::string::{GreedyString, SingleWord}, infrastructure, Sender};
use ferrumc_commands_api::CommandsAPI;
use ferrumc_config::server_config::get_global_config;
use ferrumc_macros::command;
use ferrumc_permissions_api::PermissionsAPI;
use ferrumc_plugin_api::queries::EntityQueries;
use ferrumc_text::{ComponentBuilder, NamedColor};

#[command("help")]
fn help_command(
    #[sender] sender: Sender,
    permissions: PermissionsAPI,
    mut commands: CommandsAPI,
) {
    let help_text = ComponentBuilder::text("Available commands:")
        .color(NamedColor::Gold)
        .build();
    
    let all_commands = infrastructure::get_all_commands();
    let mut sorted_commands = all_commands;
    sorted_commands.sort_by(|a, b| a.name.cmp(b.name));
    
    match sender {
        Sender::Player(entity) => {
            commands.send_to_player(entity, help_text);
            
            for cmd in sorted_commands {
                let can_use = if let Some(required_perm) = cmd.permission {
                    permissions.has_permission(entity, required_perm)
                } else {
                    true
                };
                
                if can_use {
                    let args_hint = if cmd.args.is_empty() {
                        String::new()
                    } else {
                        let arg_names: Vec<String> = cmd.args.iter()
                            .map(|arg| {
                                if arg.required {
                                    format!("<{}>", arg.name)
                                } else {
                                    format!("[{}]", arg.name)
                                }
                            })
                            .collect();
                        format!(" {}", arg_names.join(" "))
                    };
                    
                    let msg = ComponentBuilder::text(format!("  /{}{}", cmd.name, args_hint))
                        .color(NamedColor::Gray)
                        .build();
                    commands.send_to_player(entity, msg);
                }
            }
        }
        Sender::Server => {
            commands.send_to_console(help_text);
            
            for cmd in sorted_commands {
                let args_hint = if cmd.args.is_empty() {
                    String::new()
                } else {
                    let arg_names: Vec<String> = cmd.args.iter()
                        .map(|arg| {
                            if arg.required {
                                format!("<{}>", arg.name)
                            } else {
                                format!("[{}]", arg.name)
                            }
                        })
                        .collect();
                    format!(" {}", arg_names.join(" "))
                };
                
                let msg = ComponentBuilder::text(format!("  /{}{}", cmd.name, args_hint)).build();
                commands.send_to_console(msg);
            }
        }
    }
}

#[command("me")]
fn me_command(
    #[arg] message: GreedyString,
    #[sender] sender: Sender,
    entities: EntityQueries,
    mut chat: ferrumc_chat_api::ChatAPI,
) {
    let username = match sender {
        Sender::Server => "Server".to_string(),
        Sender::Player(entity) => entities
            .identity(entity)
            .map(|identity| identity.username.clone())
            .unwrap_or_else(|| "Unknown".to_string()),
    };

    let me_message = ComponentBuilder::text(format!("* {} {}", username, message.clone()))
        .color(NamedColor::Yellow)
        .italic()
        .build();

    // Broadcast to all players
    chat.broadcast(me_message);
}

#[command("list")]
fn list_command(
    #[sender] sender: Sender,
    entities: EntityQueries,
    mut commands: CommandsAPI,
) {
    let player_count = entities.player_count();
    let max_players = get_global_config().max_players;
    
    let header = ComponentBuilder::text(format!(
        "There are {} of a max of {} players online:",
        player_count, max_players
    ))
    .color(NamedColor::White)
    .build();
    
    match sender {
        Sender::Player(entity) => {
            commands.send_to_player(entity, header);
            
            if player_count > 0 {
                let player_names: Vec<String> = entities
                    .iter_players()
                    .map(|(_, _, identity)| identity.username.clone())
                    .collect();
                
                let player_list = ComponentBuilder::text(player_names.join(", "))
                    .color(NamedColor::Gray)
                    .build();
                    
                commands.send_to_player(entity, player_list);
            }
        }
        Sender::Server => {
            commands.send_to_console(header);
            
            if player_count > 0 {
                let player_names: Vec<String> = entities
                    .iter_players()
                    .map(|(_, _, identity)| identity.username.clone())
                    .collect();
                
                let player_list = ComponentBuilder::text(player_names.join(", ")).build();
                commands.send_to_console(player_list);
            }
        }
    }
}

#[command("msg")]
fn msg_command(
    #[arg] target: SingleWord,
    #[arg] message: GreedyString,
    #[sender] sender: Sender,
    entities: EntityQueries,
    mut commands: CommandsAPI,
) {
    let sender_name = match sender {
        Sender::Server => "Server".to_string(),
        Sender::Player(entity) => entities
            .identity(entity)
            .map(|identity| identity.username.clone())
            .unwrap_or_else(|| "Unknown".to_string()),
    };
    
    let Some((target_entity, target_identity)) = entities.find_player_by_name(target.as_str()) else {
        let error = ComponentBuilder::text(format!("Player '{}' not found", target.as_str()))
            .color(NamedColor::Red)
            .build();
        match sender {
            Sender::Player(player) => commands.send_to_player(player, error),
            Sender::Server => commands.send_to_console(error),
        }
        return;
    };
    
    let outgoing_msg = ComponentBuilder::text(format!(
        "You whisper to {}: {}",
        target_identity.username, message.as_str()
    ))
    .color(NamedColor::Gray)
    .italic()
    .build();
    
    let incoming_msg = ComponentBuilder::text(format!(
        "{} whispers to you: {}",
        sender_name, message.as_str()
    ))
    .color(NamedColor::Gray)
    .italic()
    .build();
    
    match sender {
        Sender::Player(player) => commands.send_to_player(player, outgoing_msg),
        Sender::Server => commands.send_to_console(outgoing_msg),
    }
    
    commands.send_to_player(target_entity, incoming_msg);
}

#[command("tell")]
fn tell_command(
    #[arg] target: SingleWord,
    #[arg] message: GreedyString,
    #[sender] sender: Sender,
    entities: EntityQueries,
    mut commands: CommandsAPI,
) {
    msg_command(target, message, sender, entities, commands);
}

#[command("w")]
fn w_command(
    #[arg] target: SingleWord,
    #[arg] message: GreedyString,
    #[sender] sender: Sender,
    entities: EntityQueries,
    mut commands: CommandsAPI,
) {
    msg_command(target, message, sender, entities, commands);
}
