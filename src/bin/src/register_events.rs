use bevy_ecs::event::EventRegistry;
use bevy_ecs::prelude::World;
use ferrumc_commands::events::{CommandDispatchEvent, ResolvedCommandDispatchEvent};
use ferrumc_core::chunks::cross_chunk_boundary_event::CrossChunkBoundaryEvent;
use ferrumc_core::conn::force_player_recount_event::ForcePlayerRecountEvent;
use ferrumc_net::packets::packet_events::TransformEvent;

// Domain API events
use ferrumc_animation_api::{PlayAnimationRequest, PlayerCommandEvent, PlayerSwingArmEvent, SetEntityPoseRequest};
use ferrumc_block_api::{BlockBreakAttemptEvent, BlockPlaceAttemptEvent, BreakBlockRequest, PlaceBlockRequest, SendBlockChangeAckRequest, SendBlockUpdateRequest};
use ferrumc_chat_api::{ChatMessageEvent, SendChatMessageRequest};
use ferrumc_inventory_api::{SendInventoryUpdateRequest, SetCreativeSlotEvent, SetHeldItemEvent};
use ferrumc_join_leave_api::{PlayerJoinEvent, PlayerLeaveEvent, SendJoinMessageRequest, SendLeaveMessageRequest};
use ferrumc_movement_api::{
    ApplyMovementRequest, BroadcastHeadRotationRequest, BroadcastMovementRequest,
    HeadRotationEvent, PlayerMoveAndRotateEvent, PlayerMoveEvent, PlayerRotateEvent,
};

pub fn register_events(world: &mut World) {
    // Core infrastructure events
    EventRegistry::register_event::<TransformEvent>(world);
    EventRegistry::register_event::<CrossChunkBoundaryEvent>(world);
    EventRegistry::register_event::<ForcePlayerRecountEvent>(world);
    EventRegistry::register_event::<CommandDispatchEvent>(world);
    EventRegistry::register_event::<ResolvedCommandDispatchEvent>(world);
    
    // Animation API events
    EventRegistry::register_event::<PlayerSwingArmEvent>(world);
    EventRegistry::register_event::<PlayerCommandEvent>(world);
    EventRegistry::register_event::<PlayAnimationRequest>(world);
    EventRegistry::register_event::<SetEntityPoseRequest>(world);
    
    // Block API events
    EventRegistry::register_event::<BlockPlaceAttemptEvent>(world);
    EventRegistry::register_event::<BlockBreakAttemptEvent>(world);
    EventRegistry::register_event::<PlaceBlockRequest>(world);
    EventRegistry::register_event::<BreakBlockRequest>(world);
    EventRegistry::register_event::<SendBlockUpdateRequest>(world);
    EventRegistry::register_event::<SendBlockChangeAckRequest>(world);
    
    // Chat API events
    EventRegistry::register_event::<ChatMessageEvent>(world);
    EventRegistry::register_event::<SendChatMessageRequest>(world);
    
    // Inventory API events
    EventRegistry::register_event::<SetCreativeSlotEvent>(world);
    EventRegistry::register_event::<SetHeldItemEvent>(world);
    EventRegistry::register_event::<SendInventoryUpdateRequest>(world);
    
    // Join/Leave API events
    EventRegistry::register_event::<PlayerJoinEvent>(world);
    EventRegistry::register_event::<PlayerLeaveEvent>(world);
    EventRegistry::register_event::<SendJoinMessageRequest>(world);
    EventRegistry::register_event::<SendLeaveMessageRequest>(world);
    
    // Movement API events
    EventRegistry::register_event::<PlayerMoveEvent>(world);
    EventRegistry::register_event::<PlayerRotateEvent>(world);
    EventRegistry::register_event::<PlayerMoveAndRotateEvent>(world);
    EventRegistry::register_event::<HeadRotationEvent>(world);
    EventRegistry::register_event::<ApplyMovementRequest>(world);
    EventRegistry::register_event::<BroadcastMovementRequest>(world);
    EventRegistry::register_event::<BroadcastHeadRotationRequest>(world);
}
