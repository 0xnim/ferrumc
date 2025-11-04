//! Core chat I/O systems
//!
//! These systems handle the I/O layer for chat:
//! - Converting network packets into high-level events
//! - Converting chat requests into network packets

mod packet_handler;
mod broadcaster;

pub use packet_handler::*;
pub use broadcaster::*;
