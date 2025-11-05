//! Vanilla Join/Leave Plugin
//!
//! Implements vanilla Minecraft join/leave message formatting.

use ferrumc_plugin_api::prelude::*;
use ferrumc_join_leave_api::{JoinLeaveAPI, PlayerJoinEvent, PlayerLeaveEvent};
use ferrumc_text::{Color, NamedColor, TextComponent};
use tracing::trace;

#[derive(Default)]
pub struct VanillaJoinLeavePlugin;

impl Plugin for VanillaJoinLeavePlugin {
    fn name(&self) -> &'static str {
        "vanilla-join-leave"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Vanilla Minecraft join/leave messages"
    }

    fn priority(&self) -> i32 {
        20
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_join_leave_api()
            .with_entity_queries()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        trace!("Loading vanilla-join-leave plugin");

        ctx.systems()
            .add_tick(handle_player_join)
            .add_tick(handle_player_leave);

        trace!("Vanilla-join-leave plugin loaded successfully");
    }
}

fn handle_player_join(
    mut events: EventReader<PlayerJoinEvent>,
    mut api: JoinLeaveAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        let mut message = TextComponent::from(format!(
            "{} joined the game",
            event.identity.username
        ));
        message.color = Some(Color::Named(NamedColor::Yellow));

        for (entity, _, _) in entities.iter_players() {
            if entity != event.joining_player {
                api.send_join_message(event.joining_player, entity, message.clone());
            }
        }
        
        trace!("Sent join message for {}", event.identity.username);
    }
}

fn handle_player_leave(
    mut events: EventReader<PlayerLeaveEvent>,
    mut api: JoinLeaveAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        let mut message = TextComponent::from(format!(
            "{} left the game",
            event.identity.username
        ));
        message.color = Some(Color::Named(NamedColor::Yellow));

        if let Some(reason) = &event.reason {
            message = TextComponent::from(format!(
                "{} left the game ({})",
                event.identity.username, reason
            ));
            message.color = Some(Color::Named(NamedColor::Yellow));
        }

        for (entity, _, _) in entities.iter_players() {
            api.send_leave_message(event.leaving_player, entity, message.clone());
        }
        
        trace!("Sent leave message for {}", event.identity.username);
    }
}
