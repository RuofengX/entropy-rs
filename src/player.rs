use serde::{Deserialize, Serialize};

use crate::{
    scaler::{Vector3, ID},
    traits::{Detectable, Entity},
};

/// 玩家实体
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: ID,
    pub pos: Vector3,
    pub velo: Vector3,
    pub mass: f64,
    pub force: f64,
    pub around: Vec<String>,
}

/// 玩家作为实体的特质
impl Entity for Player {
    fn id(&self) -> ID {
        self.id
    }
}

/// 玩家序列化与反序列化相关
impl Detectable for Player {}
impl Into<String> for Player {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
impl TryFrom<String> for Player {
    type Error = serde_json::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}
