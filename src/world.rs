use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use rayon::prelude::*;
use rustc_hash::FxHashMap;
use sled::Db;

use crate::system::{loader::system_load, utils, Loaders, Prop, TickFn};

pub struct World {
    db: Db,                                // persistence data
    system_meta: Loaders,                  // compile system_meta in once_cell
    system: FxHashMap<&'static str, Prop>, // runtime system
    wheels: JoinHandle<()>,
    tickers: HashMap<&'static str, Box<dyn TickFn + Sync + Send>>,
}
impl World {
    pub fn start(&'static mut self) {
        // 0x00 Read all systems
        self.system_meta = system_load();
        let s = self.system_meta.get().unwrap();

        // 0x01 Load systems and run ignite & rolling
        for system_meta in s {
            (system_meta.ignite)(&mut self.db);
            let prop = utils::get_tree(&self.db, system_meta.name);
            prop.set_merge_operator(system_meta.merge);
            self.tickers
                .insert(system_meta.name, Box::new(system_meta.tick));
            self.system.insert(system_meta.name, prop);
        }

        // 0x02 Start all rolling wheels after ignite.
        self.wheels = thread::spawn(|| {
            self.system_meta
                .get()
                .unwrap()
                .into_par_iter()
                .for_each(|x| {
                    (x.rolling)(&self.system);
                })
        });

        // 0x03 Start tick loop
        loop {
            self.system.par_iter().for_each(|(&name, prop)| {
                prop.iter().par_bridge().for_each(|x| {
                    let (eid, v) = x.unwrap();
                    let delta = (self.tickers.get(name).unwrap())(eid, v, prop);
                    if let Some(delta) = delta {
                        prop.merge(&eid, &delta).unwrap();
                    } else {
                        prop.remove(&eid).unwrap();
                    }
                })
            });
        }
    }
}
