use sled::Db;

use crate::basic::Value;

use super::{Ignite, MergeFn, Rolling, Systems, TickFn};

pub(crate) static NAME: &'static str = "nothing";

pub(crate) static IGNITE: &'static (dyn Ignite + Send + Sync) = &|_world: &mut Db| ();
pub(crate) static ROLLING: &'static (dyn Rolling + Send + Sync) = &|_systems: &Systems| {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SocketType::SUB).unwrap();
    socket.bind("inproc://entity.create").unwrap();
    loop {
        if let Ok(new_entity) = socket.recv_bytes(zmq::DONTWAIT) {
            println!("new entity: {:?}", new_entity);
            // TODO: 1. create a wrapper that convert between bincode and message.
            // TODO: 2. define a message in this system, and use it to storage entity in system.
        }
    }
};
pub(crate) static MERGE: &'static (dyn MergeFn + Send + Sync) =
    &|_eid, old: Option<Value>, _delta: Value| old;
pub(crate) static TICK: &'static (dyn TickFn + Send + Sync) = &|_eid, _old, _prop| None;
