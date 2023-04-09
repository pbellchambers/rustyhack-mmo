use std::time::Duration;

pub(crate) const LOG_NAME: &str = "rustyhack_server.log";
pub(crate) const WORLD_BACKUP_TMP_FILENAME: &str = "rustyhack_server_world_backup.tmp";
pub(crate) const WORLD_BACKUP_FILENAME: &str = "rustyhack_server_world_backup.json";
pub(crate) const ENTITY_UPDATE_BROADCAST_TICK: Duration = Duration::from_millis(100);
pub(crate) const SERVER_GAME_TICK: Duration = Duration::from_millis(2000);
pub(crate) const LOOP_TICK: Duration = Duration::from_millis(10);
pub(crate) const SERVER_BACKUP_TICK: Duration = Duration::from_secs(60);
pub(crate) const MONSTER_DISTANCE_ACTIVATION: i32 = 10;
pub(crate) const ASSETS_DIRECTORY: &str = "assets";
pub(crate) const MAPS_DIRECTORY: &str = "maps";
pub(crate) const MAP_EXITS_DIRECTORY: &str = "map_exits";
pub(crate) const MONSTERS_DIRECTORY: &str = "monsters";
pub(crate) const SPAWNS_DIRECTORY: &str = "spawns";
pub(crate) const TICK_SPAWN_CHANCE_PERCENTAGE: u32 = 5;
pub(crate) const BASE_HEALTH_REGEN_PERCENT: f32 = 0.75;
pub(crate) const HEALTH_REGEN_CON_PERCENT: f32 = 2.0;
pub(crate) const HEALTH_REGEN_CON_STATIC_FACTOR: f32 = 5.0;
pub(crate) const MONSTER_EXP_MULTIPLICATION_FACTOR: u32 = 100;
pub(crate) const EXP_LOSS_ON_DEATH_PERCENTAGE: u32 = 5;
pub(crate) const GOLD_LOSS_ON_PVP_DEATH_PERCENTAGE: u32 = 5;

/*
The base exp table is based on the following formula:

Exp required for next level = 1000 * (current level ^ 2)

This is the cumulative values for the above formula.
*/
pub(crate) const CUMULATIVE_EXP_TABLE: [u32; 100] = [
    1000, 5000, 14000, 30000, 55000, 91000, 140000, 204000, 285000, 385000, 506000, 650000, 819000,
    1015000, 1240000, 1496000, 1785000, 2109000, 2470000, 2870000, 3311000, 3795000, 4324000,
    4900000, 5525000, 6201000, 6930000, 7714000, 8555000, 9455000, 10416000, 11440000, 12529000,
    13685000, 14910000, 16206000, 17575000, 19019000, 20540000, 22140000, 23821000, 25585000,
    27434000, 29370000, 31395000, 33511000, 35720000, 38024000, 40425000, 42925000, 45526000,
    48230000, 51039000, 53955000, 56980000, 60116000, 63365000, 66729000, 70210000, 73810000,
    77531000, 81375000, 85344000, 89440000, 93665000, 98021000, 102510000, 107134000, 111895000,
    116795000, 121836000, 127020000, 132349000, 137825000, 143450000, 149226000, 155155000,
    161239000, 167480000, 173880000, 180441000, 187165000, 194054000, 201110000, 208335000,
    215731000, 223300000, 231044000, 238965000, 247065000, 255346000, 263810000, 272459000,
    281295000, 290320000, 299536000, 308945000, 318549000, 328350000, 338350000,
];

/*
The HP table is based on the following formula:

Level 1 HP = 45
+25 HP each level

Mathematically:
((Level + 1) * 25) - 5

*/
pub(crate) const BASE_HP_TABLE: [f32; 100] = [
    45.0, 70.0, 95.0, 120.0, 145.0, 170.0, 195.0, 220.0, 245.0, 270.0, 295.0, 320.0, 345.0, 370.0,
    395.0, 420.0, 445.0, 470.0, 495.0, 520.0, 545.0, 570.0, 595.0, 620.0, 645.0, 670.0, 695.0,
    720.0, 745.0, 770.0, 795.0, 820.0, 845.0, 870.0, 895.0, 920.0, 945.0, 970.0, 995.0, 1020.0,
    1045.0, 1070.0, 1095.0, 1120.0, 1145.0, 1170.0, 1195.0, 1220.0, 1245.0, 1270.0, 1295.0, 1320.0,
    1345.0, 1370.0, 1395.0, 1420.0, 1445.0, 1470.0, 1495.0, 1520.0, 1545.0, 1570.0, 1595.0, 1620.0,
    1645.0, 1670.0, 1695.0, 1720.0, 1745.0, 1770.0, 1795.0, 1820.0, 1845.0, 1870.0, 1895.0, 1920.0,
    1945.0, 1970.0, 1995.0, 2020.0, 2045.0, 2070.0, 2095.0, 2120.0, 2145.0, 2170.0, 2195.0, 2220.0,
    2245.0, 2270.0, 2295.0, 2320.0, 2345.0, 2370.0, 2395.0, 2420.0, 2445.0, 2470.0, 2495.0, 2520.0,
];
