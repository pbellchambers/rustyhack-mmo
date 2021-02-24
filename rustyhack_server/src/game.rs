use std::collections::HashMap;
use std::thread;
use std::time::Instant;

use crossbeam_channel::{Receiver, Sender};
use laminar::{Packet, SocketEvent};
use legion::*;

use rustyhack_lib::ecs::components::*;

use crate::consts;
use crate::networking::message_handler;

mod background_map;
mod monsters;
mod player_updates;
mod players;
mod spawns;
mod systems;

pub(crate) fn run(sender: Sender<Packet>, receiver: Receiver<SocketEvent>) {
    //initialise all basic resources
    let all_maps_resource = background_map::initialise_all_maps();
    let all_maps = background_map::initialise_all_maps();
    let all_monster_definitions = monsters::initialise_all_monster_definitions();
    let all_spawns = spawns::initialise_all_spawn_definitions();
    let mut player_velocity_updates: HashMap<String, Velocity> = HashMap::new();
    let mut world = World::default();
    info!("Initialised ECS World");
    let mut player_update_schedule = systems::build_player_update_schedule();
    let mut monster_update_schedule = systems::build_monster_update_schedule();

    //initialise message handler thread
    let (channel_sender, channel_receiver) = crossbeam_channel::unbounded();
    info!("Created thread channel sender and receiver.");
    let local_sender = sender.clone();
    message_handler::spawn_message_handler_thread(sender, receiver, all_maps, channel_sender);

    //load resources into world
    let mut resources = Resources::default();
    resources.insert(all_maps_resource);
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
        player_velocity_updates = player_updates::process_player_messages(
            &mut world,
            &channel_receiver,
            &local_sender,
            player_velocity_updates,
        );

        if !player_velocity_updates.is_empty() {
            debug!("Player velocity updates available, proceeding with world update.");
            resources.insert(player_velocity_updates.to_owned());
            debug!("Added player velocity updates to world resources.");

            debug!("Executing player update schedule...");
            player_update_schedule.execute(&mut world, &mut resources);
            debug!("Player update schedule executed successfully.");

            player_velocity_updates = player_updates::send_player_updates(
                &mut world,
                &local_sender,
                player_velocity_updates,
            );
        }

        if monster_tick_time.elapsed() > consts::MONSTER_UPDATE_TICK {
            monsters::update_velocities(&mut world);
            monster_tick_time = Instant::now();

            debug!("Executing monster update schedule...");
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
        } else {
            let duration_to_sleep = consts::LOOP_TICK - loop_tick_time.elapsed();
            if duration_to_sleep.as_nanos() > 0 {
                thread::sleep(duration_to_sleep);
            }
            loop_tick_time = Instant::now();
        }
    }
}
