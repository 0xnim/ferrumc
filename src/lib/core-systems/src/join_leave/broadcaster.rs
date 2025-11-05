//! Join/Leave Message Broadcaster
//!
//! This module handles the I/O for join/leave messages.
//! - Plugins format messages and send requests via JoinLeaveAPI
//! - This system reads those requests and delivers them (network I/O)

use bevy_ecs::prelude::*;
use ferrumc_core::mq;
use ferrumc_join_leave_api::{SendJoinMessageRequest, SendLeaveMessageRequest};

/// Broadcast join messages to players
///
/// This is PURE I/O - no game logic!
/// - Reads SendJoinMessageRequest events from plugins
/// - Queues messages to the legacy mq system
pub fn broadcast_join_messages(mut events: EventReader<SendJoinMessageRequest>) {
    for request in events.read() {
        // Queue the message for delivery (I/O only)
        mq::queue(request.message.clone(), false, request.receiver);
    }
}

/// Broadcast leave messages to players
///
/// This is PURE I/O - no game logic!
/// - Reads SendLeaveMessageRequest events from plugins
/// - Queues messages to the legacy mq system
pub fn broadcast_leave_messages(mut events: EventReader<SendLeaveMessageRequest>) {
    for request in events.read() {
        // Queue the message for delivery (I/O only)
        mq::queue(request.message.clone(), false, request.receiver);
    }
}
