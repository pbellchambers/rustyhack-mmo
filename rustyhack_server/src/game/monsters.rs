pub(super) mod movement;
pub(super) mod spawning;

use crate::consts;
use rustyhack_lib::ecs::monster::{AllMonsterDefinitions, Monster};
use rustyhack_lib::utils::file;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;

pub(super) fn initialise_all_monster_definitions() -> AllMonsterDefinitions {
    info!("About to initialise all monster definitions");
    let mut all_monster_definitions: AllMonsterDefinitions = HashMap::new();
    let mut file_location = file::current_exe_location();
    file_location.pop();
    file_location.push(consts::ASSETS_DIRECTORY);
    file_location.push(consts::MONSTERS_DIRECTORY);
    let paths = file::get_all_files_in_location(&file_location);
    for path in paths {
        let unwrapped_path = path.unwrap();
        let name = String::from(
            unwrapped_path
                .file_name()
                .to_str()
                .unwrap()
                .split('.')
                .next()
                .unwrap(),
        );
        let monster: Monster = get_monster_definition_from_path(&unwrapped_path.path());
        info!("Initialised monster: {:?}", &name);
        all_monster_definitions.insert(name, monster);
    }
    all_monster_definitions
}

fn get_monster_definition_from_path(path: &Path) -> Monster {
    let file = File::open(path).unwrap_or_else(|err| {
        error!(
            "Problem getting monster definition from file: {}, error: {err}",
            path.display()
        );
        process::exit(1);
    });
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap_or_else(|err| {
        error!(
            "Problem deserializing monster definition from file: {}, error: {err}",
            path.display()
        );
        process::exit(1);
    })
}
