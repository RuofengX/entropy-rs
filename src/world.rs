//! Define a virtual world.
//!
//!
//! # Overture
//!
//! One man cannot build a world.
//!
//! A world could consider as 'exist' when there are at lease 2 different entities.
//! The first entity is always the empty container, in this case the first entity is the struct [`World`].
//!
//! When the first entity is created, the second entity does not exist. So it cannot be called a 'World'.
//! So if the author of this framework(RuofengX) created half of the world, another half is your(Lib User) responsibility.
//!
use std::sync::{atomic::AtomicBool, Arc};

use retable::{api::PropStorage, basic::PropTag, Database};
use rustc_hash::FxHashMap;

/// A hash map of the props of the world.
///
/// Each props is created by [`retable::api::AtomStorage::create_prop`] and stored as [`std::sync::Arc`] in this hash map.
///
pub type Systems = FxHashMap<PropTag, Arc<dyn PropStorage>>;

/// A function that runs forever when the world is running.
///
/// The wheel function is run right before the world starts to tick.
/// Nothing stop the wheel from running forever, unless the world is shutting down, in which case
///
/// # Arguments
/// - `s`: Reference to the systems.
/// - `stop`: A flag to stop the wheel. Should been checked frequently.
///
/// # Return
/// - `()`: Run forever until the stop flag is set to true.
///
/// # Usage
///
/// * Run an non-stop GraphQL API along with the world.
/// * Receieve and send messages from a zmq ipc socket.
///
pub type Wheel = fn(s: &Systems, stop: &AtomicBool) -> ();

/// World container.
///
/// # Private Field
/// - `db`: The database of the world.
/// - `systems`: A hash map of the props of the world. Each props is created by [`retable::api::AtomStorage::create_prop`] and stored as [`std::sync::Arc`] in this hash map.
/// - `wheels`: The functions that run forever when the world is running, just like daemon thread. See more in [`Wheel`]
///
pub struct World {
    db: Database,
    systems: Systems,
    wheels: Vec<Wheel>,
}
