use bincode_sled::Tree;
use sled::Db;

use crate::basic::{EID, Value};

pub(super) mod _00_nothing;

pub type Prop = Tree<EID, Value>;
pub trait Ignite: Fn(&mut Db) -> () {}
impl<T> Ignite for T where T: Fn(&mut Db) {}

pub trait Rolling: Fn(&Prop) {} // ->!
impl<T> Rolling for T where T: Fn(&Prop) {}

pub trait MergeFn: Fn(EID, Option<Value>, Value) -> Option<Value> + 'static {}
impl<T> MergeFn for T where T: Fn(EID, Option<Value>, Value) -> Option<Value> + 'static {}

pub trait TickFn: Fn(EID, Value, &Prop) -> Option<Value> {}
impl<T> TickFn for T where T: Fn(EID, Value, &Prop) -> Option<Value> {}
