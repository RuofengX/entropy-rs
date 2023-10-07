use crate::scaler::{ID, Vector3};

/// 实体
pub trait Entity {
    fn id(&self) -> ID;
}
/// 可被侦测到的，默认使用json编解码
pub trait Detectable: Into<String> + TryFrom<String> {}

/// 经典力学的
pub trait Newton{
    fn mass(&self) -> f64;
    fn force(&self) -> Vector3;
    fn pos(&self) -> Vector3;
    fn velo(&self) -> Vector3;
    fn mass_mut(&mut self) -> &mut f64;
    fn force_mut(&mut self) -> &mut Vector3;
    fn pos_mut(&mut self) -> &mut Vector3;
    fn velo_mut(&mut self) -> &mut Vector3;
}