use sled::Db;

use crate::basic::{Value, EID};

use super::{utils, Ignite, MergeFn, Prop, Rolling, Systems, TickFn};

pub(crate) static NAME: &'static str = "clock";

pub(crate) static IGNITE: &'static (dyn Ignite + Send + Sync) = &|world: &mut Db| {
    let prop = utils::get_tree(world, NAME);
    prop.insert(&EID(1), &Value::Int(0)).unwrap();
};
pub(crate) static ROLLING: &'static (dyn Rolling + Send + Sync) = &|systems: &Systems| {
    let mut count = 0;
    loop {
        count += 1;
        if count / 1000 == 0{
            println!("{:?}", systems.get(NAME).unwrap().get(&EID(1)));
        }
    }
};
pub(crate) static MERGE: &'static (dyn MergeFn + Send + Sync) =
    &|_eid: EID, old: Option<Value>, delta: Value| {
        if let Some(old) = old {
            if let (Value::Int(old), Value::Int(delta)) = (old, delta) {
                Some(Value::Int(old + delta))
            } else {
                None
            }
        } else {
            None
        }
    };
pub(crate) static TICK: &'static (dyn TickFn + Send + Sync) =
    &|_eid: EID, old: Value, _prop: &Prop| {
        if let Value::Int(_) = old {
            Some(Value::Int(1))
        } else {
            None
        }
    };
