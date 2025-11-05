//! Hello World Plugin
//!
//! A simple example plugin that demonstrates the FerrumC plugin API.
//!
//! This plugin:
//! - Adds a `/hello` command
//! - Runs a periodic system that logs a message every 10 seconds
//! - Shows how to use plugin configuration

use ferrumc_plugin_api::*;
use std::time::Duration;
use tracing::info;

/// Hello World plugin
#[derive(Default)]
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn name(&self) -> &'static str {
        "hello"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "A simple hello world plugin demonstrating the plugin API"
    }

    fn priority(&self) -> i32 {
        0 // Example plugin, no specific priority needed
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder()
            .build() // No special capabilities needed
    }

    fn build(&self, mut ctx: PluginBuildContext) {
        // Get configuration (with defaults)
        let message = ctx
            .config()
            .get_string("message")
            .unwrap_or_else(|| "Hello from plugin!".to_string());
        let interval_secs = ctx.config().get_int("interval_seconds").unwrap_or(10) as u64;

        info!("Hello plugin initialized with message: '{}'", message);
        info!("Will log every {} seconds", interval_secs);

        // Register a periodic system
        ctx.systems().add_timed(
            "hello_periodic",
            Duration::from_secs(interval_secs),
            move || {
                info!("🎉 {}", message);
            },
        );

        // TODO: Register /hello command when command registration is fixed
        // ctx.register_command(hello_command);

        info!("Hello plugin loaded successfully!");
    }
}

// Example command (not yet functional due to command system integration)
// #[command]
// fn hello_command(sender: Sender) {
//     sender.send_system_message("Hello from the hello plugin!");
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = HelloPlugin;
        assert_eq!(plugin.name(), "hello");
        assert_eq!(plugin.version(), "1.0.0");
        assert!(!plugin.description().is_empty());
    }
}
