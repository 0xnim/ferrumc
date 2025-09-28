use bevy_ecs::prelude::Component;
use typename::TypeName;

/// Component that marks a player as needing initial chunk loading beyond the login chunks.
/// This is removed after the initial chunk loading is complete.
#[derive(TypeName, Component)]
pub struct NeedsInitialChunks {
    pub target_radius: u32,
}

impl NeedsInitialChunks {
    pub fn new(target_radius: u32) -> Self {
        Self { target_radius }
    }
}
