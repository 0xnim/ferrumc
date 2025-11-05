use ferrumc_macros::{packet, NetDecode};

#[derive(NetDecode)]
#[packet(packet_id = "player_input", state = "play")]
pub struct PlayerInputPacket {
    pub flags: u8,
}

impl PlayerInputPacket {
    /// Check if the forward flag is set (bit 0x01)
    pub fn is_forward(&self) -> bool {
        self.flags & 0x01 != 0
    }

    /// Check if the backward flag is set (bit 0x02)
    pub fn is_backward(&self) -> bool {
        self.flags & 0x02 != 0
    }

    /// Check if the left flag is set (bit 0x04)
    pub fn is_left(&self) -> bool {
        self.flags & 0x04 != 0
    }

    /// Check if the right flag is set (bit 0x08)
    pub fn is_right(&self) -> bool {
        self.flags & 0x08 != 0
    }

    /// Check if the jump flag is set (bit 0x10)
    pub fn is_jumping(&self) -> bool {
        self.flags & 0x10 != 0
    }

    /// Check if the sneak flag is set (bit 0x20)
    pub fn is_sneaking(&self) -> bool {
        self.flags & 0x20 != 0
    }

    /// Check if the sprint flag is set (bit 0x40)
    pub fn is_sprinting(&self) -> bool {
        self.flags & 0x40 != 0
    }
}
