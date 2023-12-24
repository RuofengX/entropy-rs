use crate::{
    basic::{Value, EID},
    system::Prop,
};

pub(crate) fn add_one(_eid: EID, old: Value, _prop: &Prop) -> Option<Value> {
    match old {
        Value::Int(_) => Some(Value::Int(1)),
        Value::UInt(_) => Some(Value::UInt(1)),
        _ => None,
    }
}
