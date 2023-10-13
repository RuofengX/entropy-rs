use serde::{Deserialize, Serialize};

use crate::{
    components::{DetectableComponent, NewtonComponent, RadarComponent, Component},
    scaler::ID, bundle::CrashBundle,
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
    pub crash: Option<CrashBundle>,
}

impl Entity{
    pub fn tick(&mut self) -> (){
        if let Some(ref mut c) = self.newton_physics{
            c.thread_tick();
        }
        if let Some(ref mut c) = self.detectable_code{
            c.thread_tick();
        }
        if let Some(ref mut c) = self.radar{
            c.thread_tick();
        }
    }
}