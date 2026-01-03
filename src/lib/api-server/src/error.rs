//! Error types for the server API.

use thiserror::Error;

/// Errors that can occur in the server API.
#[derive(Debug, Error)]
pub enum ApiError {
    /// A mod with the given ID is already registered.
    #[error("Mod '{0}' is already registered")]
    ModAlreadyRegistered(String),

    /// A required dependency is missing.
    #[error("Mod '{0}' requires '{1}' which is not loaded")]
    MissingDependency(String, String),

    /// Circular dependency detected.
    #[error("Circular dependency detected involving mod '{0}'")]
    CircularDependency(String),

    /// Mod initialization failed.
    #[error("Mod '{0}' failed to initialize: {1}")]
    InitializationFailed(String, String),

    /// Invalid mod ID format.
    #[error("Invalid mod ID '{0}': must be in format 'namespace:name'")]
    InvalidModId(String),

    /// Behavior registration error.
    #[error("Failed to register behavior: {0}")]
    BehaviorRegistration(String),
}
