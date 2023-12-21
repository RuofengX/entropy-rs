use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Valuable: Encode + Decode + Default{

}
#[enum_dispatch(Value)]
pub enum Value{
    Int(i64),
    UInt(u64),
    Float(f64),
    V2Int((i64, i64)),

}