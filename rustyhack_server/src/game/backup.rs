use crate::consts::{WORLD_BACKUP_FILENAME, WORLD_BACKUP_TMP_FILENAME};
use crate::game::map::spawns::AllSpawnsMap;
use crate::game::monsters::spawning::spawn_initial_monsters;
use crossterm::style::Color;
use legion::serialize::Canon;
use legion::{Registry, World};
use rustyhack_lib::ecs::components::{
    DisplayDetails, Inventory, ItemDetails, MonsterDetails, PlayerDetails, Position, Stats,
};
use rustyhack_lib::ecs::inventory::{Armour, Equipment, Trinket, Weapon};
use rustyhack_lib::ecs::item::Item;
use rustyhack_lib::ecs::monster::{AllMonsterDefinitions, Monster};
use rustyhack_lib::ecs::player::Player;
use rustyhack_lib::utils::file;
use serde::de::DeserializeSeed;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::ops::Range;
use std::path::Path;
use uuid::Uuid;

pub(super) fn create_world_registry() -> Registry<String> {
    let mut registry = Registry::<String>::default();
    registry.register::<DisplayDetails>("display_details".to_string());
    registry.register::<PlayerDetails>("player_details".to_string());
    registry.register::<MonsterDetails>("monster_details".to_string());
    registry.register::<ItemDetails>("item_details".to_string());
    registry.register::<Stats>("stats".to_string());
    registry.register::<Inventory>("inventory".to_string());
    registry.register::<Position>("position".to_string());
    registry.register::<Equipment>("equipment".to_string());
    registry.register::<Player>("player".to_string());
    registry.register::<Monster>("monster".to_string());
    registry.register::<Item>("item".to_string());
    registry.register::<Weapon>("weapon".to_string());
    registry.register::<Armour>("armour".to_string());
    registry.register::<Trinket>("trinket".to_string());
    registry.register::<Uuid>("uuid".to_string());
    registry.register::<Color>("color".to_string());
    registry.register::<Range<f32>>("range_f32".to_string());
    registry.register::<f32>("f32".to_string());
    registry.register::<u32>("u32".to_string());
    registry.register::<i32>("i32".to_string());
    registry.register::<bool>("bool".to_string());
    registry.register::<char>("char".to_string());

    registry
}

pub(super) fn do_world_backup(registry: &Registry<String>, world: &World) {
    info!("World backup starting...");
    let filter = legion::any();
    let entity_serializer = Canon::default();

    let mut tmp_backup_file = file::current_exe_location();
    tmp_backup_file.pop();
    tmp_backup_file.push(WORLD_BACKUP_TMP_FILENAME);

    let file = File::create(&tmp_backup_file)
        .expect("Failed to create server backup file in current directory.");
    serde_json::to_writer(
        file,
        &world.as_serializable(filter, registry, &entity_serializer),
    )
    .expect(
        "Failed to serialize world for backup, unable to proceed. Please use last good backup.",
    );

    let mut backup_file = file::current_exe_location();
    backup_file.pop();
    backup_file.push(WORLD_BACKUP_FILENAME);
    fs::rename(tmp_backup_file, backup_file)
        .expect("Failed to rename world backup tmp to world backup filename.");
    info!("World backup done.");
}

pub(super) fn load_world(
    registry: &Registry<String>,
    all_monster_definitions: &AllMonsterDefinitions,
    all_spawns_map: &AllSpawnsMap,
) -> (World, bool) {
    let mut world;
    let mut is_saved_world = false;
    if Path::new(WORLD_BACKUP_FILENAME).exists() {
        let entity_serializer = Canon::default();
        let string =
            fs::read_to_string(WORLD_BACKUP_FILENAME).expect("Unable to read world backup file.");
        let value: Value = serde_json::from_str(&string)
            .expect("World backup is not valid json, unable to proceed.");

        world = registry
            .as_deserialize(&entity_serializer)
            .deserialize(value)
            .expect("Failed to deserialize world backup, unable to proceed.");
        is_saved_world = true;
        info!("Loaded previous world backup successfully.");
    } else {
        warn!("World backup does not exist, initialising brand new world.");
        world = World::default();
        info!("Created new world successfully.");

        //spawn initial monsters
        spawn_initial_monsters(&mut world, all_monster_definitions, all_spawns_map);
        info!("Spawned all monsters in initial positions.");
    }
    (world, is_saved_world)
}
