use std::sync::atomic::AtomicBool;

use crate::world::Systems;

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

/// Act as system daemon, just like systemd.
///
/// The first thing appear in the world.
pub const INIT: Wheel = |s: &Systems, stop: &AtomicBool| {
    println!("Init");
};

pub const WHEELS: Vec<Wheel> = vec![INIT];
