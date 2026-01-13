mod backup;
pub(super) mod combat;
mod ecs;
mod map;
mod monsters;
mod player_message_handler;
mod players;

use std::collections::HashMap;
use std::thread;
use std::time::Instant;

use crossbeam_channel::{Receiver, Sender};
use ecs::queries::common_player;
use ecs::systems;
use laminar::{Packet, SocketEvent};
use legion::Resources;
use message_io::node::{NodeHandler, NodeListener};

use crate::consts;
use crate::game::combat::{CombatAttackerStats, CombatParties};
use crate::game::map::exits;
use crate::network_messages::{map_sender, packet_receiver};
use map::state::EntityPositionMap;
use map::{spawns, state, tiles};
use players::PlayersPositions;

pub(super) fn run(
    sender: &Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    tcp_handler: NodeHandler<()>,
    tcp_listener: NodeListener<()>,
) {
    //initialise all basic resources
    let all_maps = tiles::initialise_all_maps();
    let all_maps_resource = all_maps.clone();
    let all_map_states = state::initialise_all_map_states(&all_maps);
    let combat_parties: CombatParties = HashMap::new();
    let combat_attacker_stats: CombatAttackerStats = HashMap::new();
    let players_positions: PlayersPositions = HashMap::new();
    let entity_position_map: EntityPositionMap = HashMap::new();
    let all_monster_definitions = monsters::initialise_all_monster_definitions();
    let (default_spawn_counts, all_spawns_map) = spawns::initialise_all_spawn_definitions();
    let all_map_exits = exits::initialise_all_map_exit_definitions();
    let registry = backup::create_world_registry();
    let mut player_update_schedule = systems::build_player_update_schedule();
    let mut server_tick_update_schedule = systems::build_server_tick_update_schedule();
    let mut map_state_update_schedule = systems::build_map_state_update_schedule();
    let mut health_regen_schedule = systems::build_health_regen_schedule();
    let mut send_network_messages_schedule = systems::send_network_messages_schedule();
    let mut network_broadcast_schedule = systems::network_broadcast_schedule();

    //initialise message handler thread
    let (channel_sender, channel_receiver) = crossbeam_channel::unbounded();
    info!("Created thread channel sender and receiver.");
    packet_receiver::spawn_packet_receiver_thread(receiver, channel_sender);

    //initialise tcp map sender thread
    map_sender::spawn_map_sender_thread(tcp_handler, tcp_listener, all_maps);

    //load resources into world
    let mut resources = Resources::default();
    resources.insert(all_maps_resource);
    resources.insert(all_map_states);
    resources.insert(combat_parties);
    resources.insert(combat_attacker_stats);
    resources.insert(players_positions);
    resources.insert(sender.clone());
    resources.insert(all_spawns_map.clone());
    resources.insert(default_spawn_counts);
    resources.insert(all_monster_definitions.clone());
    resources.insert(entity_position_map);
    info!("Finished loading resources.");

    let (mut world, is_saved_world) =
        backup::load_world(&registry, &all_monster_definitions, &all_spawns_map);
    info!("Finished initialising ECS World.");

    if is_saved_world {
        //marking all players as logged out on initial server start
        common_player::logout_all_players(&mut world);
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
        if player_message_handler::process_player_messages(
            &mut world,
            &all_map_exits,
            &channel_receiver,
            sender,
        ) {
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
            backup::do_world_backup(&registry, &world);
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
        let duration_to_sleep = consts::LOOP_TICK
            .checked_sub(loop_tick_time_elapsed)
            .unwrap();
        if duration_to_sleep.as_nanos() > 0 {
            //sleep here for LOOP_TICK so we don't hammer the CPU unnecessarily
            thread::sleep(duration_to_sleep);
        }
        loop_tick_time = Instant::now();
    }
}
