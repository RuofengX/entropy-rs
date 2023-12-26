use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, OnceLock,
};

use config::Config;
use rayon::ThreadPoolBuilder;
use rustc_hash::FxHashMap;
use sensible_dbg::dbg;

use crate::{
    basic::{Value, EID},
    load_system,
    system::{
        utils, Prop, Systems, TickFn, _00_nothing, _01_clock, _02_track_all_entity, _10_benchmark,
        LOADERS,
    },
};

pub fn start(config: &Config) {
    // 0x00 Read config, build World.
    let path = config.get_string("entropy.db.path").unwrap();
    let temporary = config.get_bool("entropy.db.temporary").unwrap();
    let cache_size = config.get_int("entropy.db.cache_size").unwrap() as u64;
    let config = sled::Config::new()
        .path(path)
        .temporary(temporary)
        .mode(sled::Mode::HighThroughput)
        .use_compression(false)
        .print_profile_on_drop(true)
        .cache_capacity(cache_size);

    let mut db = config.open().unwrap();

    // soft interupt
    let interupt: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&interupt)).unwrap();

    // 0x010 Load system meta, from mod system.
    LOADERS
        .get_or_init(|| load_system![_00_nothing, _01_clock, _02_track_all_entity, _10_benchmark]);

    // 0x020 Load runtime systems and run ignite & rolling
    static RUNTIME_SYSTEM: OnceLock<Systems> = OnceLock::new();

    let mut system = FxHashMap::<&'static str, Prop>::default();
    for (&name, meta) in LOADERS.get().unwrap() {
        (meta.ignite)(&mut db);
        let prop = utils::get_prop(&db, name);
        prop.set_merge_operator(meta.merge);
        system.insert(name, prop);
    }
    RUNTIME_SYSTEM.get_or_init(|| system);

    // 0x030 Start all rolling wheels after ignite.
    let _wheels = std::thread::spawn(|| {
        dbg!("start wheels.");
        LOADERS
            .get()
            .unwrap()
            // use thread pool here
            .iter()
            .for_each(|(&name, meta)| {
                dbg!(format!("start {0} rolling", name));
                std::thread::spawn(|| {
                    (meta.rolling)(&RUNTIME_SYSTEM.get().unwrap());
                });
            })
    });

    // 0x040 Start tick loop
    // 0x041 Create thread pool.
    let executor = ThreadPoolBuilder::new()
        .num_threads(0)
        .stack_size(32 * 1024 * 1024)
        .build()
        .unwrap();

    static TICK_ALL: fn(&dyn TickFn, EID, Value, &Prop) = |tick, eid, old, prop| {
        let delta = tick(eid, old, prop);
        if let Some(delta) = delta {
            prop.merge(&eid, &delta).expect("Unregister merge method.");
        }
    };

    let systems = RUNTIME_SYSTEM.get().unwrap();
    loop {
        // interupt check
        if interupt.load(Ordering::Relaxed) {
            println!("Soft closing...");
            db.flush().unwrap();
            println!("Closed!");
            break;
        }

        // start tick all

        // create scope, to enable the full-concurrency
        executor.scope(|s| {
            // iter all props
            systems.iter().for_each(|(&name, prop)| {
                // get tick method pointer
                let tick_method = LOADERS.get().unwrap().get(name).unwrap().tick;
                // start iter entity in concurrent pool
                s.spawn(move |s| {
                    for x in prop.iter() {
                        let (eid, v) = x.unwrap();
                        // start tick process in concurrent pool
                        s.spawn(move |_s| {
                            TICK_ALL(tick_method, eid, v, prop);
                        });
                    }
                });
            });
        });
    }
}
