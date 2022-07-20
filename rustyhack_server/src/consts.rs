use std::ops::Range;
use std::time::Duration;

pub(crate) const LOG_NAME: &str = "rustyhack_server.log";
pub(crate) const ENTITY_UPDATE_BROADCAST_TICK: Duration = Duration::from_millis(50);
pub(crate) const SERVER_GAME_TICK: Duration = Duration::from_millis(1000);
pub(crate) const LOOP_TICK: Duration = Duration::from_millis(10);
pub(crate) const MONSTER_DISTANCE_ACTIVATION: i32 = 10;
pub(crate) const ASSETS_DIRECTORY: &str = "assets";
pub(crate) const MAPS_DIRECTORY: &str = "maps";
pub(crate) const MONSTERS_DIRECTORY: &str = "monsters";
pub(crate) const SPAWNS_DIRECTORY: &str = "spawns";
pub(crate) const BASE_COMBAT_ACCURACY: f32 = 75.0;
pub(crate) const BASE_WEAPON_DAMAGE: Range<f32> = 5.0..10.0;
pub(crate) const TICK_SPAWN_CHANCE_PERCENTAGE: u32 = 10;
