//! # FerrumC Server API Implementation
//!
//! This crate provides the server-side implementation of the FerrumC mod API.
//! It includes:
//! - Behavior registries for blocks, entities, and items
//! - Mod loader for managing mod lifecycle
//! - Component provider registry for entity setup
//! - Server API implementation

pub mod error;
pub mod mod_loader;
pub mod registries;
pub mod server_api;
pub mod world_accessor;

pub use error::ApiError;
pub use mod_loader::{get_registered_mods, register_mod, ModLoader};
pub use registries::{
    BlockBehaviorRegistry, ComponentProviderRegistry, EntityBehaviorRegistry, ItemBehaviorRegistry,
};
pub use server_api::ServerApiImpl;
pub use world_accessor::{WorldAccessor, WorldAccessorResource};

/// Re-export API types for convenience
pub use ferrumc_api::prelude::*;
pub use ferrumc_api::provider;
pub use ferrumc_api::{CoreApi, ModSystem, ServerApi};
