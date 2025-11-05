//! Inventory API for FerrumC
//!
//! This crate defines the domain API for inventory management.
//! 
//! # Architecture
//!
//! - **Events**: Input events from packet handlers (SetCreativeSlotEvent, SetHeldItemEvent)
//! - **Traits**: API for plugins to interact with inventories (InventoryAPI)
//!
//! # Usage
//!
//! Plugins listen to inventory events and use the InventoryAPI to respond:
//!
//! ```rust,ignore
//! fn handle_creative_slot(
//!     mut events: EventReader<SetCreativeSlotEvent>,
//!     mut inventory_api: InventoryAPI,
//! ) {
//!     for event in events.read() {
//!         // Handle creative slot change
//!     }
//! }
//! ```

pub mod events;
pub mod traits;

pub use events::*;
