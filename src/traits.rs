use crate::scaler::ID;

/// 实体
pub trait Entity {
    fn id(&self) -> ID;
}
/// 可被侦测到的，默认使用json编解码
pub trait Detectable: Into<String> + TryFrom<String> {}
