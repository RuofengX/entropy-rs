use std::{
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Duration,
};

use crate::world::{AbortFlag, Systems};

/// A fuction that run only once before the university start.
pub trait Prime: FnOnce(&mut Systems) + Fn(&mut Systems) + Send + Sync + 'static {}
impl<T> Prime for T where T: FnOnce(&mut Systems) -> () + Fn(&mut Systems) + Send + Sync + 'static {}

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
pub trait Wheel:
    FnMut(&Systems, AbortFlag) -> () + Fn(&Systems, AbortFlag) -> () + Send + Sync + 'static
{
}
impl<T> Wheel for T where
    T: FnMut(&Systems, AbortFlag) -> () + Fn(&Systems, AbortFlag) -> () + Send + Sync + 'static
{
}

/// Example Init function, it will called once before the world begin.
const INIT: &dyn Prime = &|_s: &mut Systems| {
    println!("Init");
};

static LOOP_TIMES: AtomicU64 = AtomicU64::new(0);
/// Example Loop function, it will called in a delicated thread after the world.
const LOOP: &dyn Wheel = &|_s: &Systems, _stop: AbortFlag| {
    let age = LOOP_TIMES.fetch_add(1, Ordering::Acquire);
    println!("Age: {} second.", age);
    thread::sleep(Duration::from_secs(1));
};

/// Act as system daemon, just like systemd.
///
/// include prime move and wheels that run forever.
pub struct Wheels {
    start: Vec<Box<dyn Prime>>,
    forever: Vec<Box<dyn Wheel>>,
}

impl Wheels {
    pub fn new() -> Self {
        Self {
            start: Vec::new(),
            forever: Vec::new(),
        }
    }
    pub fn add_prime_move(&mut self, prime: impl Prime) {
        self.start.push(Box::new(prime))
    }
    pub fn add_forever(&mut self, wheel: impl Wheel) {
        self.forever.push(Box::new(wheel))
    }

    pub fn prime_move(&mut self, s: &mut Systems) {
        self.start.iter().for_each(|x| (x)(s));
    }

    pub fn run_forever(&mut self, s: &Systems, stop: AbortFlag) {
        self.forever.iter().for_each(|x| {
            x(s, stop.clone());
        })
    }
}

impl Default for Wheels {
    fn default() -> Self {
        let mut rtn = Self::new();
        rtn.add_prime_move(INIT);
        rtn.add_forever(LOOP);
        // Add more here
        rtn
    }
}
