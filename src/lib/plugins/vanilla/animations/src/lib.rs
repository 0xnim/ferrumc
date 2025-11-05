//! Vanilla Animations Plugin
//!
//! Implements vanilla Minecraft animation logic and broadcasting.

use ferrumc_plugin_api::prelude::*;
use ferrumc_animation_api::{
    AnimationAPI, AnimationType, EntityPose, PlayerCommandEvent, PlayerSwingArmEvent,
    PlayAnimationRequest, SetEntityPoseRequest,
};
use tracing::trace;

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

        ctx.events()
            .register::<PlayerSwingArmEvent>()
            .register::<PlayerCommandEvent>()
            .register::<PlayAnimationRequest>()
            .register::<SetEntityPoseRequest>();

        ctx.systems()
            .add_tick(handle_swing_arm)
            .add_tick(handle_player_command);

        trace!("Vanilla-animations plugin loaded successfully");
    }
}

fn handle_swing_arm(
    mut events: EventReader<PlayerSwingArmEvent>,
    mut api: AnimationAPI,
) {
    for event in events.read() {
        use ferrumc_animation_api::Hand;
        
        let animation = match event.hand {
            Hand::Main => AnimationType::SwingMainArm,
            Hand::Off => AnimationType::SwingOffhand,
        };
        
        // Vanilla: Broadcast to all players
        api.play_animation_global(event.player, animation);
        
        trace!("Played swing animation for player {}", event.player.index());
    }
}

fn handle_player_command(
    mut events: EventReader<PlayerCommandEvent>,
    mut api: AnimationAPI,
) {
    for event in events.read() {
        use ferrumc_animation_api::PlayerCommand;
        
        let pose = match event.command {
            PlayerCommand::StartSneaking => EntityPose::Sneaking,
            PlayerCommand::StopSneaking => EntityPose::Standing,
            PlayerCommand::StartSprinting => EntityPose::Sprinting,
            PlayerCommand::StopSprinting => EntityPose::Standing,
            PlayerCommand::StartFlyingWithElytra => EntityPose::FlyingWithElytra,
            _ => continue,
        };
        
        // Vanilla: Broadcast to all players
        api.set_pose_global(event.player, event.entity_id, pose);
        
        trace!("Set pose {:?} for player {}", pose, event.player.index());
    }
}
