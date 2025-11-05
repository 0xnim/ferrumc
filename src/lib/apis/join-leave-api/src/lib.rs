//! Join/Leave API for FerrumC
//!
//! This API handles player join and leave events and message formatting.
//!
//! # Architecture
//!
//! **Events (Core → Plugin):**
//! - `PlayerJoinEvent` - Fired when a player joins
//! - `PlayerLeaveEvent` - Fired when a player leaves
//!
//! **Requests (Plugin → Core):**
//! - `SendJoinMessageRequest` - Request to send join message
//! - `SendLeaveMessageRequest` - Request to send leave message
//!
//! # Example
//!
//! ```rust,no_run
//! fn handle_player_join(
//!     mut events: EventReader<PlayerJoinEvent>,
//!     mut api: JoinLeaveAPI,
//! ) {
//!     for event in events.read() {
//!         // Customize the message
//!         let message = format!("{} has joined!", event.username);
//!         api.send_join_message(event.joining_player, message);
//!     }
//! }
//! ```

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_text::TextComponent;

/// Event fired when a player joins the server
///
/// Core fires this event after successful login.
/// Plugins can listen to customize join messages or perform other actions.
#[derive(Event, Clone)]
pub struct PlayerJoinEvent {
    /// The entity ID of the joining player
    pub joining_player: Entity,
    
    /// Player identity information
    pub identity: PlayerIdentity,
}

/// Event fired when a player leaves the server
///
/// Core fires this event before disconnecting.
/// Plugins can listen to customize leave messages or perform cleanup.
#[derive(Event, Clone)]
pub struct PlayerLeaveEvent {
    /// The entity ID of the leaving player
    pub leaving_player: Entity,
    
    /// Player identity information
    pub identity: PlayerIdentity,
    
    /// Optional reason for leaving (e.g., "Timed out", "Kicked")
    pub reason: Option<String>,
}

/// Request to send a join message to a specific player
///
/// Plugin sends this to core to broadcast join messages.
#[derive(Event)]
pub struct SendJoinMessageRequest {
    /// The player who joined (for reference)
    pub joining_player: Entity,
    
    /// The player who should receive this message
    pub receiver: Entity,
    
    /// The formatted message to send
    pub message: TextComponent,
}

/// Request to send a leave message to a specific player
///
/// Plugin sends this to core to broadcast leave messages.
#[derive(Event)]
pub struct SendLeaveMessageRequest {
    /// The player who left (for reference)
    pub leaving_player: Entity,
    
    /// The player who should receive this message
    pub receiver: Entity,
    
    /// The formatted message to send
    pub message: TextComponent,
}

/// JoinLeave API - SystemParam for plugins
///
/// Provides methods to send join/leave messages.
///
/// # Example
///
/// ```rust,no_run
/// fn my_system(mut api: JoinLeaveAPI) {
///     api.send_join_message(player, receiver, message);
/// }
/// ```
#[derive(SystemParam)]
pub struct JoinLeaveAPI<'w> {
    join_requests: EventWriter<'w, SendJoinMessageRequest>,
    leave_requests: EventWriter<'w, SendLeaveMessageRequest>,
}

impl<'w> JoinLeaveAPI<'w> {
    /// Send a join message to a specific player
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let message = TextComponent::from("Player joined!");
    /// api.send_join_message(joining_player, receiver, message);
    /// ```
    pub fn send_join_message(
        &mut self,
        joining_player: Entity,
        receiver: Entity,
        message: TextComponent,
    ) {
        self.join_requests.send(SendJoinMessageRequest {
            joining_player,
            receiver,
            message,
        });
    }
    
    /// Send a leave message to a specific player
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let message = TextComponent::from("Player left!");
    /// api.send_leave_message(leaving_player, receiver, message);
    /// ```
    pub fn send_leave_message(
        &mut self,
        leaving_player: Entity,
        receiver: Entity,
        message: TextComponent,
    ) {
        self.leave_requests.send(SendLeaveMessageRequest {
            leaving_player,
            receiver,
            message,
        });
    }
}
