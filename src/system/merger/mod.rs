use crate::basic::{Value, EID};

pub fn int_add(_eid: EID, old: Option<Value>, delta: Value) -> Option<Value> {
    if let Some(old) = old {
        match (old, delta) {
            (Value::Int(old), Value::Int(delta)) => Some(Value::Int(old + delta)),
            (Value::UInt(old), Value::UInt(delta)) => Some(Value::UInt(old + delta)),
            _ => None,
        }
    } else {
        None
    }
}
