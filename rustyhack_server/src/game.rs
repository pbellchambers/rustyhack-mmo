use std::collections::HashMap;
use std::thread;
use std::time::Instant;

use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use legion::Resources;

use crate::game::combat::{CombatAttackerStats, CombatParties};
use crate::game::map_state::EntityPositionMap;
use crate::game::players::PlayersPositions;
use crate::networking::message_handler;
use crate::{consts, world_backup};

mod background_map;
mod combat;
mod map_state;
pub(crate) mod monsters;
mod player_updates;
mod players;
pub(crate) mod spawns;
mod systems;

pub(crate) fn run(sender: Sender<Packet>, receiver: Receiver<SocketEvent>) {
    //initialise all basic resources
    let all_maps = background_map::initialise_all_maps();
    let all_maps_resource = all_maps.clone();
    let all_map_states = map_state::initialise_all_map_states(&all_maps);
    let combat_parties: CombatParties = HashMap::new();
    let combat_attacker_stats: CombatAttackerStats = HashMap::new();
    let players_positions: PlayersPositions = HashMap::new();
    let entity_position_map: EntityPositionMap = HashMap::new();
    let all_monster_definitions = monsters::initialise_all_monster_definitions();
    let (default_spawn_counts, all_spawns_map) = spawns::initialise_all_spawn_definitions();
    let registry = world_backup::create_world_registry();
    let mut player_update_schedule = systems::build_player_update_schedule();
    let mut server_tick_update_schedule = systems::build_server_tick_update_schedule();
    let mut map_state_update_schedule = systems::build_map_state_update_schedule();
    let mut health_regen_schedule = systems::build_health_regen_schedule();
    let mut send_network_messages_schedule = systems::send_network_messages_schedule();
    let mut network_broadcast_schedule = systems::network_broadcast_schedule();

    //initialise message handler thread
    let (channel_sender, channel_receiver) = crossbeam_channel::unbounded();
    info!("Created thread channel sender and receiver.");
    let local_sender = sender.clone();
    message_handler::spawn_message_handler_thread(sender, receiver, all_maps, channel_sender);

    //load resources into world
    //todo are all of these needed here, can they be handled dynamically
    let mut resources = Resources::default();
    resources.insert(all_maps_resource);
    resources.insert(all_map_states);
    resources.insert(combat_parties);
    resources.insert(combat_attacker_stats);
    resources.insert(players_positions);
    resources.insert(local_sender.clone());
    resources.insert(all_spawns_map.clone());
    resources.insert(default_spawn_counts);
    resources.insert(all_monster_definitions.clone());
    resources.insert(entity_position_map);
    info!("Finished loading resources.");

    let (mut world, is_saved_world) =
        world_backup::load_world(&registry, &all_monster_definitions, &all_spawns_map);
    info!("Finished initialising ECS World.");

    if is_saved_world {
        //marking all players as logged out on initial server start
        players::logout_all_players(&mut world);
    }

    //start tick counts
    let mut entity_update_broadcast_tick_time = Instant::now();
    let mut server_game_tick_time = Instant::now();
    let mut server_backup_tick_time = Instant::now();
    let mut loop_tick_time = Instant::now();
    let mut server_game_tick_count = 0;

    //measuring performance
    let mut bench_loop_count = 1;
    let mut bench_tick_loop_count = 1;
    let mut bench_backup_regen_count = 1;
    let mut bench_broadcast_loop_count = 1;
    let mut bench_backup_loop_count = 1;
    let mut rolling_bench_map_1 = 0;
    let mut rolling_bench_player = 0;
    let mut rolling_bench_network_1 = 0;
    let mut rolling_bench_map_2 = 0;
    let mut rolling_bench_tick = 0;
    let mut rolling_bench_regen = 0;
    let mut rolling_bench_network_2 = 0;
    let mut rolling_bench_broadcast = 0;
    let mut rolling_bench_backup = 0;

    info!("Starting game loop");
    loop {
        //process player updates as soon as they are received
        if player_updates::process_player_messages(&mut world, &channel_receiver, &local_sender) {
            debug!("Executing player update schedule...");
            let instant = Instant::now();
            map_state_update_schedule.execute(&mut world, &mut resources);
            rolling_bench_map_1 =
                update_rolling_bench(rolling_bench_map_1, bench_loop_count, instant);
            let instant = Instant::now();
            player_update_schedule.execute(&mut world, &mut resources);
            rolling_bench_player =
                update_rolling_bench(rolling_bench_player, bench_loop_count, instant);
            debug!("Player update schedule executed successfully.");

            //send game tick updates to players
            let instant = Instant::now();
            send_network_messages_schedule.execute(&mut world, &mut resources);
            rolling_bench_network_1 =
                update_rolling_bench(rolling_bench_network_1, bench_loop_count, instant);
            bench_loop_count += 1;
        }

        //all other updates that depend on the server game tick
        if server_game_tick_time.elapsed() >= consts::SERVER_GAME_TICK {
            server_game_tick_count += 1;
            debug!("Executing server tick schedule...");
            let instant = Instant::now();
            map_state_update_schedule.execute(&mut world, &mut resources);
            rolling_bench_map_2 =
                update_rolling_bench(rolling_bench_map_2, bench_tick_loop_count, instant);
            let instant = Instant::now();
            server_tick_update_schedule.execute(&mut world, &mut resources);
            rolling_bench_tick =
                update_rolling_bench(rolling_bench_tick, bench_tick_loop_count, instant);
            debug!("Server tick update schedule executed successfully.");

            //things that happen every 2 ticks rather than every tick
            if server_game_tick_count == 2 {
                let instant = Instant::now();
                health_regen_schedule.execute(&mut world, &mut resources);
                rolling_bench_regen =
                    update_rolling_bench(rolling_bench_regen, bench_backup_regen_count, instant);
                server_game_tick_count = 0;
                bench_backup_regen_count += 1;
            }

            //send game tick updates to players
            let instant = Instant::now();
            send_network_messages_schedule.execute(&mut world, &mut resources);
            rolling_bench_network_2 =
                update_rolling_bench(rolling_bench_network_2, bench_tick_loop_count, instant);

            server_game_tick_time = Instant::now();
            bench_tick_loop_count += 1;
        }

        if entity_update_broadcast_tick_time.elapsed() >= consts::ENTITY_UPDATE_BROADCAST_TICK {
            debug!("Broadcasting entity updates");
            let instant = Instant::now();
            network_broadcast_schedule.execute(&mut world, &mut resources);
            rolling_bench_broadcast =
                update_rolling_bench(rolling_bench_broadcast, bench_broadcast_loop_count, instant);
            debug!("Finished broadcasting entity updates");
            entity_update_broadcast_tick_time = Instant::now();
            bench_broadcast_loop_count += 1;
        }

        if server_backup_tick_time.elapsed() >= consts::SERVER_BACKUP_TICK {
            let instant = Instant::now();
            world_backup::do_world_backup(&registry, &world);
            rolling_bench_backup =
                update_rolling_bench(rolling_bench_backup, bench_backup_loop_count, instant);
            server_backup_tick_time = Instant::now();
            bench_backup_loop_count += 1;
            info!("Average loop performance...");
            info!("{} rolling_bench_map_1", rolling_bench_map_1);
            info!("{} rolling_bench_player", rolling_bench_player);
            info!("{} rolling_bench_network_1", rolling_bench_network_1);
            info!("{} rolling_bench_map_2", rolling_bench_map_2);
            info!("{} rolling_bench_tick", rolling_bench_tick);
            info!("{} rolling_bench_regen", rolling_bench_regen);
            info!("{} rolling_bench_network_2", rolling_bench_network_2);
            info!("{} rolling_bench_broadcast", rolling_bench_broadcast);
            info!("{} rolling_bench_backup", rolling_bench_backup);
        }

        //snapshotting the duration here to prevent a possible server crash
        let loop_tick_time_elapsed = loop_tick_time.elapsed();
        if loop_tick_time_elapsed >= consts::LOOP_TICK {
            warn!(
                "Loop took longer than specified tick time, expected: {}ms, actual: {}ms",
                consts::LOOP_TICK.as_millis(),
                loop_tick_time_elapsed.as_millis()
            );
            loop_tick_time = Instant::now();
            continue;
        }
        let duration_to_sleep = consts::LOOP_TICK - loop_tick_time_elapsed;
        if duration_to_sleep.as_nanos() > 0 {
            //sleep here for LOOP_TICK so we don't hammer the CPU unnecessarily
            thread::sleep(duration_to_sleep);
        }
        loop_tick_time = Instant::now();
    }
}

fn update_rolling_bench(current_average: u128, loop_count: u128, instant: Instant) -> u128 {
    (((current_average) * (loop_count - 1)) + instant.elapsed().as_nanos()) / loop_count
}
