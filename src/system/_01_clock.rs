use std::{thread::sleep, time::Duration};

use sled::Db;

use crate::basic::{Value, EID};

use super::{utils::set_entity_if_no_exists, Ignite, MergeFn, Prop, Rolling, Systems, TickFn};

pub(crate) static NAME: &'static str = "clock";

pub(crate) static IGNITE: &'static (dyn Ignite + Send + Sync) = &|world: &mut Db| {
    set_entity_if_no_exists(world, NAME, EID(1), Value::UInt(1));
};

pub(crate) static ROLLING: &'static (dyn Rolling + Send + Sync) = &|systems: &Systems| loop {
    let count_prop = systems.get(NAME).unwrap();
    {
        if let Ok(Some(Value::UInt(count))) = count_prop.get(&EID(1)) {
            println!("距离启动已过去{}个tick", count);
        }
    }
    sleep(Duration::from_secs(1))
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
