use ferrumc_net_codec::net_types::var_int::VarInt;

/// Face of a block being interacted with
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockFace {
    Bottom = 0,
    Top = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

impl BlockFace {
    /// Convert from protocol var_int to BlockFace
    pub fn from_var_int(value: VarInt) -> Option<Self> {
        match value.0 {
            0 => Some(Self::Bottom),
            1 => Some(Self::Top),
            2 => Some(Self::North),
            3 => Some(Self::South),
            4 => Some(Self::West),
            5 => Some(Self::East),
            _ => None,
        }
    }

    /// Get the offset for placing a block on this face
    pub fn offset(&self) -> (i32, i32, i32) {
        match self {
            Self::Bottom => (0, -1, 0),
            Self::Top => (0, 1, 0),
            Self::North => (0, 0, -1),
            Self::South => (0, 0, 1),
            Self::West => (-1, 0, 0),
            Self::East => (1, 0, 0),
        }
    }
}

/// Hand used for block placement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Main = 0,
    Off = 1,
}

impl Hand {
    pub fn from_var_int(value: VarInt) -> Option<Self> {
        match value.0 {
            0 => Some(Self::Main),
            1 => Some(Self::Off),
            _ => None,
        }
    }
}
