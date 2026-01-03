//! # FerrumC Server Core
//!
//! This mod provides essential server administration functionality:
//! - `/tps` - Server performance monitoring
//! - `/gamemode` - Change player game modes
//!
//! This is a core mod that most servers will want to include.
//!
//! ## Usage
//!
//! Add `ferrumc-server-core` as a dependency to automatically register
//! the server core mod.
//!
//! ```toml
//! [dependencies]
//! ferrumc-server-core = { path = "../server-core" }
//! ```

mod commands;
pub mod systems;

use std::sync::Arc;

use ferrumc_api::{CoreApi, ModSystem, ServerApi};
use ferrumc_api_server::register_mod;
use tracing::info;

/// The server core mod providing essential admin commands.
pub struct ServerCoreMod;

impl ModSystem for ServerCoreMod {
    fn mod_id(&self) -> &'static str {
        "ferrumc:server-core"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn start(&self, _api: &mut dyn CoreApi) {
        info!("Server Core mod starting...");
        // Commands are registered via #[command] macro and ctor
    }

    fn start_server_side(&self, _api: &mut dyn ServerApi) {
        info!("Server Core mod registering systems...");
        // Systems are registered by the binary directly since they need
        // access to the Bevy schedule. See systems module for exports.
    }

    fn assets_loaded(&self, _api: &mut dyn ServerApi) {
        info!("Server Core mod assets loaded");
    }

    fn assets_finalize(&self, _api: &mut dyn ServerApi) {
        info!("Server Core mod initialized successfully");
    }
}

/// Register the server core mod during static initialization.
#[ctor::ctor]
fn register_server_core_mod() {
    register_mod(Arc::new(ServerCoreMod));
}
