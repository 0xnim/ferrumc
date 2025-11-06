use ferrumc_commands::{arg::primitive::string::SingleWord, Sender};
use ferrumc_commands_api::CommandsAPI;
use ferrumc_macros::command;
use ferrumc_permissions_api::PermissionsAPI;
use ferrumc_plugin_api::queries::EntityQueries;
use ferrumc_text::{ComponentBuilder, NamedColor};

#[command("op", permission = "minecraft.command.op")]
fn op_command(
    #[arg] player_name: SingleWord,
    #[sender] sender: Sender,
    entities: EntityQueries,
    mut permissions: PermissionsAPI,
    mut commands: CommandsAPI,
) {
    let Some((entity, identity)) = entities.find_player_by_name(player_name.as_str()) else {
        let error = ComponentBuilder::text(format!("Player '{}' not found", player_name.as_str()))
            .color(NamedColor::Red)
            .build();
        match sender {
            Sender::Player(player) => commands.send_to_player(player, error),
            Sender::Server => commands.send_to_console(error),
        }
        return;
    };

    if permissions.add_group(entity, "minecraft.op.level.2") {
        let success = ComponentBuilder::text(format!(
            "Made {} a server operator",
            identity.username
        ))
        .color(NamedColor::Green)
        .build();

        match sender {
            Sender::Player(player) => commands.send_to_player(player, success.clone()),
            Sender::Server => commands.send_to_console(success.clone()),
        }

        let notification = ComponentBuilder::text("You are now a server operator")
            .color(NamedColor::Yellow)
            .build();
        commands.send_to_player(entity, notification);
    } else {
        let error = ComponentBuilder::text("Failed to grant operator status")
            .color(NamedColor::Red)
            .build();
        match sender {
            Sender::Player(player) => commands.send_to_player(player, error),
            Sender::Server => commands.send_to_console(error),
        }
    }
}

#[command("deop", permission = "minecraft.command.deop")]
fn deop_command(
    #[arg] player_name: SingleWord,
    #[sender] sender: Sender,
    entities: EntityQueries,
    mut permissions: PermissionsAPI,
    mut commands: CommandsAPI,
) {
    let Some((entity, identity)) = entities.find_player_by_name(player_name.as_str()) else {
        let error = ComponentBuilder::text(format!("Player '{}' not found", player_name.as_str()))
            .color(NamedColor::Red)
            .build();
        match sender {
            Sender::Player(player) => commands.send_to_player(player, error),
            Sender::Server => commands.send_to_console(error),
        }
        return;
    };

    if permissions.remove_group(entity, "minecraft.op.level.2") {
        let success = ComponentBuilder::text(format!(
            "Made {} no longer a server operator",
            identity.username
        ))
        .color(NamedColor::Green)
        .build();

        match sender {
            Sender::Player(player) => commands.send_to_player(player, success.clone()),
            Sender::Server => commands.send_to_console(success.clone()),
        }

        let notification = ComponentBuilder::text("You are no longer a server operator")
            .color(NamedColor::Yellow)
            .build();
        commands.send_to_player(entity, notification);
    } else {
        let error = ComponentBuilder::text("Failed to revoke operator status")
            .color(NamedColor::Red)
            .build();
        match sender {
            Sender::Player(player) => commands.send_to_player(player, error),
            Sender::Server => commands.send_to_console(error),
        }
    }
}
