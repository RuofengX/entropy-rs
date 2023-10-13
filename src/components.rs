use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    entity::Entity,
    scaler::{Chunk3, EntityCode, Vector3},
};

pub trait Component: DeserializeOwned + Serialize {
    /// 对每个实体进行更新
    /// 默认行为
    /// 将自动对每个component并行迭代
    fn thread_tick(&mut self) -> ();

    /// 对所有Component顺序更新
    /// 额外的行为
    /// 每次tick对全部实体顺序运行一次
    /// 包含了无法并行处理的情形
    fn sync_tick(_entities: Vec<&mut Entity>) -> () {}
}

/// 可被侦测到的，使用bson编解码
#[derive(Serialize, Deserialize)]
pub struct DetectableComponent {}
impl DetectableComponent {
    pub fn code(&self) -> EntityCode {
        EntityCode(bson::to_document(self).unwrap())
    }
}
impl Component for DetectableComponent {
    fn thread_tick(&mut self) {}
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
    fn thread_tick(&mut self) -> () {
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
    fn thread_tick(&mut self) {
        self.since_last_update += 1;
        if self.since_last_update >= self.interval {
            todo!("添加扫描附近的代码，可能需要跨组件执行")
        }
    }
}
