use crate::game::map::spawns::{AllSpawnCounts, AllSpawnsMap};
use crate::game::map::state::EntityPositionMap;
use crate::game::monsters::{movement, spawning};
use crate::game::players::PlayersPositions;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{maybe_changed, system, Entity, Query};
use rustyhack_lib::consts::{DEAD_MAP, DEFAULT_ITEM_COLOUR, DEFAULT_ITEM_ICON};
use rustyhack_lib::ecs::components::{
    Dead, DisplayDetails, Inventory, ItemDetails, MonsterDetails, Position, Stats,
};
use rustyhack_lib::ecs::item::Item;
use rustyhack_lib::ecs::monster::AllMonsterDefinitions;
use std::collections::HashMap;
use uuid::Uuid;

#[allow(clippy::trivially_copy_pass_by_ref)]
#[system(for_each)]
#[filter(maybe_changed::<Stats>())]
pub(super) fn resolve_monster_deaths(
    entity: &Entity,
    monster: &MonsterDetails,
    stats: &Stats,
    position: &Position,
    inventory: &Inventory,
    commands: &mut CommandBuffer,
    #[resource] entity_position_map: &mut EntityPositionMap,
) {
    if stats.current_hp <= 0.0 {
        debug!(
            "Monster {} {:?} {} died.",
            monster.id, entity, monster.monster_type
        );
        //drop inventory items
        let mut items_vec: Vec<(ItemDetails, DisplayDetails, Position, Item)> = vec![];
        debug!("Monster inventory was: {:?}", inventory);
        for item in &inventory.carried {
            items_vec.push((
                ItemDetails {
                    id: Uuid::new_v4(),
                    has_been_picked_up: false,
                },
                DisplayDetails {
                    icon: DEFAULT_ITEM_ICON,
                    colour: DEFAULT_ITEM_COLOUR,
                    visible: true,
                    collidable: false,
                },
                Position {
                    update_available: true,
                    pos_x: position.pos_x,
                    pos_y: position.pos_y,
                    current_map: position.current_map.clone(),
                    velocity_x: 0,
                    velocity_y: 0,
                },
                item.clone(),
            ));
        }
        //add dropped item entities to world
        debug!("Items being added to world are: {:?}", items_vec);
        commands.extend(items_vec);
        let dead_position = Position {
            current_map: position.current_map.clone() + DEAD_MAP,
            ..Dead::dead()
        };
        entity_position_map.insert(
            monster.id,
            (
                dead_position,
                DisplayDetails::dead(),
                "dead_monster".to_string(),
            ),
        );
        //remove monster from world
        commands.remove(*entity);
    }
}

#[system(par_for_each)]
pub(super) fn update_monster_velocities(
    monster: &mut MonsterDetails,
    position: &mut Position,
    #[resource] players_positions: &PlayersPositions,
) {
    debug!("Updating monster velocities - checking for movement to player positions");
    let mut moving_towards_existing_target = false;

    //get nearby players and whether monster is currently outside its spawn range
    let nearby_players = movement::get_all_players_nearby(players_positions, position);
    let outside_spawn_range =
        movement::check_if_outside_spawn_range(&monster.spawn_position, position);

    //check if current target within range and move towards it
    if let Some(target) = monster.current_target {
        if nearby_players.contains_key(&target) {
            debug!("Monster moving towards existing target.");
            movement::move_towards_target(position, nearby_players.get(&target).unwrap());
            moving_towards_existing_target = true;
        }
    }

    //else either return to spawn, pick a new target, or move randomly
    if outside_spawn_range && !moving_towards_existing_target {
        debug!("Monster returning to spawn location.");
        monster.current_target = None;
        movement::move_towards_target(position, &monster.spawn_position);
    } else if !outside_spawn_range && !moving_towards_existing_target && !nearby_players.is_empty()
    {
        debug!("Monster moving towards new target.");
        let nearest_target = movement::get_nearest_target(&nearby_players, position);
        monster.current_target = Some(nearest_target);
        movement::move_towards_target(position, nearby_players.get(&nearest_target).unwrap());
    } else if !outside_spawn_range && !moving_towards_existing_target && nearby_players.is_empty() {
        debug!("Monster moving randomly.");
        movement::move_randomly(position);
    }
}

#[system]
pub(super) fn spawn_monsters(
    world: &mut SubWorld,
    query: &mut Query<(&MonsterDetails, &Position)>,
    commands: &mut CommandBuffer,
    #[resource] all_spawns_map: &AllSpawnsMap,
    #[resource] default_spawn_counts: &AllSpawnCounts,
    #[resource] all_monster_definitions: &AllMonsterDefinitions,
) {
    debug!("Checking whether replacement monsters need to spawn.");
    let mut current_monsters_count: AllSpawnCounts = HashMap::new();
    for (monster, position) in query.iter(world) {
        current_monsters_count =
            spawning::count_alive_monsters(current_monsters_count, monster, position);
    }
    let monsters_needing_respawn: AllSpawnCounts =
        spawning::count_monsters_needing_respawn(&current_monsters_count, default_spawn_counts);
    spawning::respawn_monsters(
        &monsters_needing_respawn,
        all_monster_definitions,
        all_spawns_map,
        commands,
    );
}
