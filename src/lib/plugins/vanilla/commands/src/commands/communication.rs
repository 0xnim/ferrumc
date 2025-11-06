//! Communication commands
//!
//! Implements:
//! - `/help` - Provides help for commands
//! - `/me <message>` - Displays a message about the sender

use ferrumc_commands::{arg::primitive::string::GreedyString, infrastructure, Sender};
use ferrumc_commands_api::CommandsAPI;
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
