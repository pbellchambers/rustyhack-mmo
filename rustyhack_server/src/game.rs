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

    info!("Starting game loop");
    loop {
        //process player updates as soon as they are received
        if player_updates::process_player_messages(&mut world, &channel_receiver, &local_sender) {
            debug!("Executing player update schedule...");
            map_state_update_schedule.execute(&mut world, &mut resources);
            player_update_schedule.execute(&mut world, &mut resources);
            debug!("Player update schedule executed successfully.");

            //send game tick updates to players
            send_network_messages_schedule.execute(&mut world, &mut resources);
        }

        //all other updates that depend on the server game tick
        if server_game_tick_time.elapsed() >= consts::SERVER_GAME_TICK {
            server_game_tick_count += 1;
            debug!("Executing server tick schedule...");
            map_state_update_schedule.execute(&mut world, &mut resources);
            server_tick_update_schedule.execute(&mut world, &mut resources);
            debug!("Server tick update schedule executed successfully.");

            //things that happen every 2 ticks rather than every tick
            if server_game_tick_count == 2 {
                health_regen_schedule.execute(&mut world, &mut resources);
                server_game_tick_count = 0;
            }

            //send game tick updates to players
            send_network_messages_schedule.execute(&mut world, &mut resources);

            server_game_tick_time = Instant::now();
        }

        if entity_update_broadcast_tick_time.elapsed() >= consts::ENTITY_UPDATE_BROADCAST_TICK {
            debug!("Broadcasting entity updates");
            network_broadcast_schedule.execute(&mut world, &mut resources);
            debug!("Finished broadcasting entity updates");
            entity_update_broadcast_tick_time = Instant::now();
        }

        if server_backup_tick_time.elapsed() >= consts::SERVER_BACKUP_TICK {
            world_backup::do_world_backup(&registry, &world);
            server_backup_tick_time = Instant::now();
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
