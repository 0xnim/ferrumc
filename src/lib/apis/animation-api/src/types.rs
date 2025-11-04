/// Animation types that can be played on entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationType {
    /// Main hand swing (animation ID: 0)
    SwingMainArm = 0,
    /// Offhand swing (animation ID: 3)
    SwingOffhand = 3,
    /// Take damage animation (animation ID: 1)
    TakeDamage = 1,
    /// Leave bed animation (animation ID: 2)
    LeaveBed = 2,
    /// Critical effect (animation ID: 4)
    CriticalEffect = 4,
    /// Magic critical effect (animation ID: 5)
    MagicCriticalEffect = 5,
}

impl AnimationType {
    /// Get the animation ID for network packets
    pub fn id(self) -> u8 {
        self as u8
    }
}

/// Entity pose/stance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityPose {
    /// Standing normally
    Standing,
    /// Sneaking/crouching
    Sneaking,
    /// Sprinting
    Sprinting,
    /// Swimming
    Swimming,
    /// Sleeping in bed
    Sleeping,
    /// Flying with elytra
    FlyingWithElytra,
}

/// Player command actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerCommand {
    StartSneaking,
    StopSneaking,
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartJumpWithHorse,
    StopJumpWithHorse,
    OpenVehicleInventory,
    StartFlyingWithElytra,
}

/// Which hand is being used
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hand {
    /// Main hand (ID: 0)
    Main = 0,
    /// Off hand (ID: 1)
    Off = 1,
}

impl From<i32> for Hand {
    fn from(value: i32) -> Self {
        match value {
            0 => Hand::Main,
            _ => Hand::Off,
        }
    }
}
