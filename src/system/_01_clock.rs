use std::{
    thread::sleep,
    time::{self, Duration},
};

use sled::Db;

use crate::basic::{Value, EID};

use super::{
    merger, ticker, utils::set_entity_if_no_exists, Ignite, MergeFn, Rolling, Systems, TickFn,
};

pub(crate) static NAME: &'static str = "clock";

pub(crate) static IGNITE: &'static (dyn Ignite + Send + Sync) = &|world: &mut Db| {
    set_entity_if_no_exists(world, NAME, EID(1), Value::UInt(1));
};

pub(crate) static ROLLING: &'static (dyn Rolling + Send + Sync) = &|systems: &Systems| {
    static ONE_SECOND: Duration = Duration::from_secs(1);
    let mut mspt: f32;
    let mut last_tick: u64 = 0;
    loop {
        let start_time = time::Instant::now(); // 记录开始时间

        let count_prop = systems.get(NAME).unwrap();
        let count = count_prop.get(&EID(1)).unwrap().unwrap();

        if let Value::UInt(count) = count {
            println!("距离启动已过去{}个tick", count);
            let delta = count - last_tick;
            if delta != 0 {
                mspt = (ONE_SECOND.as_millis() as f32) / ((count - last_tick) as f32);
                println!("MSPT: {}", mspt);
            } else {
                println!("MSPT: >1000");
            }
            last_tick = count;
        } else {
            panic!("count is not a uint");
        }

        let elapsed = start_time.elapsed(); // 调用结束时间

        if elapsed <= ONE_SECOND {
            // no lagging
            sleep(ONE_SECOND - elapsed);
        } else {
            // ! lagging
            println!("lagging by {}ms (over one second)", elapsed.as_millis());
        }
    }
};

pub(crate) static MERGE: &'static (dyn MergeFn + Send + Sync) = &merger::int_add;

pub(crate) static TICK: &'static (dyn TickFn + Send + Sync) = &ticker::int::add_one;
