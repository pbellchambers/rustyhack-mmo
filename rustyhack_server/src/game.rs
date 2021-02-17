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
mod player_updates;
mod systems;

pub(crate) fn run(sender: Sender<Packet>, receiver: Receiver<SocketEvent>) {
    let all_maps_resource = background_map::initialise_all_maps();
    let all_maps = background_map::initialise_all_maps();

    let (channel_sender, channel_receiver) = crossbeam_channel::unbounded();
    info!("Created thread channel sender and receiver.");
    let local_sender = sender.clone();

    message_handler::spawn_message_handler_thread(sender, receiver, all_maps, channel_sender);

    let mut world = World::default();
    info!("Initialised ECS World");

    let mut schedule = systems::build_schedule();

    let mut resources = Resources::default();
    resources.insert(all_maps_resource);
    info!("Finished loading all_maps into world resources.");

    let mut player_velocity_updates: HashMap<String, Velocity> = HashMap::new();

    let mut entity_tick_time = Instant::now();
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

            debug!("Executing schedule...");
            schedule.execute(&mut world, &mut resources);
            debug!("Schedule executed successfully.");

            player_velocity_updates = player_updates::send_player_updates(
                &mut world,
                &local_sender,
                player_velocity_updates,
            );
        }

        //do every 50ms
        if entity_tick_time.elapsed() > consts::ENTITY_UPDATE_TICK {
            player_updates::send_other_entities_updates(&mut world, &local_sender);
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
