#![allow(unused)]
use crate::world::World;

pub mod config;
pub mod merge;
pub mod tick;
pub mod wheel;
pub mod world;
pub mod system;

fn main() {
    println!("Hello, world!");
    let world = World::new().unwrap();
}
