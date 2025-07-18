use ferrumc_macros::{packet, NetDecode};
use ferrumc_net_codec::net_types::var_int::VarInt;

#[derive(NetDecode)]
#[packet(packet_id = "accept_teleportation", state = "play")]
pub struct ConfirmPlayerTeleport {
    pub teleport_id: VarInt,
}
