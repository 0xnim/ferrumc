//! Animations Plugin for FerrumC
//!
//! This plugin implements the gameplay logic for entity animations.
//! It listens to high-level events (from the animation API) and decides
//! when to trigger animations using the AnimationAPI trait.
//!
//! # Architecture
//!
//! - Core converts packets → events
//! - This plugin reads events and applies game logic
//! - Plugin uses AnimationAPI to request animations
//! - Core converts animation requests → packets
//!
//! # Example Flow
//!
//! 1. Player swings arm in game
//! 2. Network layer receives SwingArmPacket
//! 3. Core emits PlayerSwingArmEvent
//! 4. **This plugin** reads the event
//! 5. **This plugin** decides to show the animation
//! 6. **This plugin** calls world.play_animation()
//! 7. Core receives PlayAnimationRequest
//! 8. Core broadcasts EntityAnimationPacket to players

use ferrumc_plugin_api::prelude::*;
use ferrumc_animation_api::{
    AnimationAPI, AnimationType, EntityPose, Hand, PlayerCommand, PlayerCommandEvent,
    PlayerSwingArmEvent, PlayAnimationRequest, SetEntityPoseRequest,
};
use tracing::info;

#[derive(Default)]
pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn name(&self) -> &'static str {
        "animations"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Handles entity animations like arm swings and damage effects"
    }

    fn priority(&self) -> i32 {
        50 // Base system - calculates which animations to play
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_animation_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        info!("Loading animations plugin");

        // Register events from animation API
        ctx.events()
            .register::<PlayerSwingArmEvent>()
            .register::<PlayerCommandEvent>()
            .register::<PlayAnimationRequest>()
            .register::<SetEntityPoseRequest>();

        // Register our gameplay logic systems
        ctx.systems()
            .add_tick(handle_player_swings)
            .add_tick(handle_player_commands);

        info!("Animations plugin loaded successfully");
    }
}

/// Plugin system: Handle player arm swings
///
/// This is pure game logic - when a player swings their arm,
/// we trigger the appropriate animation using the AnimationAPI.
///
/// # Game Logic
///
/// - Main hand swing → SwingMainArm animation
/// - Off hand swing → SwingOffhand animation
///
/// The animation is broadcast to nearby players automatically
/// by the core broadcasting system.
fn handle_player_swings(
    mut events: EventReader<PlayerSwingArmEvent>,
    mut animations: AnimationAPI,
) {
    for event in events.read() {
        // Game logic: Determine which animation to play
        let animation = match event.hand {
            Hand::Main => AnimationType::SwingMainArm,
            Hand::Off => AnimationType::SwingOffhand,
        };

        // Use the Animation API to request the animation
        // Clean, high-level API - no knowledge of events or packets!
        animations.play_animation(event.player, animation);
    }
}

/// Plugin system: Handle player commands (sneak, sprint, etc.)
///
/// This is pure game logic - when a player executes a command,
/// we update their pose/stance appropriately.
///
/// # Game Logic
///
/// - StartSneaking → Set pose to Sneaking
/// - StopSneaking → Set pose to Standing
/// - LeaveBed → Play LeaveBed animation + set pose to Standing
///
/// Future: StartSprinting, StopSprinting, etc.
fn handle_player_commands(
    mut events: EventReader<PlayerCommandEvent>,
    mut animations: AnimationAPI,
) {
    for event in events.read() {
        match event.command {
            PlayerCommand::StartSneaking => {
                animations.set_pose(event.player, event.entity_id, EntityPose::Sneaking);
            }
            PlayerCommand::StopSneaking => {
                animations.set_pose(event.player, event.entity_id, EntityPose::Standing);
            }
            PlayerCommand::LeaveBed => {
                // Play the leave bed animation
                animations.play_animation(event.player, AnimationType::LeaveBed);
                // Return to standing pose
                animations.set_pose(event.player, event.entity_id, EntityPose::Standing);
            }
            PlayerCommand::StartSprinting => {
                animations.set_pose(event.player, event.entity_id, EntityPose::Sprinting);
            }
            PlayerCommand::StopSprinting => {
                animations.set_pose(event.player, event.entity_id, EntityPose::Standing);
            }
            // Other commands not yet implemented
            _ => {}
        }
    }
}
