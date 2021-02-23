use std::time::Duration;

pub(crate) const LOG_NAME: &str = "rustyhack_server.log";
pub(crate) const ENTITY_UPDATE_TICK: Duration = Duration::from_millis(50);
pub(crate) const MONSTER_UPDATE_TICK: Duration = Duration::from_millis(300);
pub(crate) const LOOP_TICK: Duration = Duration::from_millis(10);
pub(crate) const MONSTER_DISTANCE_ACTIVATION: i32 = 10;
