use serde::{Deserialize, Serialize};

use crate::{
    components::{DetectableComponent, NewtonComponent, RadarComponent},
    scaler::ID,
};

/// 实体
#[derive(Serialize, Deserialize)]
pub struct Entity {
    // #[serde(serialize_with = "bson::serde_helpers::serialize_hex_string_as_object_id")]
    // oid: String,
    pub id: ID,
    pub newton_physics: Option<NewtonComponent>,
    pub detectable_code: Option<DetectableComponent>,
    pub radar: Option<RadarComponent>,
}
