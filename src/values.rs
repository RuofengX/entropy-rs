use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Valuable: Encode + Decode + Default {}

#[enum_dispatch(Value)]
pub enum Value {
    Int(i64),
    UInt(u64),
    Float(f64),

    Int2((i64, i64)),
    UInt2((u64, u64)),
    Float2((f64, f64, f64)),

    Int3((i64, i64, i64)),
    UInt3((u64, u64, u64)),
    Float3((f64, f64, f64)),
}
