//! Inventory I/O systems
//!
//! Converts inventory-related packets into domain events.

pub mod broadcaster;
mod packet_handlers;

pub use packet_handlers::*;
