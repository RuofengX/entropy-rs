use crate::world::World;

pub mod world;
pub mod merge;
pub mod wheel;
pub mod tick;
pub mod config;

fn main() {
    println!("Hello, world!");
    let world = World::new().unwrap();
    
}
