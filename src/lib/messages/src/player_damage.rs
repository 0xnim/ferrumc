use bevy_ecs::prelude::{Entity, Message};

/// Source/type of damage for determining effects and death messages.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DamageSource {
    /// Fall damage from falling a distance
    Fall {
        /// Distance fallen in blocks
        distance: f32,
    },
    /// Starvation damage (hunger = 0)
    Starvation,
    /// Void damage (below world)
    Void,
    /// Attack from another entity
    EntityAttack {
        /// The attacking entity
        attacker: Entity,
    },
    /// Fire or lava damage
    Fire,
    /// Drowning damage
    Drowning,
    /// Suffocation in a block
    Suffocation,
    /// Explosion damage
    Explosion {
        /// Source entity of explosion (if any)
        source: Option<Entity>,
    },
    /// Cactus damage
    Cactus,
    /// Magic/potion damage
    Magic,
    /// Generic/unknown damage source
    #[default]
    Generic,
}

/// Fired when a player should take damage.
///
/// * Fired by: Physics (fall damage), Hunger System (starvation), Combat.
/// * Listened for by: `damage_handler` that will decrease the `Health` component.
#[derive(Message)]
pub struct PlayerDamaged {
    /// The player entity taking damage
    pub player: Entity,
    /// Amount of damage (in half-hearts, 1.0 = half heart)
    pub amount: f32,
    /// Source of the damage
    pub source: DamageSource,
}

impl PlayerDamaged {
    /// Create a new damage event with a specific source.
    pub fn new(player: Entity, amount: f32, source: DamageSource) -> Self {
        Self { player, amount, source }
    }

    /// Create a fall damage event.
    pub fn fall(player: Entity, distance: f32) -> Self {
        // Minecraft formula: damage = distance - 3 (in half-hearts)
        let amount = (distance - 3.0).max(0.0);
        Self::new(player, amount, DamageSource::Fall { distance })
    }

    /// Create a starvation damage event.
    pub fn starvation(player: Entity) -> Self {
        Self::new(player, 1.0, DamageSource::Starvation)
    }

    /// Create an entity attack damage event.
    pub fn attack(player: Entity, attacker: Entity, amount: f32) -> Self {
        Self::new(player, amount, DamageSource::EntityAttack { attacker })
    }
}

/// Fired by the `damage_handler` when a player's health reaches <= 0.
///
/// * Fired by: `damage_handler`.
/// * Listened for by: `death_handler` (respawn, death message broadcast).
#[derive(Message)]
pub struct PlayerDied {
    /// The player entity that died
    pub player: Entity,
    /// The source of the fatal damage
    pub source: DamageSource,
    /// Optional killer entity (for death messages)
    pub killer: Option<Entity>,
}
