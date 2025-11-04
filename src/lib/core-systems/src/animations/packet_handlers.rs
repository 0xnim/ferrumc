//! Packet handlers for animations
//!
//! Converts raw network packets into high-level animation events.

use bevy_ecs::prelude::*;
use ferrumc_animation_api::{Hand, PlayerSwingArmEvent};
use ferrumc_net::SwingArmPacketReceiver;

/// Core system: Converts SwingArmPacket into PlayerSwingArmEvent
pub fn handle_swing_arm_packets(
    packets: Res<SwingArmPacketReceiver>,
    mut events: EventWriter<PlayerSwingArmEvent>,
) {
    for (packet, entity) in packets.0.try_iter() {
        events.write(PlayerSwingArmEvent {
            player: entity,
            hand: Hand::from(packet.hand.0),
        });
    }
}
