pub mod grid;
pub mod vector;
pub use grid::Chunk3;
pub use vector::Vector3;

use bson::Document;
use serde::{Deserialize, Serialize};

/// 序列化后的实体
#[derive(Clone, Serialize, Deserialize)]
pub struct EntityCode(pub Document);

/// 识别符ID
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ID(u64);
