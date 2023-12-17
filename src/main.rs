#![allow(unused)]
use crate::world::World;

pub mod config;
pub mod merge;
pub mod tick;
pub mod wheel;
pub mod world;

fn main() {
    println!("Hello, world!");
    let world = World::new().unwrap();
}
