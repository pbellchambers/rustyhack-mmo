use crate::game::players;
use bincode::serialize;
use crossbeam_channel::Sender;
use laminar::Packet;
use legion::{IntoQuery, World};
use rustyhack_lib::ecs::components::{DisplayDetails, Inventory, PlayerDetails, Position, Stats};
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::network::packets::ServerMessage;
use std::process;
use uuid::Uuid;

pub(crate) fn join_player(
    world: &mut World,
    name: &str,
    client_addr: String,
    sender: &Sender<Packet>,
) {
    let mut query = <(
        &mut PlayerDetails,
        &mut DisplayDetails,
        &Position,
        &Stats,
        &Inventory,
    )>::query();
    let mut should_create_new_player = true;
    for (player_details, display_details, position, stats, inventory) in query.iter_mut(world) {
        if player_details.player_name == name && !player_details.currently_online {
            player_details.currently_online = true;
            player_details.client_addr = client_addr.clone();
            display_details.collidable = true;
            display_details.visible = true;
            info!(
                "Existing player \"{}\" logged in from: {}",
                name, &client_addr
            );
            let player = Player {
                player_details: player_details.clone(),
                display_details: *display_details,
                position: position.clone(),
                stats: *stats,
                inventory: inventory.clone(),
            };
            players::send_player_joined_response(&player, sender);
            should_create_new_player = false;
            break;
        } else if player_details.player_name == name && player_details.currently_online {
            warn!("Player join request from {} for existing player that's currently online ({} at {}).", &client_addr, &name, &player_details.client_addr);
            let response = serialize(&ServerMessage::PlayerAlreadyOnline).unwrap_or_else(|err| {
                error!(
                    "Failed to serialize player already online response, error: {}",
                    err
                );
                process::exit(1);
            });
            rustyhack_lib::network::send_packet(
                Packet::reliable_ordered(client_addr.parse().unwrap(), response, Some(14)),
                sender,
            );
            should_create_new_player = false;
            break;
        }
    }
    if should_create_new_player {
        create_player(world, name, client_addr, sender);
    }
}

fn create_player(world: &mut World, name: &str, client_addr: String, sender: &Sender<Packet>) {
    let player = Player {
        player_details: PlayerDetails {
            id: Uuid::new_v4(),
            player_name: name
                .parse()
                .expect("Something went wrong parsing player name."),
            client_addr,
            currently_online: true,
        },
        ..Default::default()
    };

    let player_entity = world.push((
        player.player_details.clone(),
        player.display_details,
        player.position.clone(),
        player.stats,
        player.inventory.clone(),
    ));
    info!("New player \"{}\" created: {:?}", name, player_entity);
    players::send_player_joined_response(&player, sender);
}
