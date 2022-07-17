use std::time::Duration;

pub(crate) const LOG_NAME: &str = "rustyhack_server.log";
pub(crate) const ENTITY_UPDATE_TICK: Duration = Duration::from_millis(50);
pub(crate) const MONSTER_UPDATE_TICK: Duration = Duration::from_millis(300);
pub(crate) const LOOP_TICK: Duration = Duration::from_millis(10);
pub(crate) const MONSTER_DISTANCE_ACTIVATION: isize = 10;
pub(crate) const ASSETS_DIRECTORY: &str = "assets";
pub(crate) const MAPS_DIRECTORY: &str = "maps";
pub(crate) const MONSTERS_DIRECTORY: &str = "monsters";
pub(crate) const SPAWNS_DIRECTORY: &str = "spawns";
pub(crate) const BASE_COMBAT_ACCURACY: usize = 75;
pub(crate) const BASE_WEAPON_DAMAGE: usize = 10;
