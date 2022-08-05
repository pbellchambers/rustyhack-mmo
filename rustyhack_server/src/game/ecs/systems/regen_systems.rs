use crate::consts::{
    BASE_HEALTH_REGEN_PERCENT, HEALTH_REGEN_CON_PERCENT, HEALTH_REGEN_CON_STATIC_FACTOR,
};
use legion::system;
use rustyhack_lib::ecs::components::Stats;

#[system(par_for_each)]
pub(super) fn apply_health_regen(stats: &mut Stats) {
    //only apply health regen if out of combat
    if !stats.in_combat && stats.current_hp > 0.0 && stats.current_hp < stats.max_hp {
        debug!("Applying health to all injured but still alive entities.");
        let regen_amount = calculate_regen_amount(stats.max_hp, stats.con);
        debug!(
            "Current hp: {}/{}, regen amount is: {}, update_available is {}",
            stats.current_hp,
            stats.max_hp,
            regen_amount.round(),
            stats.update_available
        );
        stats.current_hp += regen_amount.round();
        //don't heal more than max hp
        if stats.current_hp > stats.max_hp {
            stats.current_hp = stats.max_hp;
        }
        stats.update_available = true;
    }
}

fn calculate_regen_amount(max_hp: f32, con: f32) -> f32 {
    // Current regen calculation is as follows, this is just a first pass, it may not make sense.
    // current hp
    // + (max hp * BASE_HEALTH_REGEN_PERCENT)
    // + (con * HEALTH_REGEN_CON_PERCENT)
    // + (con / HEALTH_REGEN_CON_STATIC_FACTOR)
    (max_hp * (BASE_HEALTH_REGEN_PERCENT / 100.0))
        + (con * (HEALTH_REGEN_CON_PERCENT / 100.0))
        + (con / HEALTH_REGEN_CON_STATIC_FACTOR)
}
