//! Inventory Sync Plugin (Core)
//!
//! Updates server-side inventory state from network packets.
//! NO broadcasting - vanilla plugin handles that.

use ferrumc_plugin_api::prelude::*;
use ferrumc_inventory_api::{SendInventoryUpdateRequest, SetCreativeSlotEvent, SetHeldItemEvent};
use tracing::trace;

#[derive(Default)]
pub struct InventorySyncPlugin;

impl Plugin for InventorySyncPlugin {
    fn name(&self) -> &'static str {
        "inventory-sync"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "Core inventory state synchronization - updates server state (no broadcasting)"
    }

    fn priority(&self) -> i32 {
        1000
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .with_inventory_api()
            .build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        trace!("Loading inventory-sync plugin");

        trace!("Inventory-sync plugin loaded successfully");
    }
}
