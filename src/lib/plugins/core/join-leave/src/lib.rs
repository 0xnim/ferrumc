//! Join/Leave Plugin for FerrumC
//!
//! This plugin handles player join and leave message formatting.
//!
//! # Architecture
//!
//! - Core fires PlayerJoinEvent / PlayerLeaveEvent
//! - This plugin formats messages (game logic)
//! - Plugin uses JoinLeaveAPI to send messages
//! - Core handles actual message delivery (I/O)

use ferrumc_plugin_api::prelude::*;
use ferrumc_join_leave_api::{JoinLeaveAPI, PlayerJoinEvent, PlayerLeaveEvent};
use ferrumc_text::{Color, NamedColor, TextComponent};
use tracing::info;

#[derive(Default)]
pub struct JoinLeavePlugin;

impl Plugin for JoinLeavePlugin {
    fn name(&self) -> &'static str {
        "join-leave"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Handles player join and leave message formatting"
    }

    fn priority(&self) -> i32 {
        20 // Run before chat broadcasting
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_join_leave_api()
            .with_entity_queries() // To iterate all players
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        info!("Loading join/leave plugin");

        // Register events from join-leave API
        ctx.events()
            .register::<PlayerJoinEvent>()
            .register::<PlayerLeaveEvent>()
            .register::<ferrumc_join_leave_api::SendJoinMessageRequest>()
            .register::<ferrumc_join_leave_api::SendLeaveMessageRequest>();

        // Register our gameplay logic systems
        ctx.systems()
            .add_tick(handle_player_join)
            .add_tick(handle_player_leave);

        info!("Join/leave plugin loaded successfully");
    }
}

/// Handle player join events
///
/// This is pure game logic:
/// - Formats the join message
/// - Sends to all OTHER players (not the joining player)
fn handle_player_join(
    mut events: EventReader<PlayerJoinEvent>,
    mut api: JoinLeaveAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        // Format join message (game logic)
        let mut message = TextComponent::from(format!(
            "{} joined the game",
            event.identity.username
        ));
        message.color = Some(Color::Named(NamedColor::Yellow));

        // Send to all OTHER players
        for (entity, _, _) in entities.iter_players() {
            if entity != event.joining_player {
                api.send_join_message(event.joining_player, entity, message.clone());
            }
        }
    }
}

/// Handle player leave events
///
/// This is pure game logic:
/// - Formats the leave message
/// - Sends to all remaining players
fn handle_player_leave(
    mut events: EventReader<PlayerLeaveEvent>,
    mut api: JoinLeaveAPI,
    entities: EntityQueries,
) {
    for event in events.read() {
        // Format leave message (game logic)
        let mut message = TextComponent::from(format!(
            "{} left the game",
            event.identity.username
        ));
        message.color = Some(Color::Named(NamedColor::Yellow));

        // Optional: add reason if provided
        if let Some(reason) = &event.reason {
            message = TextComponent::from(format!(
                "{} left the game ({})",
                event.identity.username, reason
            ));
            message.color = Some(Color::Named(NamedColor::Yellow));
        }

        // Send to all remaining players
        for (entity, _, _) in entities.iter_players() {
            api.send_leave_message(event.leaving_player, entity, message.clone());
        }
    }
}
