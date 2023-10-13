use std::{borrow::BorrowMut, collections::HashMap, thread::AccessError};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    components::NewtonComponent,
    entity::{self, Entity},
    scaler::{grid::Grid, Vector3, ID},
    system::World,
};

/// Bundle虚拟组件
/// 一个虚组件对应了一组实体组件
/// 以及一个独占的实体tick方法
pub trait Bundle: Serialize + DeserializeOwned {
    /// 对每个实体进行更新
    /// 默认行为
    /// 将自动对每个component并行迭代
    fn thread_tick(&mut self) -> Result<(), ()>;

    /// 对所有Bundle顺序更新
    /// 额外的行为
    /// 每次tick对全部实体顺序运行一次
    /// 包含了无法并行处理的情形
    fn sync_tick(s: &mut World) -> Result<(), ()> {
        Ok(())
    }

    /// 检查Entity是否满足Bundle,
    /// 自动除错机制，如果调用失败，则由上层将该Bundle设置为None
    fn valid(_entity: &Entity) -> bool;
}

/// 碰撞体积组件
#[derive(Serialize, Deserialize)]
pub struct CrashBundle {
    /// 作用范围
    react_distance: f64,
}
impl CrashBundle {
    pub fn new() -> CrashBundle {
        CrashBundle {
            react_distance: 0.1f64,
        }
    }

    pub fn tick_for_each(
        ent: &mut Entity,
        w: &mut World,
        g: &Grid,
    ) -> Result<HashMap<ID, Vector3>, ()> {
        if let (Some(self_crash), Some(self_newton)) = (&ent.crash, &ent.newton_physics) {
            let nearby_id = Grid::nearby_id(g, self_newton, self_crash.react_distance);
            nearby_id.into_iter().filter_map(
                // 对每个在范围内的实体进行操作
                |id| -> Option<(ID, Vector3)>{
                    let other_newton = w
                        .entities
                        .get_mut(&id)
                        .unwrap()
                        .newton_physics
                        .as_mut()
                        .unwrap();
                    // 如果不存在目标物体则直接跑飞TODO: 添加错误类型，上级根据类型选择性删除该Bundle

                    let distance = other_newton.pos - self_newton.pos; // 距离向量，从自身指向目标
                    let delta = (distance.get_length() - self_crash.react_distance)
                        / self_crash.react_distance; // 距离和影响距离的倍率
                    if 0.0 < delta {
                        // 距离大于影响距离，不影响
                        return None;
                    }
                    if -0.1 <= delta && delta < 0.0 {
                        // 距离小于影响距离，但仍然大于0.1倍影响距离，体现为随delta减小而减小的引力
                        todo!()
                        return Some((id, FORCE))
                    }
                    if -1.0 <= delta && delta < -0.1 {
                        // 距离小于影响距离的0.1，体现为逐渐增至无穷大的斥力
                        todo!()
                        return Some((id, FORCE))
                    }
                },
            );

            return Ok(());
        }
        return Err(());
    }
}

impl Bundle for CrashBundle {
    fn thread_tick(&mut self) -> Result<(), ()> {
        // 碰撞体积检查只能同步(自定义异步)检查
        Ok(())
    }

    fn sync_tick(w: &mut World) -> Result<(), ()> {
        let mut modify_list: HashMap<ID, Vector3> = HashMap::new(); // 修改列表
        for (_, ent) in w.entities.iter_mut() {
            CrashBundle::tick_for_each(ent, w, &w.grid);
        }

        Ok(())
    }

    fn valid(_entity: &Entity) -> bool {
        todo!()
    }
}
