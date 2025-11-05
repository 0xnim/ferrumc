//! Core block I/O systems
//!
//! These systems handle the I/O layer for blocks:
//! - Converting network packets into high-level events
//! - Converting block update requests into network packets

mod block_operations;
mod broadcasters;
mod packet_handlers;

pub use block_operations::{handle_break_block_requests, handle_place_block_requests};
pub use broadcasters::*;
pub use packet_handlers::*;
