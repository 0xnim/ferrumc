//! Animation API for FerrumC
//!
//! This crate provides the domain API for entity animations. It sits between
//! the core infrastructure and gameplay plugins.
//!
//! # Architecture
//!
//! - **Core** converts network packets into high-level events
//! - **This API** defines the events and traits
//! - **Plugins** use the API to trigger animations
//! - **Core** converts animation requests back into network packets
//!
//! # Example
//!
//! ```rust,no_run
//! use ferrumc_animation_api::*;
//! use bevy_ecs::prelude::*;
//!
//! fn handle_player_swings(
//!     mut events: EventReader<PlayerSwingArmEvent>,
//!     world: &mut World,
//! ) {
//!     for event in events.read() {
//!         let animation = match event.hand {
//!             Hand::Main => AnimationType::SwingMainArm,
//!             Hand::Off => AnimationType::SwingOffhand,
//!         };
//!         world.play_animation(event.player, animation);
//!     }
//! }
//! ```

pub mod events;
pub mod traits;
pub mod types;

pub use events::*;
pub use traits::*;
pub use types::*;
