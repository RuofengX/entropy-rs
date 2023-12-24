use std::{collections::HashMap, time::Duration};

use bincode::{Decode, Encode};

#[derive(Debug, Clone, Copy, Encode, Decode, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct EID(pub u64);

pub trait Valuable: Encode + Decode + Send + Sync + Sized {}
impl<T> Valuable for T where T: Encode + Decode + Send + Sync {}

#[derive(Debug, Clone, Encode, Decode)]
pub enum Value {
    // unit
    Nothing(()),

    // primitive
    Boolean(bool),
    Int(i64),
    UInt(u64),
    Float(f64),

    // fixed-size primitive
    Boolean2([bool; 2]),
    Int2([i64; 2]),
    UInt2([u64; 2]),
    Float2([f64; 2]),

    Boolean3([bool; 3]),
    Int3([i64; 3]),
    UInt3([u64; 3]),
    Float3([f64; 3]),

    // foreign
    Time(Duration),
    EID(EID),

    // On Stack
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Record(HashMap<EID, Value>),

    // Special
    EntityCreate(EID), // for entity create.
    EntityCreateOnProp((EID, String)), // for entity create.
    // VelocityMessage(([f64; 3], [u64; 3])), // for velocity system.
}
