//! Core block I/O systems
//!
//! These systems handle the I/O layer for blocks:
//! - Converting network packets into high-level events
//! - Converting block update requests into network packets

mod packet_handlers;
mod broadcasters;

pub use packet_handlers::*;
pub use broadcasters::*;
