//! Tests for the plugin API

#[cfg(test)]
mod plugin_tests {
    use crate::{Plugin, PluginConfig};

    #[derive(Default)]
    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn name(&self) -> &'static str {
            "test_plugin"
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        fn build(&self, _ctx: &mut crate::PluginContext<'_>) {
            // Simple test plugin
        }
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = TestPlugin;
        assert_eq!(plugin.name(), "test_plugin");
        assert_eq!(plugin.version(), "1.0.0");
        assert!(plugin.dependencies().is_empty());
    }

    #[test]
    fn test_plugin_config() {
        let config = PluginConfig::new();

        // Config should be empty initially
        assert!(!config.has("key"));
        assert!(config.get::<String>("key").is_none());

        // Test default values
        let value = config.get::<i64>("interval").unwrap_or(100);
        assert_eq!(value, 100);
    }
}
