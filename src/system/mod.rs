pub(super) mod _00_nothing;
pub(super) mod _01_clock;
pub(super) mod _02_track_all_entity;
pub mod loader;
pub(crate) mod utils;

use bincode_sled::Tree;
use rustc_hash::FxHashMap;
use sled::Db;
use std::sync::OnceLock;

use crate::basic::{Value, EID};

/// The meta data of system, which is defined at compile time.
pub static LOADERS: OnceLock<FxHashMap<&'static str, loader::SystemMeta>> = OnceLock::new();

/// The runtime system.
pub type Systems = FxHashMap<&'static str, Prop>;

/// The persistence system data.
pub type Prop = Tree<EID, Value>;

/// Function that run before the time start.
///
/// Can get exclusive access for whole world database,
/// One for One system.
pub trait Ignite: Fn(&mut Db) -> () {}
impl<T> Ignite for T where T: Fn(&mut Db) {}

/// Function that run after the time begin.
///
/// Can get immutable access for runtime systems(which implemented inner mutable),
/// No return, One for One system.
pub trait Rolling: Fn(&Systems) {} // ->!
impl<T> Rolling for T where T: Fn(&Systems) {}

/// Function how a value merge a delta-value in a system.
pub trait MergeFn: Fn(EID, Option<Value>, Value) -> Option<Value> + 'static {}
impl<T> MergeFn for T where T: Fn(EID, Option<Value>, Value) -> Option<Value> + 'static {}

/// Function how a entity change during every tick.
pub trait TickFn: Fn(EID, Value, &Prop) -> Option<Value> {}
impl<T> TickFn for T where T: Fn(EID, Value, &Prop) -> Option<Value> {}
