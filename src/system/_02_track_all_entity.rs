use sled::Db;

use crate::basic::Value;

use super::{Ignite, MergeFn, Rolling, TickFn, Systems};

pub(crate) static NAME: &'static str = "nothing";

pub(crate) static IGNITE: &'static (dyn Ignite + Send + Sync) = &|_world: &mut Db| ();
pub(crate) static ROLLING: &'static (dyn Rolling + Send + Sync) = &|_systems: &Systems| {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SocketType::SUB).unwrap();
    socket.bind("inproc://entity.create").unwrap();
    let mut buffer = [0; 1024];
    loop {
        if let Ok(length) = socket.recv_into(&mut buffer, zmq::DONTWAIT) {
            let (msg, _): (Value, usize) =
                bincode::decode_from_slice(&buffer[0..length], bincode_sled::DEFAULT_CONF)
                    .expect("反序列化失败");
            match msg {
                Value::EntityCreate(eid) => {
                    println!("new entity {:?} created. ", eid);
                }
                Value::EntityCreateOnProp((eid, prop)) => {
                    println!("new entity {:?} created on {:?}.", eid, prop);
                }
                _ => {
                    println!("unknown message: {:?} on endpoint: entity.create", msg);
                }
            };
        }
    }
};

pub(crate) static MERGE: &'static (dyn MergeFn + Send + Sync) =
    &|_eid, old: Option<Value>, _delta: Value| old;
pub(crate) static TICK: &'static (dyn TickFn + Send + Sync) = &|_eid, _old, _prop| None;

