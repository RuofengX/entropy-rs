use retable::{
    basic::{Value, EID},
    method::TickFn,
    Prop,
};

/// The method that record the number of times the world has been ticked.
const META_SYSTEM: &'static dyn TickFn = &|_, old, prop: &Prop| -> Option<Value> {
    const INIT: bool = false;
    if !INIT {
        prop.set(&EID::new(1), Value::Int(0), false);
    }

    // ZMQ related
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REQ).unwrap();

    // Connect to local ipc
    let ipc_address = "ipc:///entropy-rs-server/WORLD_META";
    socket.connect(ipc_address).unwrap();

    let message = "Hello, IPC!";
    socket.send(message.as_bytes(), 0).unwrap();
    let received_message = socket.recv_string(0).unwrap().unwrap();

    println!("Received: {}", received_message);

    match old {
        Value::Int(old) => Some(Value::Int(old + 1)),
        _ => None,
    }
};
