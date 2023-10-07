use serde::{Deserialize, Serialize};

use crate::{
    scaler::{Vector3, ID},
    traits::{Detectable, Entity, Newton},
};

/// 玩家实体
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: ID,
    pub pos: Vector3,
    pub velo: Vector3,
    pub mass: f64,
    pub force: Vector3,
    pub around: Vec<String>,
}

/// 玩家作为实体的特质
impl Entity for Player {
    fn id(&self) -> ID {
        self.id
    }
}

/// 玩家的物理特性
impl Newton for Player {
    fn mass(&self) -> f64 {
        self.mass
    }

    fn force(&self) -> Vector3 {
        self.force
    }

    fn pos(&self) -> Vector3 {
        self.pos
    }

    fn velo(&self) -> Vector3 {
        self.velo
    }

    fn mass_mut(&mut self) -> &mut f64 {
        &mut self.mass
    }

    fn force_mut(&mut self) -> &mut Vector3 {
        &mut self.force
    }

    fn pos_mut(&mut self) -> &mut Vector3 {
        &mut self.pos
    }

    fn velo_mut(&mut self) -> &mut Vector3 {
        &mut self.velo
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
    // TODO: 使用mongodb兼容的bson
    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}
