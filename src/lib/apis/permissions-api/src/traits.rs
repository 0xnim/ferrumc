use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use ferrumc_core::permissions::{PermissionGroup, PlayerPermissions};
use std::collections::HashMap;

use crate::events::*;

#[derive(SystemParam)]
pub struct PermissionsAPI<'w, 's> {
    permissions_query: Query<'w, 's, &'static mut PlayerPermissions>,
    groups: Res<'w, PermissionGroups>,
    events: EventWriter<'w, PermissionGrantedEvent>,
    group_events: EventWriter<'w, GroupAddedEvent>,
}

impl<'w, 's> PermissionsAPI<'w, 's> {
    pub fn has_permission(&self, player: Entity, permission: &str) -> bool {
        let Ok(player_perms) = self.permissions_query.get(player) else {
            return false;
        };

        if let Some(override_value) = player_perms.get_permission_override(permission) {
            return override_value;
        }

        for group_name in &player_perms.groups {
            if self.groups.has_permission_in_group(group_name, permission) {
                return true;
            }
        }

        false
    }

    pub fn add_group(&mut self, player: Entity, group: impl Into<String>) -> bool {
        let group = group.into();
        
        if !self.groups.exists(&group) {
            return false;
        }

        let Ok(mut player_perms) = self.permissions_query.get_mut(player) else {
            return false;
        };

        player_perms.add_group(group.clone());
        self.group_events.write(GroupAddedEvent {
            player,
            group,
        });
        true
    }

    pub fn remove_group(&mut self, player: Entity, group: &str) -> bool {
        let Ok(mut player_perms) = self.permissions_query.get_mut(player) else {
            return false;
        };

        player_perms.remove_group(group)
    }

    pub fn set_permission(&mut self, player: Entity, permission: impl Into<String>, value: bool) -> bool {
        let permission = permission.into();
        let Ok(mut player_perms) = self.permissions_query.get_mut(player) else {
            return false;
        };

        player_perms.set_permission(permission.clone(), value);
        self.events.write(PermissionGrantedEvent {
            player,
            permission,
            value,
        });
        true
    }

    pub fn unset_permission(&mut self, player: Entity, permission: &str) -> bool {
        let Ok(mut player_perms) = self.permissions_query.get_mut(player) else {
            return false;
        };

        player_perms.unset_permission(permission)
    }

    pub fn get_player_groups(&self, player: Entity) -> Option<Vec<String>> {
        self.permissions_query
            .get(player)
            .ok()
            .map(|perms| perms.groups.iter().cloned().collect())
    }

    pub fn list_groups(&self) -> Vec<String> {
        self.groups.0.keys().cloned().collect()
    }
}

#[derive(Resource, Default, Clone)]
pub struct PermissionGroups(pub HashMap<String, PermissionGroup>);

impl PermissionGroups {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_group(&mut self, group: PermissionGroup) {
        self.0.insert(group.name.clone(), group);
    }

    pub fn get_group(&self, name: &str) -> Option<&PermissionGroup> {
        self.0.get(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn has_permission_in_group(&self, group_name: &str, permission: &str) -> bool {
        let Some(group) = self.get_group(group_name) else {
            return false;
        };

        if let Some(&value) = group.permissions.get(permission) {
            return value;
        }

        if permission.ends_with('*') {
            let prefix = &permission[..permission.len() - 1];
            for (perm, &value) in &group.permissions {
                if perm.starts_with(prefix) && value {
                    return true;
                }
            }
        } else {
            let wildcard = format!("{}.*", permission.rsplitn(2, '.').nth(1).unwrap_or(""));
            if let Some(&value) = group.permissions.get(&wildcard) {
                if value {
                    return true;
                }
            }
        }

        for parent_name in &group.inherits {
            if self.has_permission_in_group(parent_name, permission) {
                return true;
            }
        }

        false
    }
}
