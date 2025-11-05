//! Test plugin that attempts to violate the enforcement boundaries
//!
//! This plugin should FAIL to compile. It tests that:
//! 1. Plugins cannot import StreamWriter
//! 2. Plugins cannot use raw Query
//! 3. Plugins cannot import bevy_ecs directly
//!
//! To test enforcement: `cargo check -p ferrumc-test-violations`
//! Expected result: COMPILATION FAILURE

use ferrumc_plugin_api::prelude::*;

// ❌ TEST 1: Try to import StreamWriter (should fail - not in prelude)
// Uncomment to test:
// use ferrumc_net::connection::StreamWriter;

// ❌ TEST 2: Try to import bevy_ecs Query directly (should fail - not a dependency)
// Uncomment to test:
// use bevy_ecs::system::Query;

// ❌ TEST 3: Try to use Query in system (should fail - not in prelude)
// Uncomment to test:
// fn bad_system(query: Query<&StreamWriter>) {
//     // This should fail to compile
// }

#[derive(Default)]
pub struct TestViolationsPlugin;

impl Plugin for TestViolationsPlugin {
    fn name(&self) -> &'static str {
        "test-violations"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "Test"
    }

    fn description(&self) -> &'static str {
        "Test plugin to verify enforcement"
    }

    fn priority(&self) -> i32 {
        0
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder().build()
    }

    fn build(&self, ctx: PluginBuildContext<'_>) {
        // This should compile fine - using only safe APIs
    }
}
