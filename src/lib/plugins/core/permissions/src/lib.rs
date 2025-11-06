use ferrumc_core::permissions::PermissionGroup;
use ferrumc_permissions_api::{GroupAddedEvent, GroupRemovedEvent, PermissionGrantedEvent, PermissionGroups};
use ferrumc_plugin_api::{Plugin, PluginBuildContext, PluginCapabilities};
use tracing::info;

#[derive(Default)]
pub struct PermissionsPlugin;

impl Plugin for PermissionsPlugin {
    fn name(&self) -> &'static str {
        "permissions"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn author(&self) -> &'static str {
        "FerrumC Team"
    }

    fn description(&self) -> &'static str {
        "LuckPerms-style permission system with groups and inheritance"
    }

    fn priority(&self) -> i32 {
        100
    }

    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::builder().build()
    }

    fn build(&self, mut ctx: PluginBuildContext<'_>) {
        ctx.insert_resource(create_default_groups());
        ctx.events().register::<PermissionGrantedEvent>();
        ctx.events().register::<GroupAddedEvent>();
        ctx.events().register::<GroupRemovedEvent>();
        
        info!("Permission system initialized with default Minecraft OP groups");
    }
}

fn create_default_groups() -> PermissionGroups {
    let mut groups = PermissionGroups::new();

    let level_0 = PermissionGroup::new("minecraft.op.level.0")
        .with_permission("minecraft.command.help", true)
        .with_permission("minecraft.command.me", true)
        .with_permission("minecraft.command.trigger", true)
        .with_permission("minecraft.command.list", true)
        .with_permission("minecraft.command.teammsg", true)
        .with_permission("minecraft.command.msg", true)
        .with_permission("minecraft.command.tell", true);

    let level_1 = PermissionGroup::new("minecraft.op.level.1")
        .with_inherit("minecraft.op.level.0")
        .with_permission("minecraft.bypass.spawn_protection", true);

    let level_2 = PermissionGroup::new("minecraft.op.level.2")
        .with_inherit("minecraft.op.level.1")
        .with_permission("minecraft.command.*", true)
        .with_permission("minecraft.command.say", true)
        .with_permission("minecraft.command.title", true)
        .with_permission("minecraft.command.tellraw", true)
        .with_permission("minecraft.command.gamemode", true)
        .with_permission("minecraft.command.give", true)
        .with_permission("minecraft.command.teleport", true)
        .with_permission("minecraft.command.tp", true)
        .with_permission("minecraft.command.kill", true)
        .with_permission("minecraft.command.time", true)
        .with_permission("minecraft.command.weather", true)
        .with_permission("minecraft.command.advancement", true)
        .with_permission("minecraft.command.clear", true)
        .with_permission("minecraft.command.effect", true)
        .with_permission("minecraft.command.enchant", true)
        .with_permission("minecraft.command.experience", true)
        .with_permission("minecraft.command.xp", true)
        .with_permission("minecraft.command.summon", true);

    let level_3 = PermissionGroup::new("minecraft.op.level.3")
        .with_inherit("minecraft.op.level.2")
        .with_permission("minecraft.command.ban", true)
        .with_permission("minecraft.command.ban-ip", true)
        .with_permission("minecraft.command.banlist", true)
        .with_permission("minecraft.command.deop", true)
        .with_permission("minecraft.command.kick", true)
        .with_permission("minecraft.command.op", true)
        .with_permission("minecraft.command.pardon", true)
        .with_permission("minecraft.command.pardon-ip", true)
        .with_permission("minecraft.command.whitelist", true)
        .with_permission("minecraft.command.setidletimeout", true)
        .with_permission("minecraft.command.transfer", true);

    let level_4 = PermissionGroup::new("minecraft.op.level.4")
        .with_inherit("minecraft.op.level.3")
        .with_permission("minecraft.command.stop", true)
        .with_permission("minecraft.command.save-all", true)
        .with_permission("minecraft.command.save-on", true)
        .with_permission("minecraft.command.save-off", true)
        .with_permission("minecraft.command.publish", true);

    groups.add_group(level_0);
    groups.add_group(level_1);
    groups.add_group(level_2);
    groups.add_group(level_3);
    groups.add_group(level_4);

    groups
}
