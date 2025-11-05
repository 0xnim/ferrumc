//! Vanilla Animations Plugin
//!
//! Implements vanilla Minecraft animation logic and broadcasting.

use ferrumc_plugin_api::prelude::*;
use ferrumc_animation_api::{
    AnimationAPI, AnimationType, EntityPose, PlayerCommandEvent, PlayerInputEvent,
    PlayerSwingArmEvent,
};
use tracing::{debug, trace};

#[derive(Default)]
pub struct VanillaAnimationsPlugin;

impl Plugin for VanillaAnimationsPlugin {
    fn name(&self) -> &'static str {
        "vanilla-animations"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Vanilla Minecraft animations - swing arm and pose changes"
    }

    fn priority(&self) -> i32 {
        50
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_animation_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        trace!("Loading vanilla-animations plugin");

        ctx.systems()
            .add_tick(handle_swing_arm)
            .add_tick(handle_player_command)
            .add_tick(handle_player_input);

        trace!("Vanilla-animations plugin loaded successfully");
    }
}

fn handle_swing_arm(
    mut events: EventReader<PlayerSwingArmEvent>,
    mut api: AnimationAPI,
) {
    let count = events.len();
    if count > 0 {
        trace!("Plugin handling {} swing arm events", count);
    }
    for event in events.read() {
        use ferrumc_animation_api::Hand;
        
        let animation = match event.hand {
            Hand::Main => AnimationType::SwingMainArm,
            Hand::Off => AnimationType::SwingOffhand,
        };
        
        // Vanilla: Broadcast to all players except the one who swung
        api.play_animation_except(event.player, animation, event.player);
        
        trace!("Played swing animation for player {}", event.player.index());
    }
}

fn handle_player_command(
    mut events: EventReader<PlayerCommandEvent>,
    mut api: AnimationAPI,
) {
    for event in events.read() {
        use ferrumc_animation_api::PlayerCommand;
        
        debug!("Received player command: {:?}", event.command);
        
        let pose = match event.command {
            PlayerCommand::StartSprinting => EntityPose::Sprinting,
            PlayerCommand::StopSprinting => EntityPose::Standing,
            PlayerCommand::StartFlyingWithElytra => EntityPose::FlyingWithElytra,
            _ => {
                debug!("Ignoring command: {:?}", event.command);
                continue;
            }
        };
        
        // Vanilla: Broadcast to all players except the one who changed pose
        api.set_pose_except(event.player, event.entity_id, pose, event.player);
        
        debug!("Set pose {:?} for player {}", pose, event.player.index());
    }
}

fn handle_player_input(
    mut events: EventReader<PlayerInputEvent>,
    mut api: AnimationAPI,
) {
    for event in events.read() {
        // Set pose based on sneak state
        let pose = if event.is_sneaking {
            EntityPose::Sneaking
        } else {
            EntityPose::Standing
        };
        
        // Vanilla: Broadcast to all players except the one who changed pose
        api.set_pose_except(event.player, event.entity_id, pose, event.player);
        
        debug!(
            "Player {} {} sneaking (flags: 0x{:02X})",
            event.player.index(),
            if event.is_sneaking { "started" } else { "stopped" },
            event.flags
        );
    }
}
