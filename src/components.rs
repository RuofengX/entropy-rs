use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::scaler::{Chunk3, EntityCode, Vector3};

trait Component: DeserializeOwned + Serialize {
    fn tick(&mut self);
}

/// 可被侦测到的，使用bson编解码
#[derive(Serialize, Deserialize)]
pub struct DetectableComponent {}
impl DetectableComponent {
    fn code(&self) -> EntityCode {
        EntityCode(bson::to_document(self).unwrap())
    }
}
impl Component for DetectableComponent {
    fn tick(&mut self) {}
}

/// 经典力学的
#[derive(Serialize, Deserialize)]
pub struct NewtonComponent {
    pub pos: Vector3,
    pub chunk: Chunk3,
    pub velo: Vector3,
    pub mass: f64,
    pub force: Vector3,
}
impl Component for NewtonComponent {
    fn tick(&mut self) -> () {
        // 位置tick
        let pos = self.pos;
        let velo = self.velo;
        let new_pos = pos + velo;
        self.pos = new_pos;

        // 加速度tick
        let force = self.force;
        let mass = self.mass;
        let delta_velo = force / mass;
        self.velo = velo + delta_velo;
        self.force = Vector3::zero();

        // 更新grid
        self.chunk = Chunk3::from_vector(self.pos);
    }
}

/// 可探测周围的
#[derive(Serialize, Deserialize)]
pub struct RadarComponent {
    pub radius: f64,
    pub interval: u16,
    pub since_last_update: u16,
    pub around: Vec<EntityCode>,
}
impl Component for RadarComponent {
    fn tick(&mut self) {
        self.since_last_update += 1;
        if self.since_last_update >= self.interval {
            todo!("添加扫描附近的代码，可能需要跨组件执行")
        }
    }
}
