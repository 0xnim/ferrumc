//! Core I/O Systems for FerrumC
//!
//! This crate contains the I/O layer systems that convert between
//! network packets and high-level domain events.
//!
//! # Architecture
//!
//! - Network → Packet Handler → Domain Event → Plugin
//! - Plugin → Request Event → Broadcaster → Network
//!
//! This crate sits at the I/O boundary and should ONLY contain:
//! - Packet → Event converters
//! - Event → Packet broadcasters
//!
//! Game logic belongs in plugins!

pub mod animations;
pub mod blocks;
pub mod chat;
pub mod commands;
pub mod inventory;
pub mod join_leave;
pub mod movement;
