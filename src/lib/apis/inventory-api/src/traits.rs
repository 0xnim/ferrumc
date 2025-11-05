//! Inventory API trait
//!
//! This provides the high-level interface for plugins to interact with inventories.
//!
//! Future: This will provide methods for inventory operations like:
//! - give_item(player: Entity, item: ItemStack)
//! - remove_item(player: Entity, slot: usize)
//! - etc.
//!
//! For now, plugins interact directly with Inventory and Hotbar components.
