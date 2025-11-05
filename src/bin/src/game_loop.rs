use crate::errors::BinaryError;
use crate::packet_handlers::play_packets;
use crate::register_events::register_events;
use crate::register_resources::register_resources;
use crate::systems::lan_pinger::LanPinger;
use crate::systems::register_game_systems;
use crate::systems::shutdown_systems::register_shutdown_systems;
use bevy_ecs::prelude::World;
use bevy_ecs::schedule::{ExecutorKind, Schedule};
use crossbeam_channel::Sender;
use ferrumc_commands::infrastructure::register_command_systems;
use ferrumc_config::server_config::get_global_config;
use ferrumc_net::connection::{handle_connection, NewConnection};
use ferrumc_net::server::create_server_listener;
use ferrumc_net::PacketSender;
use ferrumc_scheduler::MissedTickBehavior;
use ferrumc_scheduler::{drain_registered_schedules, Scheduler, TimedSchedule};
use ferrumc_state::{GlobalState, GlobalStateResource};
use ferrumc_utils::formatting::format_duration;
use play_packets::register_packet_handlers;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, error, info, info_span, trace, warn, Instrument};

pub fn start_game_loop(global_state: GlobalState) -> Result<(), BinaryError> {
    // ECS world and schedules
    let mut ecs_world = World::new();

    // Shutdown schedule runs once on exit
    let mut shutdown_schedule = bevy_ecs::schedule::Schedule::default();

    // Setup channels and stuff for new connections
    let sender_struct = Arc::new(ferrumc_net::create_packet_senders(&mut ecs_world));
    let (new_conn_send, new_conn_recv) = crossbeam_channel::unbounded();

    // Setup shutdown related channels
    let (shutdown_send, shutdown_recv) = tokio::sync::oneshot::channel();
    let (shutdown_response_send, shutdown_response_recv) = crossbeam_channel::unbounded();

    ferrumc_default_commands::init();

    // Register events/resources (one-time into World)
    let global_state_res = GlobalStateResource(global_state.clone());

    register_events(&mut ecs_world);
    register_resources(&mut ecs_world, new_conn_recv, global_state_res);

    // Initialize plugins BEFORE building schedules so plugin schedules get registered
    info!("Initializing plugin system...");
    let plugin_registry = crate::plugin_loader::create_plugin_registry()
        .map_err(|e| BinaryError::Custom(format!("Failed to create plugin registry: {}", e)))?;

    // Build timed scheduler (without plugin schedules yet - they're registered during plugin build)
    let mut timed = build_timed_scheduler_base();

    // Build all plugins (they will register their timed schedules)
    plugin_registry
        .build_all(
            &mut ecs_world,
            global_state.clone(),
            &mut timed.schedules[0].schedule,
        )
        .map_err(|e| BinaryError::Custom(format!("Failed to initialize plugins: {}", e)))?;

    // Now drain and register plugin schedules
    for pending in drain_registered_schedules() {
        timed.register(pending.into_timed());
    }

    // Shutdown systems
    register_shutdown_systems(&mut shutdown_schedule);

    // Start the TCP acceptor thread
    tcp_conn_acceptor(
        global_state.clone(),
        sender_struct,
        Arc::new(new_conn_send),
        shutdown_recv,
        shutdown_response_send,
    )?;

    info!(
        "Server is ready in {}",
        format_duration(global_state.start_time.elapsed())
    );

    // Run all schedules that are due, then sleep until the next one.
    const MAX_GLOBAL_CATCH_UP: usize = 64;

    while !global_state
        .shut_down
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        let mut ran_any = false;
        let mut ran_count = 0;

        loop {
            if ran_count >= MAX_GLOBAL_CATCH_UP {
                break;
            }

            let now = Instant::now();
            let Some((idx, due)) = timed.peek_next_due() else {
                std::thread::sleep(Duration::from_millis(1));
                continue;
            };

            if due > now {
                break;
            }

            let (popped_idx, _popped_due) = timed
                .pop_next_due()
                .expect("scheduler heap changed unexpectedly");
            debug_assert_eq!(popped_idx, idx);

            let name = timed.schedules[idx].name.clone();
            let period = timed.schedules[idx].period;

            let start = Instant::now();
            timed.schedules[idx].schedule.run(&mut ecs_world);
            let elapsed = start.elapsed();

            if elapsed > period {
                warn!(
                    "Schedule '{}' overran: took {:?}, budget {:?}",
                    name, elapsed, period
                );
            } else {
                trace!(
                    "Schedule '{}' ran in {:?} (budget {:?})",
                    name,
                    elapsed,
                    period
                );
            }

            timed.after_run(idx);

            ran_any = true;
            ran_count += 1;
        }

        if !ran_any {
            timed.park_until_next_due();
        }
    }

    shutdown_schedule.run(&mut ecs_world);

    // tell the TCP connection acceptor to shut down
    trace!("Sending shutdown signal to TCP connection acceptor");
    shutdown_send
        .send(())
        .expect("Failed to send shutdown signal");

    // Wait until the TCP connection acceptor has shut down
    trace!("Waiting for TCP connection acceptor to shut down");
    shutdown_response_recv
        .recv()
        .expect("Failed to receive shutdown response");

    Ok(())
}

fn build_timed_scheduler_base() -> Scheduler {
    let mut timed = Scheduler::new();

    // Tick schedule
    let build_tick = |s: &mut Schedule| {
        s.set_executor_kind(ExecutorKind::SingleThreaded);
        
        // Core I/O layer: packet → event converters
        s.add_systems(ferrumc_core_systems::animations::handle_swing_arm_packets);
        s.add_systems(ferrumc_core_systems::animations::handle_player_command_packets);
        s.add_systems(ferrumc_core_systems::animations::handle_player_input_packets);
        s.add_systems(ferrumc_core_systems::blocks::handle_place_block_packets);
        s.add_systems(ferrumc_core_systems::blocks::handle_player_action_packets);
        s.add_systems(ferrumc_core_systems::chat::handle_chat_packets);
        s.add_systems(ferrumc_core_systems::inventory::handle_set_creative_slot_packets);
        s.add_systems(ferrumc_core_systems::inventory::handle_set_held_item_packets);
        s.add_systems(ferrumc_core_systems::movement::packet_handlers::handle_set_player_position_packets);
        s.add_systems(ferrumc_core_systems::movement::packet_handlers::handle_set_player_rotation_packets);
        s.add_systems(ferrumc_core_systems::movement::packet_handlers::handle_set_player_position_and_rotation_packets);
        s.add_systems(ferrumc_core_systems::movement::packet_handlers::handle_transform_events);
        
        // Core I/O layer: event → packet broadcasters
        s.add_systems(ferrumc_core_systems::animations::broadcast_animations);
        s.add_systems(ferrumc_core_systems::animations::broadcast_pose_changes);
        s.add_systems(ferrumc_core_systems::blocks::broadcast_block_updates);
        s.add_systems(ferrumc_core_systems::blocks::send_block_change_acks);
        s.add_systems(ferrumc_core_systems::chat::broadcast_chat_messages);
        s.add_systems(ferrumc_core_systems::inventory::broadcaster::send_inventory_updates);
        s.add_systems(ferrumc_core_systems::join_leave::broadcaster::broadcast_join_messages);
        s.add_systems(ferrumc_core_systems::join_leave::broadcaster::broadcast_leave_messages);
        s.add_systems(ferrumc_core_systems::movement::broadcaster::apply_movement_requests);
        s.add_systems(ferrumc_core_systems::movement::broadcaster::broadcast_movement_updates);
        s.add_systems(ferrumc_core_systems::movement::broadcaster::broadcast_head_rotation);
        
        // Core I/O layer: block operations (chunk loading/saving)
        s.add_systems(ferrumc_core_systems::blocks::handle_place_block_requests);
        s.add_systems(ferrumc_core_systems::blocks::handle_break_block_requests);
        
        register_packet_handlers(s);
        register_command_systems(s);
        register_game_systems(s);
    };
    let tick_period = Duration::from_secs(1) / get_global_config().tps;
    timed.register(
        TimedSchedule::new("tick", tick_period, build_tick)
            .with_behavior(MissedTickBehavior::Burst)
            .with_max_catch_up(5),
    );

    // World sync
    let build_world_sync = |s: &mut Schedule| {
        s.add_systems(crate::systems::world_sync::sync_world);
    };
    timed.register(
        TimedSchedule::new("world_sync", Duration::from_secs(15), build_world_sync)
            .with_behavior(MissedTickBehavior::Skip),
    );

    // Player count refresh
    let build_player_count = |s: &mut Schedule| {
        s.add_systems(crate::systems::player_count_update::player_count_updater);
    };
    timed.register(
        TimedSchedule::new(
            "player_count_refresh",
            Duration::from_secs(10),
            build_player_count,
        )
        .with_behavior(MissedTickBehavior::Skip),
    );

    // Keepalive
    let build_keepalive = |s: &mut Schedule| {
        s.add_systems(crate::systems::keep_alive_system::keep_alive_system);
    };
    timed.register(
        TimedSchedule::new("keepalive", Duration::from_secs(1), build_keepalive)
            .with_behavior(MissedTickBehavior::Skip)
            .with_phase(Duration::from_millis(250)),
    );

    timed
}

fn tcp_conn_acceptor(
    state: GlobalState,
    packet_sender: Arc<PacketSender>,
    sender: Arc<Sender<NewConnection>>,
    mut shutdown_notify: tokio::sync::oneshot::Receiver<()>,
    shutdown_response: Sender<()>,
) -> Result<(), BinaryError> {
    let named_thread = std::thread::Builder::new().name("TokioNetworkThread".to_string());
    named_thread.spawn(move || {
        let caught_panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let async_runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .thread_name("Tokio-Async-Network")
                .build()?;
            async_runtime.spawn(async move {
                let Ok(mut pinger) = LanPinger::new().await else {
                    error!("Failed creating LAN pinger");
                    return
                };

                loop {
                    pinger.send().await;
                    sleep(Duration::from_millis(1500)).await;
                }
            });
            async_runtime.block_on({
                let state = Arc::clone(&state);
                async move {
                    let Ok(listener) = create_server_listener().await else {
                        error!("Failed to create TCP listener");
                        return Err::<(), BinaryError>(BinaryError::Custom(
                            "Failed to create TCP listener".to_string(),
                        ));
                    };
                    while !state.shut_down.load(std::sync::atomic::Ordering::Relaxed) {
                        // Wait for a new connection or shutdown signal
                        tokio::select! {
                            accept_result = listener.accept() => {
                                match accept_result {
                                    Ok((stream, _)) => {
                                        let addy = stream.peer_addr()?;
                                        debug!("Got TCP connection from {}", addy);
                                        tokio::spawn({
                                            let state = Arc::clone(&state);
                                            let packet_sender = Arc::clone(&packet_sender);
                                            let sender = Arc::clone(&sender);
                                            async move {
                                                _ = handle_connection(state, stream, packet_sender, sender)
                                                    .instrument(info_span!("conn", %addy).or_current())
                                                    .await;
                                            }
                                        });
                                    }
                                    Err(e) => {
                                        error!("Failed to accept TCP connection: {:?}", e);
                                    }
                                }
                            }
                            _ = &mut shutdown_notify => {
                                trace!("Shutdown signal received on notify channel");
                                break;
                            }
                        }
                    }
                    Ok(())
                }
            })?;
            trace!("Shutting down TCP connection acceptor");

            shutdown_response.send(()).expect("Failed to send shutdown response");
            Ok::<(), BinaryError>(())
        }));
        if let Err(e) = caught_panic {
            error!("TCP connection acceptor thread panicked: {:?}", e);
            // If we get here, the thread panicked
            state
                .shut_down
                .store(true, std::sync::atomic::Ordering::Relaxed);
            return Err::<(), BinaryError>(BinaryError::Custom(
                "TCP connection acceptor thread panicked".to_string(),
            ));
        }
        Err(BinaryError::Custom(
            "TCP connection acceptor thread panicked".to_string(),
        ))
    })?;
    Ok(())
}
