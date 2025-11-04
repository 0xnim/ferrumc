//! Player command packet handler
//!
//! Converts PlayerCommandPacket into PlayerCommandEvent

use bevy_ecs::prelude::*;
use ferrumc_animation_api::{PlayerCommand, PlayerCommandEvent};
use ferrumc_net::packets::incoming::player_command::PlayerCommandAction;
use ferrumc_net::PlayerCommandPacketReceiver;

/// Core system: Converts PlayerCommandPacket into PlayerCommandEvent
pub fn handle_player_command_packets(
    packets: Res<PlayerCommandPacketReceiver>,
    mut events: EventWriter<PlayerCommandEvent>,
) {
    for (packet, entity) in packets.0.try_iter() {
        let command = match packet.action {
            PlayerCommandAction::StartSneaking => PlayerCommand::StartSneaking,
            PlayerCommandAction::StopSneaking => PlayerCommand::StopSneaking,
            PlayerCommandAction::LeaveBed => PlayerCommand::LeaveBed,
            PlayerCommandAction::StartSprinting => PlayerCommand::StartSprinting,
            PlayerCommandAction::StopSprinting => PlayerCommand::StopSprinting,
            PlayerCommandAction::StartJumpWithHorse => PlayerCommand::StartJumpWithHorse,
            PlayerCommandAction::StopJumpWithHorse => PlayerCommand::StopJumpWithHorse,
            PlayerCommandAction::OpenVehicleInventory => PlayerCommand::OpenVehicleInventory,
            PlayerCommandAction::StartFlyingWithElytra => PlayerCommand::StartFlyingWithElytra,
        };

        events.write(PlayerCommandEvent {
            player: entity,
            command,
            entity_id: packet.entity_id,
            jump_boost: packet.jump_boost,
        });
    }
}
