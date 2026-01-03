//! Mod loader for managing mod registration and lifecycle.
//!
//! Mods are registered statically at startup and initialized
//! in dependency order during server startup.

use std::sync::{Arc, OnceLock, RwLock};

use ferrumc_api::ModSystem;
use tracing::{debug, info, warn};

use crate::error::ApiError;

/// Global mod registry.
static MOD_REGISTRY: OnceLock<RwLock<Vec<Arc<dyn ModSystem>>>> = OnceLock::new();

/// Register a mod with the mod loader.
///
/// This should be called during static initialization (e.g., via `ctor`).
/// Mods will be initialized in dependency order during server startup.
///
/// # Example
///
/// ```ignore
/// use ferrumc_api_server::register_mod;
///
/// #[ctor::ctor]
/// fn register() {
///     register_mod(Arc::new(MySurvivalMod));
/// }
/// ```
pub fn register_mod(mod_system: Arc<dyn ModSystem>) {
    let registry = MOD_REGISTRY.get_or_init(|| RwLock::new(Vec::new()));
    let mut mods = registry.write().expect("Failed to lock mod registry");

    let mod_id = mod_system.mod_id();

    // Check for duplicate registration
    if mods.iter().any(|m| m.mod_id() == mod_id) {
        warn!("Mod '{}' is already registered, skipping duplicate", mod_id);
        return;
    }

    debug!("Registered mod: {} v{}", mod_id, mod_system.version());
    mods.push(mod_system);
}

/// Get all registered mods in registration order.
pub fn get_registered_mods() -> Vec<Arc<dyn ModSystem>> {
    MOD_REGISTRY
        .get()
        .map(|registry| {
            registry
                .read()
                .expect("Failed to lock mod registry")
                .clone()
        })
        .unwrap_or_default()
}

/// Mod loader that manages mod lifecycle.
pub struct ModLoader {
    mods: Vec<Arc<dyn ModSystem>>,
    load_order: Vec<usize>,
}

impl ModLoader {
    /// Create a new mod loader with the registered mods.
    pub fn new() -> Self {
        let mods = get_registered_mods();
        Self {
            mods,
            load_order: Vec::new(),
        }
    }

    /// Resolve dependencies and determine load order.
    ///
    /// Returns an error if dependencies cannot be satisfied.
    pub fn resolve_dependencies(&mut self) -> Result<(), ApiError> {
        let mod_count = self.mods.len();
        let mut resolved = vec![false; mod_count];
        let mut load_order = Vec::with_capacity(mod_count);

        // Build mod ID to index map
        let mod_indices: std::collections::HashMap<&str, usize> = self
            .mods
            .iter()
            .enumerate()
            .map(|(i, m)| (m.mod_id(), i))
            .collect();

        // Topological sort with cycle detection
        fn visit(
            idx: usize,
            mods: &[Arc<dyn ModSystem>],
            mod_indices: &std::collections::HashMap<&str, usize>,
            resolved: &mut [bool],
            visiting: &mut [bool],
            order: &mut Vec<usize>,
        ) -> Result<(), ApiError> {
            if resolved[idx] {
                return Ok(());
            }
            if visiting[idx] {
                return Err(ApiError::CircularDependency(
                    mods[idx].mod_id().to_string(),
                ));
            }

            visiting[idx] = true;

            for dep_id in mods[idx].dependencies() {
                if let Some(&dep_idx) = mod_indices.get(dep_id) {
                    visit(dep_idx, mods, mod_indices, resolved, visiting, order)?;
                } else {
                    return Err(ApiError::MissingDependency(
                        mods[idx].mod_id().to_string(),
                        dep_id.to_string(),
                    ));
                }
            }

            visiting[idx] = false;
            resolved[idx] = true;
            order.push(idx);
            Ok(())
        }

        let mut visiting = vec![false; mod_count];
        for i in 0..mod_count {
            visit(
                i,
                &self.mods,
                &mod_indices,
                &mut resolved,
                &mut visiting,
                &mut load_order,
            )?;
        }

        self.load_order = load_order;

        info!(
            "Resolved mod load order: {:?}",
            self.load_order
                .iter()
                .map(|&i| self.mods[i].mod_id())
                .collect::<Vec<_>>()
        );

        Ok(())
    }

    /// Get mods in load order.
    ///
    /// Call `resolve_dependencies` first to populate load order.
    pub fn mods_in_order(&self) -> impl Iterator<Item = &Arc<dyn ModSystem>> {
        self.load_order.iter().map(|&i| &self.mods[i])
    }

    /// Get a mod by ID.
    pub fn get_mod(&self, mod_id: &str) -> Option<&Arc<dyn ModSystem>> {
        self.mods.iter().find(|m| m.mod_id() == mod_id)
    }

    /// Check if a mod is loaded.
    pub fn is_mod_loaded(&self, mod_id: &str) -> bool {
        self.mods.iter().any(|m| m.mod_id() == mod_id)
    }

    /// Get the number of registered mods.
    pub fn mod_count(&self) -> usize {
        self.mods.len()
    }
}

impl Default for ModLoader {
    fn default() -> Self {
        Self::new()
    }
}
