use config::{builder::DefaultState, Config, ConfigBuilder};

use crate::world::start;

#[macro_use]
extern crate builder_macro;

pub mod basic;
pub mod system;
pub mod world;
pub mod msg;

fn main() {
    println!("Hello, world!");
    let builder: ConfigBuilder<DefaultState> = Config::builder();
    let config = builder
        .set_default("entropy.db.path", "./db")
        .unwrap()
        .set_default("entropy.db.temporary", false)
        .unwrap()
        .set_default("entropy.db.cache_size", 1_000_000_000)
        .unwrap()
        .build()
        .unwrap();

    start(&config);
}
