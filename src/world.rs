use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use rayon::prelude::*;
use sled::{Config, Db};

use crate::system::{BasicSystem, NothingSyetem, System};

pub struct Systems(Vec<&'static dyn System>);

impl Systems {
    pub fn new() -> Self {
        let mut inner: Vec<&'static dyn System> = Vec::new();
        inner.push(&NothingSyetem {});
        inner.push(&BasicSystem {});
        Systems(inner)
    }
}
pub type Wheels = Vec<JoinHandle<()>>;

pub struct World {
    database: Db,
    systems: Systems,
    wheels: Wheels,
}

impl World {
    pub fn new() -> World {
        World {
            database: Config::new().temporary(true).open().unwrap(),
            systems: Systems::new(),
            wheels: vec![],
        }
    }

    pub fn init(mut self) -> Self {
        // register merge
        // ignite all systems
        for system in &self.systems.0 {
            system.register_merge(&self.database);
            system.ignite(&mut self.database);
        }

        // start rolling wheel
        for system in self.systems.0 {
            let tree = system.get_self_tree(&self.database);
            self.wheels.push(thread::spawn(move || {
                system.rolling(&tree);
            }));
        }
        todo!()
    }
}
