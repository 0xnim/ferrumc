//! Command implementations

use ferrumc_plugin_api::PluginBuildContext;

pub mod communication;

/// Register all command handlers
pub fn register(_ctx: &mut PluginBuildContext<'_>) {
    // Command handlers will be registered here
    // For now, we're using the legacy command system
}
