use crate::scaler::Vector3;

use super::traits::Newton;

fn newton_tick(mut ent: impl Newton) -> impl Newton {
    // 位置tick
    let pos = ent.pos();
    let velo = ent.velo();
    let new_pos = pos + velo;
    *ent.pos_mut() = new_pos;

    // 加速度tick
    let force = ent.force();
    let mass = ent.mass();
    let delta_velo = force / mass;
    *ent.velo_mut() = delta_velo + velo;
    *ent.force_mut() = Vector3::zero();

    ent
}
