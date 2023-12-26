use std::{thread::sleep, time::Duration};

use sled::Db;

use crate::basic::{Value, EID};

use super::{
    merger::int_add, utils::set_entity_if_no_exists, Ignite, MergeFn, Prop, Rolling, Systems,
    TickFn,
};

pub(crate) static NAME: &'static str = "benchmark";

pub(crate) static IGNITE: &'static (dyn Ignite + Send + Sync) = &|world: &mut Db| {
    (0..1000).into_iter().for_each(|x| {
        set_entity_if_no_exists(world, NAME, EID(x), Value::UInt(1));
    });
};
pub(crate) static ROLLING: &'static (dyn Rolling + Send + Sync) = &|systems: &Systems| {
    let prop = systems.get(NAME).unwrap();
    loop {
        let value = prop.get(&EID(1)).unwrap().unwrap();
        println!("Benchmark: {:?}", value);
        sleep(Duration::from_secs(1));
    }
};
pub(crate) static MERGE: &'static (dyn MergeFn + Send + Sync) = &int_add;

pub(crate) static TICK: &'static (dyn TickFn + Send + Sync) =
    &|_eid: EID, _old: Value, _prop: &Prop| {
        Some(Value::UInt(1))
    };
