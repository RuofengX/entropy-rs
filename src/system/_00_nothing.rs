use sled::Db;

use crate::basic::Value;

use super::{Ignite, MergeFn, Rolling, TickFn, Systems};

pub(crate) static NAME: &'static str = "nothing";

pub(crate) static IGNITE: &'static (dyn Ignite + Send + Sync) = &|_world: &mut Db| ();
pub(crate) static ROLLING: &'static (dyn Rolling + Send + Sync) = &|_systems: &Systems| {};
pub(crate) static MERGE: &'static (dyn MergeFn + Send + Sync) =
    &|_eid, old: Option<Value>, _delta: Value| old;
pub(crate) static TICK: &'static (dyn TickFn + Send + Sync) = &|_eid, _old, _prop| None;
