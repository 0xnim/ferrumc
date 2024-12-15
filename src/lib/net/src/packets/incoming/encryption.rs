use crate::packets::IncomingPacket;
use ferrumc_macros::{packet, NetDecode};
use crate::NetResult;
use ferrumc_net_codec::net_types::var_int::VarInt;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_state::ServerState;
use tracing::trace;
use std::sync::Arc;

#[derive(NetDecode, Debug)]
#[packet(packet_id = 0x01, state = "encryption")]
pub struct EncryptionResponsePacket {
    pub shared_secret_length: VarInt, // The length of the shared secret.
    pub shared_secret: LengthPrefixedVec<u8>, // The shared secret.
    pub verify_token_length: VarInt, // The length of the verify token.
    pub verify_token: LengthPrefixedVec<u8>, // The verify token.
}

impl IncomingPacket for EncryptionResponsePacket {
    async fn handle(self, conn_id: usize, state: Arc<ServerState>) -> NetResult<()> {
        trace!("Handshake packet received: {:?}", self);


        Ok(())
    }
}

