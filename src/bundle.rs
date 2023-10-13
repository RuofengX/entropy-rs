use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{components::NewtonComponent, entity::{Entity, self}, scaler::{grid::Grid, Vector3}, system::World};

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
    fn sync_tick(s: &World) -> Result<(), ()> {
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
    fn new() -> CrashBundle {
        CrashBundle {
            react_distance: 0.1f64,
        }
    }
    /// 根据距离修改力量
    fn force_by_distance(distance: f64, force: &mut Vector3){ 
        let x: f64 = 0.1;
        todo!("TODO: 根据粒子间的距离算出对应算子x")
        *force = *force * x;
    }

    fn sync_tick(ent: &mut Entity, w: &World, g: &Grid) -> Result<(), ()> {
        if let (Some(crash), Some(newton)) = (&ent.crash, &ent.newton_physics) {
            let nearby_id = Grid::nearby_id(g, newton, crash.react_distance);
            nearby_id.into_iter().filter(
                // 返回在碰撞范围内的实体
                |id| -> bool{
                    let other_p = w.entities.get(id).ok_or(())?.newton_physics.ok_or(())?.pos; // 如果不存在目标物体则直接跑飞TODO: 添加错误类型，上级根据类型选择性删除该Bundle
                    if (other_p - newton.pos).get_length() < crash.react_distance{
                        true
                    } else{
                        false
                    }

                }
            ).for_each(
                // 对每个在范围内的实体进行操作
                |id|{
                    let (_, mut target) = w.entities.get_key_value(&id).ok_or(())?;
                    target.newton_physics.ok_or(())?.force += SOME_FORCE todo!()
                }
            )
            
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

    fn sync_tick(s: &World) -> Result<(), ()> {
        Ok(())
    }

    fn valid(_entity: &Entity) -> bool {
        todo!()
    }
}
