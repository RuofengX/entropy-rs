use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use crate::components::NewtonComponent;

use super::{Vector3, ID};

/// 结构化Grid网格
pub struct Grid(HashMap<Chunk3, Vec<ID>>);
impl Grid {
    /// 新建一个空的Grid
    pub fn new() -> Grid {
        Grid(HashMap::new())
    }
    /// 从经典实体中构建Grid
    pub fn from_newtons(newtons: HashMap<ID, &NewtonComponent>) -> Grid {
        let mut dict: HashMap<Chunk3, Vec<ID>> = HashMap::new();
        newtons.iter().for_each(|(&id, &ent)| {
            let chunk = ent.chunk;
            // 尝试获取这个chunk
            if let Some(chunk_list) = dict.get_mut(&chunk) {
                // 如果有这个chunk，大多数情况
                (*chunk_list).push(id);
            } else {
                // 如果没有，则新建一个包含这个id的vec作为chunk
                dict.insert(chunk, vec![id]);
            }
        });
        Grid(dict)
    }
    /// Grid边长
    pub fn unit() -> f64 {
        1000f64
    }
    /// 将r整除单位长度(向下取整)
    pub fn divide_unit_floor(r: f64) -> i64 {
        (r / Grid::unit()).floor() as i64
    }
    /// 将r整除单位长度(向上取整)
    pub fn divide_unit_ceil(r: f64) -> i64 {
        (r / Grid::unit()).floor() as i64 + 1
    }
    /// 获取临近r范围内的所有Chunk，包含r所在的Chunk在内
    pub fn nearby_chunk(&self, target: Chunk3, r: f64) -> Vec<Chunk3> {
        let u = Grid::divide_unit_ceil(r); // 向外辐射多少区块
        let mut rtn: Vec<Chunk3> = Vec::new();

        for x in (target.x() - u)..(target.x() + u) {
            for y in (target.y() - u)..(target.y() + u) {
                for z in (target.z() - u)..(target.z() + u) {
                    let near_target = Chunk3([x, y, z]);
                    if self.0.contains_key(&near_target) {
                        rtn.push(near_target);
                    }
                }
            }
        }

        rtn
    }
    /// 获取chunk中的ID
    pub fn id_in_chunk(&self, chunks: Vec<Chunk3>) -> Vec<ID> {
        chunks
            .iter()
            .map(|c| self.0.get_key_value(c).unwrap().1.clone())
            .flatten()
            .collect()
    }

    /// 获取周边的IDs
    pub fn nearby_id(
        grid: &Grid,
        target: &NewtonComponent,
        radius: f64,
    ) -> Vec<ID> {
        grid.id_in_chunk(grid.nearby_chunk(target.chunk, radius))
    }
}

/// 单个网格
#[derive(Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Chunk3([i64; 3]);

/// 四则运算
impl Add<Chunk3> for Chunk3 {
    type Output = Chunk3;

    fn add(self, mut rhs: Chunk3) -> Self::Output {
        for i in 0..2 {
            rhs.0[i] += self.0[i];
        }
        rhs
    }
}
impl Sub<Chunk3> for Chunk3 {
    type Output = Chunk3;

    fn sub(self, mut rhs: Chunk3) -> Self::Output {
        for i in 0..2 {
            rhs.0[i] -= self.0[i];
        }
        rhs
    }
}

/// 类型转换
impl From<[i64; 3]> for Chunk3 {
    fn from(value: [i64; 3]) -> Self {
        Chunk3 { 0: value }
    }
}
impl Chunk3 {
    /// 零号Grid
    pub fn zero() -> Chunk3 {
        Chunk3([0i64; 3])
    }
    /// 从Vector3中转换而来
    pub fn from_vector(value: Vector3) -> Chunk3 {
        let mut rtn = [0; 3];
        rtn[0] = Grid::divide_unit_floor(value.x()); // 这里统一使用向下取整，取Grid内朝向原点的边界点
        rtn[1] = Grid::divide_unit_floor(value.y());
        rtn[2] = Grid::divide_unit_floor(value.z());
        rtn.into()
    }

    /// 三维
    pub fn x(&self) -> i64 {
        self.0[0]
    }
    pub fn y(&self) -> i64 {
        self.0[1]
    }
    pub fn z(&self) -> i64 {
        self.0[2]
    }
}
