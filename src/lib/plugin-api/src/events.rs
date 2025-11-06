//! Common event types for plugins
//!
//! This module previously contained PlayerJoinEvent and PlayerLeaveEvent,
//! but those have been moved to ferrumc_join_leave_api to avoid duplication
//! and provide better type safety.
//!
//! Domain-specific events are in their respective API crates:
//! - Join/Leave: ferrumc_join_leave_api
//! - Blocks: ferrumc_block_api
//! - Chat: ferrumc_chat_api
//! - Animation: ferrumc_animation_api
//! - etc.
//!
//! This file is kept as a placeholder for future generic plugin events
//! that don't belong to a specific domain.


