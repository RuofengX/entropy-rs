use sled::Db;

use super::{Ignite, MergeFn, Prop, Rolling, TickFn};

pub(crate) static NAME: &'static str = "nothing";

pub(crate) static IGNITE: &'static (dyn Ignite + Send + Sync) = &|_db: &mut Db| ();
pub(crate) static ROLLING: &'static (dyn Rolling + Send + Sync) = &|_prop: &Prop| loop {};
pub(crate) static MERGE: &'static (dyn MergeFn + Send + Sync) = &|_eid, old, _delta| old;
pub(crate) static TICK: &'static (dyn TickFn + Send + Sync) = &|_eid, old, _prop| Some(old);
