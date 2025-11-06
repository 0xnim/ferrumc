use bevy_ecs::prelude::*;

#[derive(Event, Clone, Debug)]
pub struct PermissionCheckEvent {
    pub player: Entity,
    pub permission: String,
}

/// Event emitted when a permission is granted via PermissionsAPI
///
/// **NOTE:** This is `pub(crate)` - plugins cannot create this directly.
/// Automatically emitted by `PermissionsAPI::set_permission()`.
#[derive(Event, Clone, Debug)]
pub struct PermissionGrantedEvent {
    pub(crate) player: Entity,
    pub(crate) permission: String,
    pub(crate) value: bool,
}

#[derive(Event, Clone, Debug)]
pub struct PermissionRevokedEvent {
    pub player: Entity,
    pub permission: String,
}

/// Event emitted when a group is added via PermissionsAPI
///
/// **NOTE:** This is `pub(crate)` - plugins cannot create this directly.
/// Automatically emitted by `PermissionsAPI::add_group()`.
#[derive(Event, Clone, Debug)]
pub struct GroupAddedEvent {
    pub(crate) player: Entity,
    pub(crate) group: String,
}

#[derive(Event, Clone, Debug)]
pub struct GroupRemovedEvent {
    pub player: Entity,
    pub group: String,
}

#[derive(Event, Clone, Debug)]
pub struct PermissionsReloadEvent;
