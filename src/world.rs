use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, OnceLock,
};

use config::Config;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use sensible_dbg::dbg;

use crate::{
    load_system,
    system::{utils, Prop, Systems, _00_nothing, _01_clock, _02_track_all_entity, LOADERS},
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

    // 0x01 Load system meta, from mod system.
    LOADERS.get_or_init(|| load_system![_00_nothing, _01_clock, _02_track_all_entity]);

    // 0x02 Load runtime systems and run ignite & rolling
    static RUNTIME_SYSTEM: OnceLock<Systems> = OnceLock::new();

    let mut system = FxHashMap::<&'static str, Prop>::default();
    for (&name, meta) in LOADERS.get().unwrap() {
        (meta.ignite)(&mut db);
        let prop = utils::get_tree(&db, name);
        prop.set_merge_operator(meta.merge);
        system.insert(name, prop);
    }
    RUNTIME_SYSTEM.get_or_init(|| system);

    // 0x03 Start all rolling wheels after ignite.
    let _wheels = std::thread::spawn(|| {
        LOADERS
            .get()
            .unwrap()
            .into_par_iter()
            .panic_fuse()
            .for_each(|(_, meta)| {
                (meta.rolling)(&RUNTIME_SYSTEM.get().unwrap());
            })
    });

    // 0x04 Start tick loop

    loop {
        dbg!("loop start");
        dbg!("interupt check");
        if interupt.load(Ordering::Relaxed) {
            dbg!("interupt!");
            println!("Soft close.");
            db.flush().unwrap();
            break;
        }
        dbg!("interupt pass");
        RUNTIME_SYSTEM
            .get()
            .unwrap()
            .par_iter()
            .panic_fuse()
            .for_each(|(&name, prop)| {
                dbg!(name);
                // TODO: Use light thread pool here.
                prop.iter().for_each(|x| {
                    let (eid, v) = x.unwrap();
                    dbg!(eid);
                    dbg!(v.clone());
                    let delta = (LOADERS.get().unwrap().get(name).unwrap().tick)(eid, v, prop);
                    if let Some(delta) = delta {
                        dbg!("merge");
                        prop.merge(&eid, &delta).expect("Unregister merge method.");
                    } else {
                        dbg!("nothing change");
                    }
                    dbg!("op_done");
                });
            });
        dbg!("loop end");
    }
}
