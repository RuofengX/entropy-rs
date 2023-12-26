use sled::Db;

use crate::basic::Value;

use super::{Ignite, MergeFn, Rolling, Systems, TickFn};

pub(crate) static NAME: &'static str = "nothing";

/// The function that run before the world start to tick.
///
/// Can do value initialization here.
/// Method here can get an exclusive access to the world(whole database).
///
#[allow(unused_variables)]
pub(crate) static IGNITE: &'static (dyn Ignite + Send + Sync) = &|world: &mut Db| ();

/// The function that run after the world start to tick, in dalicate thread.
///
/// It usually contains a loop, that do the print job or
/// an API daemon that allow outside to inspect and interact with the world.
///
/// Method here can get an shared access to the Systems(Runtime Systems).
///
#[allow(unused_variables)]
pub(crate) static ROLLING: &'static (dyn Rolling + Send + Sync) = &|systems: &Systems| {};

/// The merge function that merge a Delta value into an old value.
///
/// # Params
/// * `EID`: The entity id now ticking.
/// * `Option<Value>`: The old value raw entity.
///   If none, old value is not exist, you should define the default behavior
///   when merge a delta into an Entity that not exist.
/// * `Value`: The delta value of this entity.
///
/// Notice that an income value can be a different variable type with the old value.
/// You should define the behavior when the old value and the delta value are different.
/// You could also define what hannped when the old value does not exist(in condition that the old value is None).
///
/// # Return
/// * `Value`: The new value.
///
#[allow(unused_variables)]
pub(crate) static MERGE: &'static (dyn MergeFn + Send + Sync) =
    &|eid, old: Option<Value>, delta: Value| old;

/// The function that run onto every entity in a prop, will be called one time per tick.
///
/// The world, the systems and time manager, will promise that: in every tick loop, every entity
/// in this(named by NAME) prop, would be ticked using this method exactly once.
///
/// # Params
/// * `EID`: The entity id now ticking.
/// * `Value`: The old value of this entity.
/// * `&Prop`: A reference to the prop of this entity. See more in [`crate::system::Prop`]
///
/// # Return
/// * `Option::Some<Value>`:  If some => the delta value of this entity.
/// * `Option::<Value>::None`: If none => the entity will not get changed.
///
#[allow(unused_variables)]
pub(crate) static TICK: &'static (dyn TickFn + Send + Sync) = &|eid, old, prop| None;
