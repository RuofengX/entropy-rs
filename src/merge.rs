use retable::{
    basic::{Delta, Value},
    method::MergeFn,
};

/// The method that add two integers.
pub const INT_ADD_MERGE: MergeFn = |_, old: Option<Value>, delta: Delta| -> Option<Value> {
    match (old, delta) {
        (Some(Value::Int(old)), Value::Int(delta)) => Some(Value::Int(old + delta)),
        _ => None,
    }
};
