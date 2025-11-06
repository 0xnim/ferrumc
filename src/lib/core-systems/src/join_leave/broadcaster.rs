//! Join/Leave Message Broadcaster
//!
//! This module handles the I/O for join/leave messages.
//! - Plugins format messages and send requests via JoinLeaveAPI
//! - This system reads those requests and delivers them (network I/O)

use bevy_ecs::prelude::*;
use ferrumc_chat_api::ChatAPI;
use ferrumc_join_leave_api::{SendJoinMessageRequest, SendLeaveMessageRequest};

/// Broadcast join messages to players
///
/// This is PURE I/O - no game logic!
/// - Reads SendJoinMessageRequest events from plugins
/// - Delivers messages via ChatAPI
pub fn broadcast_join_messages(
    mut events: EventReader<SendJoinMessageRequest>,
    mut chat: ChatAPI,
) {
    for request in events.read() {
        chat.send(request.receiver, request.message.clone());
    }
}

/// Broadcast leave messages to players
///
/// This is PURE I/O - no game logic!
/// - Reads SendLeaveMessageRequest events from plugins
/// - Delivers messages via ChatAPI
pub fn broadcast_leave_messages(
    mut events: EventReader<SendLeaveMessageRequest>,
    mut chat: ChatAPI,
) {
    for request in events.read() {
        chat.send(request.receiver, request.message.clone());
    }
}
