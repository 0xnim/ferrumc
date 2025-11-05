//! Safe query APIs for plugins
//!
//! These SystemParams provide restricted access to entity and world data.
//! Plugins cannot query sensitive components like StreamWriter or database access.

pub mod entity;
pub mod inventory;
pub mod world;

pub use entity::EntityQueries;
pub use inventory::{InventoryQueries, InventoryQueriesMut};
pub use world::WorldQueries;
