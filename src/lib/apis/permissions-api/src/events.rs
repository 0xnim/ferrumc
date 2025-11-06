use bevy_ecs::prelude::*;

#[derive(Event, Clone, Debug)]
pub struct PermissionCheckEvent {
    pub player: Entity,
    pub permission: String,
}

#[derive(Event, Clone, Debug)]
pub struct PermissionGrantedEvent {
    pub player: Entity,
    pub permission: String,
    pub value: bool,
}

#[derive(Event, Clone, Debug)]
pub struct PermissionRevokedEvent {
    pub player: Entity,
    pub permission: String,
}

#[derive(Event, Clone, Debug)]
pub struct GroupAddedEvent {
    pub player: Entity,
    pub group: String,
}

#[derive(Event, Clone, Debug)]
pub struct GroupRemovedEvent {
    pub player: Entity,
    pub group: String,
}

#[derive(Event, Clone, Debug)]
pub struct PermissionsReloadEvent;
