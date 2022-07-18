use std::collections::HashMap;
use std::thread;
use std::time::Instant;

use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use legion::{Resources, World};

use crate::consts;
use crate::game::combat::{CombatAttackerStats, CombatParties};
use crate::game::player_updates::PlayerPositionUpdates;
use crate::networking::message_handler;

mod background_map;
mod combat;
mod map_state;
mod monsters;
mod player_updates;
mod players;
mod spawns;
mod systems;

pub(crate) fn run(sender: Sender<Packet>, receiver: Receiver<SocketEvent>) {
    //initialise all basic resources
    let all_maps = background_map::initialise_all_maps();
    let all_maps_resource = all_maps.clone();
    let all_map_states = map_state::initialise_all_map_states(&all_maps);
    let combat_parties: CombatParties = HashMap::new();
    let combat_attacker_stats: CombatAttackerStats = HashMap::new();
    let all_monster_definitions = monsters::initialise_all_monster_definitions();
    let all_spawns = spawns::initialise_all_spawn_definitions();
    let mut player_position_updates: PlayerPositionUpdates = HashMap::new();
    let mut world = World::default();
    info!("Initialised ECS World");
    let mut player_update_schedule = systems::build_player_update_schedule();
    let mut monster_update_schedule = systems::build_monster_update_schedule();
    let mut map_state_update_schedule = systems::build_map_state_update_schedule();

    //initialise message handler thread
    let (channel_sender, channel_receiver) = crossbeam_channel::unbounded();
    info!("Created thread channel sender and receiver.");
    let local_sender = sender.clone();
    message_handler::spawn_message_handler_thread(sender, receiver, all_maps, channel_sender);

    //load resources into world
    let mut resources = Resources::default();
    resources.insert(all_maps_resource);
    resources.insert(all_map_states);
    resources.insert(combat_parties);
    resources.insert(combat_attacker_stats);
    info!("Finished loading resources into world.");

    //spawn initial monsters
    monsters::spawn_initial_monsters(&mut world, &all_monster_definitions, &all_spawns);
    info!("Spawned all monsters in initial positions.");

    //start tick counts
    let mut entity_tick_time = Instant::now();
    let mut monster_tick_time = Instant::now();
    let mut loop_tick_time = Instant::now();
    info!("Starting game loop");
    loop {
        player_position_updates = player_updates::process_player_messages(
            &mut world,
            &channel_receiver,
            &local_sender,
            player_position_updates,
        );

        if !player_position_updates.is_empty() {
            debug!("Player velocity updates available, proceeding with world update.");
            resources.insert(player_position_updates.clone());
            debug!("Added player position updates to world resources.");

            debug!("Executing player update schedule...");
            map_state_update_schedule.execute(&mut world, &mut resources);
            player_update_schedule.execute(&mut world, &mut resources);
            debug!("Player update schedule executed successfully.");

            player_position_updates = player_updates::send_player_position_updates(
                &mut world,
                &local_sender,
                player_position_updates,
            );
            player_updates::send_player_stats_updates(&mut world, &local_sender);
        }

        if monster_tick_time.elapsed() > consts::MONSTER_UPDATE_TICK {
            monsters::update_velocities(&mut world);
            monster_tick_time = Instant::now();

            debug!("Executing monster update schedule...");
            map_state_update_schedule.execute(&mut world, &mut resources);
            monster_update_schedule.execute(&mut world, &mut resources);
            debug!("Monster update schedule executed successfully.");
        }

        if entity_tick_time.elapsed() > consts::ENTITY_UPDATE_TICK {
            player_updates::send_other_entities_updates(&world, &local_sender);
            entity_tick_time = Instant::now();
        }

        if loop_tick_time.elapsed() > consts::LOOP_TICK {
            warn!(
                "Loop took longer than specified tick time, expected: {:?}, actual: {:?}",
                consts::LOOP_TICK,
                loop_tick_time.elapsed()
            );
            loop_tick_time = Instant::now();
            continue;
        }
        let duration_to_sleep = consts::LOOP_TICK - loop_tick_time.elapsed();
        if duration_to_sleep.as_nanos() > 0 {
            thread::sleep(duration_to_sleep);
        }
        loop_tick_time = Instant::now();
    }
}
