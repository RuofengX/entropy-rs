use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::ops::Add;

/// 识别符ID
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ID(u64);

/// 三维向量
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Vector3([f64; 3]);

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, mut rhs: Vector3) -> Self::Output {
        for i in 0..2 {
            rhs.0[i] += self.0[i];
        }
        rhs
    }
}
impl From<[f64; 3]> for Vector3 {
    fn from(value: [f64; 3]) -> Self {
        Vector3 { 0: value }
    }
}
impl Vector3 {
    pub fn get_unit(&self) -> Vector3 {
        let mut rtn = [0f64; 3];
        for i in 0..2 {
            rtn[i] = self.0[i] / (i as f64);
        }
        rtn.into()
    }
}

/// 序列化后的实体
pub struct EntityCode(String);
