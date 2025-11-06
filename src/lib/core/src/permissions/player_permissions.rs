use bevy_ecs::prelude::Component;
use std::collections::{HashMap, HashSet};
use typename::TypeName;

#[derive(TypeName, Debug, Component, Clone)]
pub struct PlayerPermissions {
    pub groups: HashSet<String>,
    pub permissions: HashMap<String, bool>,
}

impl PlayerPermissions {
    pub fn new() -> Self {
        Self {
            groups: HashSet::new(),
            permissions: HashMap::new(),
        }
    }

    pub fn with_group(group: impl Into<String>) -> Self {
        let mut perms = Self::new();
        perms.groups.insert(group.into());
        perms
    }

    pub fn add_group(&mut self, group: impl Into<String>) {
        self.groups.insert(group.into());
    }

    pub fn remove_group(&mut self, group: &str) -> bool {
        self.groups.remove(group)
    }

    pub fn has_group(&self, group: &str) -> bool {
        self.groups.contains(group)
    }

    pub fn set_permission(&mut self, permission: impl Into<String>, value: bool) {
        self.permissions.insert(permission.into(), value);
    }

    pub fn unset_permission(&mut self, permission: &str) -> bool {
        self.permissions.remove(permission).is_some()
    }

    pub fn get_permission_override(&self, permission: &str) -> Option<bool> {
        self.permissions.get(permission).copied()
    }
}

impl Default for PlayerPermissions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(TypeName, Debug, Clone)]
pub struct PermissionGroup {
    pub name: String,
    pub permissions: HashMap<String, bool>,
    pub inherits: Vec<String>,
}

impl PermissionGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            permissions: HashMap::new(),
            inherits: Vec::new(),
        }
    }

    pub fn with_permission(mut self, permission: impl Into<String>, value: bool) -> Self {
        self.permissions.insert(permission.into(), value);
        self
    }

    pub fn with_inherit(mut self, parent: impl Into<String>) -> Self {
        self.inherits.push(parent.into());
        self
    }
}
