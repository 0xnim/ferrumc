//! Player dimension component.
//!
//! Tracks which dimension the player is currently in.

use bevy_ecs::prelude::Component;

/// The dimension/world the player is currently in.
///
/// This component is used to track which dimension a player is in,
/// avoiding hardcoded "overworld" strings throughout the codebase.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PlayerDimension {
    /// The main overworld dimension
    #[default]
    Overworld,
    /// The nether dimension
    Nether,
    /// The end dimension
    TheEnd,
}

impl PlayerDimension {
    /// Get the string identifier for this dimension.
    pub fn as_str(&self) -> &'static str {
        match self {
            PlayerDimension::Overworld => "overworld",
            PlayerDimension::Nether => "the_nether",
            PlayerDimension::TheEnd => "the_end",
        }
    }
}

impl std::fmt::Display for PlayerDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
