//! Legacy MQ Queue Bridge
//!
//! Bridges the old command system (which uses ferrumc_core::mq)
//! to the new chat API. Once all commands are migrated to use ChatAPI
//! directly, this module can be removed.

use ferrumc_chat_api::ChatAPI;

/// Drain the legacy mq queue and convert to chat API events
pub fn drain_legacy_mq_queue(mut chat: ChatAPI) {
    while !ferrumc_core::mq::QUEUE.is_empty() {
        let Some(entry) = ferrumc_core::mq::QUEUE.pop() else {
            break;
        };

        match entry.receiver {
            Some(receiver) => {
                if entry.overlay {
                    chat.send_actionbar(receiver, entry.message);
                } else {
                    chat.send(receiver, entry.message);
                }
            }
            None => {
                if entry.overlay {
                    chat.broadcast_actionbar(entry.message);
                } else {
                    chat.broadcast(entry.message);
                }
            }
        }
    }
}
