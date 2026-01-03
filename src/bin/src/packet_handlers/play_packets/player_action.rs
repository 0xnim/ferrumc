//! Player action packet handler.
//!
//! This handler fires events for all player actions (digging, etc.).
//! Game mode specific logic is handled by mod systems:
//! - Creative instant-break: `ferrumc-creative` systems
//! - Survival digging timer: `digging_system.rs`

use bevy_ecs::prelude::{MessageWriter, Res};
use ferrumc_messages::player_digging::*;
use ferrumc_net::PlayerActionReceiver;
use tracing::trace;

pub fn handle(
    receiver: Res<PlayerActionReceiver>,
    (mut start_dig_events, mut cancel_dig_events, mut finish_dig_events): (
        MessageWriter<PlayerStartedDigging>,
        MessageWriter<PlayerCancelledDigging>,
        MessageWriter<PlayerFinishedDigging>,
    ),
) {
    // https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2773393#Player_Action
    for (event, trigger_eid) in receiver.0.try_iter() {
        // Fire events for all game modes - mode-specific handling is done by mod systems
        match event.status.0 {
            0 => {
                // Started digging
                trace!("Player {:?} started digging at {:?}", trigger_eid, event.location);
                start_dig_events.write(PlayerStartedDigging {
                    player: trigger_eid,
                    position: event.location,
                    sequence: event.sequence,
                });
            }
            1 => {
                // Cancelled digging
                trace!("Player {:?} cancelled digging at {:?}", trigger_eid, event.location);
                cancel_dig_events.write(PlayerCancelledDigging {
                    player: trigger_eid,
                    position: event.location,
                    sequence: event.sequence,
                });
            }
            2 => {
                // Finished digging
                trace!("Player {:?} finished digging at {:?}", trigger_eid, event.location);
                finish_dig_events.write(PlayerFinishedDigging {
                    player: trigger_eid,
                    position: event.location,
                    sequence: event.sequence,
                });
            }
            _ => {} // Other statuses (drop item, etc.) are handled by different packets
        }
    }
}
