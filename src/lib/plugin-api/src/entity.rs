//! Entity and player helpers for plugins

use bevy_ecs::prelude::*;
use ferrumc_core::transform::position::Position;
use ferrumc_net::connection::StreamWriter;
use ferrumc_net_codec::encode::NetEncode;

/// Helper trait for entity-related operations in plugins.
///
/// Provides convenient methods for querying and manipulating entities,
/// broadcasting packets, and finding entities by location.
pub trait EntityExt {
    /// Get all players within a certain range of a position.
    ///
    /// # Arguments
    ///
    /// * `pos` - Center position
    /// * `range` - Maximum distance in blocks
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let nearby = world.players_in_range(Position { x: 0.0, y: 64.0, z: 0.0 }, 100.0);
    /// for entity in nearby {
    ///     // Do something with nearby players
    /// }
    /// ```
    fn players_in_range(&self, pos: &Position, range: f64) -> Vec<Entity>;

    /// Broadcast a packet to all connected players.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let packet = SystemChatMessage {
    ///     message: "Hello everyone!".into(),
    /// };
    /// world.broadcast_packet(&packet);
    /// ```
    fn broadcast_packet<P: NetEncode + Send>(&self, packet: &P);

    /// Broadcast a packet to all players within range of a position.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// world.broadcast_packet_in_range(
    ///     &packet,
    ///     &position,
    ///     128.0  // 128 block range
    /// );
    /// ```
    fn broadcast_packet_in_range<P: NetEncode + Send>(
        &self,
        packet: &P,
        pos: &Position,
        range: f64,
    );

    /// Send a system message to all players.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// world.broadcast_message("Server will restart in 5 minutes!");
    /// ```
    fn broadcast_message(&self, message: &str);

    /// Count the number of connected players.
    fn player_count(&self) -> usize;
}

impl EntityExt for World {
    fn players_in_range(&self, pos: &Position, range: f64) -> Vec<Entity> {
        let range_squared = range * range;

        self.iter_entities()
            .filter_map(|entity_ref| {
                let player_pos = entity_ref.get::<Position>()?;
                let dx = player_pos.x - pos.x;
                let dy = player_pos.y - pos.y;
                let dz = player_pos.z - pos.z;
                let distance_squared = dx * dx + dy * dy + dz * dz;

                if distance_squared <= range_squared {
                    Some(entity_ref.id())
                } else {
                    None
                }
            })
            .collect()
    }

    fn broadcast_packet<P: NetEncode + Send>(&self, packet: &P) {
        for entity_ref in self.iter_entities() {
            if let Some(writer) = entity_ref.get::<StreamWriter>() {
                let _ =
                    writer
                        .send_packet_ref(packet)
                        .map_err(|e: ferrumc_net::errors::NetError| {
                            tracing::error!("Failed to broadcast packet: {:?}", e);
                        });
            }
        }
    }

    fn broadcast_packet_in_range<P: NetEncode + Send>(
        &self,
        packet: &P,
        pos: &Position,
        range: f64,
    ) {
        let nearby_players = self.players_in_range(pos, range);

        for entity in nearby_players {
            if let Ok(entity_ref) = self.get_entity(entity) {
                if let Some(writer) = entity_ref.get::<StreamWriter>() {
                    let _ = writer.send_packet_ref(packet).map_err(
                        |e: ferrumc_net::errors::NetError| {
                            tracing::error!("Failed to send packet to player: {:?}", e);
                        },
                    );
                }
            }
        }
    }

    fn broadcast_message(&self, _message: &str) {
        // TODO: Implement once SystemChatMessage packet is available in ferrumc-net
        tracing::warn!("broadcast_message not yet fully implemented");
    }

    fn player_count(&self) -> usize {
        self.iter_entities()
            .filter(|e| e.contains::<StreamWriter>())
            .count()
    }
}

/// Helper function to calculate distance between two positions
pub fn distance(pos1: &Position, pos2: &Position) -> f64 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Helper function to calculate squared distance (faster, no sqrt)
pub fn distance_squared(pos1: &Position, pos2: &Position) -> f64 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    dx * dx + dy * dy + dz * dz
}
