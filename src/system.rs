use std::sync::Arc;

use basic::Value;
use bincode_sled::{MergeOperator, Tree};
use sled::Db;

use crate::basic::{self, Valuable, EID};

pub type Prop = Tree<EID, Value>;
pub trait MergeFn: Fn(EID, Option<Value>, Value) -> Option<Value> + 'static{}
impl<T> MergeFn for T where T: Fn(EID, Option<Value>, Value) -> Option<Value> + 'static{}

pub trait TickFn: Fn(EID, Value, &Prop) -> Option<Value> {}
impl<T> TickFn for T where T: Fn(EID, Value, &Prop) -> Option<Value> {}

pub trait System: Sync + Send {
    fn get_self_tree(&self, world: &Db) -> Tree<EID, Value> {
        Tree::<EID, Value>::open(world, self.name())
    }
    fn get_typed_tree(&self, world: &Db, name: &str) -> Tree<EID, Value> {
        Tree::<EID, Value>::open(world, name)
    }
    fn get_value(&self, world: &Db, eid: EID) -> Option<Value> {
        self.get_self_tree(world).get(&eid).unwrap()
    }
    fn insert_value(&self, world: &Db, eid: EID, value: Value) -> Option<Value> {
        self.get_self_tree(world).insert(&eid, &value).unwrap()
    }
    fn merge_delta(&self, world: &Db, eid: EID, delta: Value) -> Option<Value> {
        self.get_self_tree(world).merge(&eid, &delta).unwrap()
    }
    fn register_merge(&'static self, world: &Db) {
        self.get_self_tree(world)
            .set_merge_operator(self.get_merge_fn())
    }

    fn name(&self) -> &str;
    fn ignite(&self, world: &mut Db);
    fn rolling(&self, prop: &Prop) -> !;
    fn get_merge_fn(&self) -> &dyn MergeFn;
    fn get_tick_fn(&self) -> &dyn TickFn;
}

pub struct NothingSyetem {}
impl System for NothingSyetem {
    fn name(&self) -> &str {
        "nothing"
    }

    fn ignite(&self, world: &mut Db) {
        ()
    }

    fn rolling(&self, prop: &Prop) -> ! {
        loop {}
    }

    fn get_merge_fn(&self) -> &dyn MergeFn {
        &|_eid, old, _delta| old
    }

    fn get_tick_fn(&self) -> &dyn TickFn {
        &|_eid, old, _prop| Some(old)
    }
}

pub struct BasicSystem {}
impl System for BasicSystem {
    fn name(&self) -> &str {
        "basic_system"
    }
    fn ignite(&self, world: &mut Db) -> () {
        let a = self.get_self_tree(world);
        a.insert(&EID(1), &Value::Int(1)).unwrap();
    }

    fn rolling(&self, prop: &Prop) -> ! {
        loop {
            println!("{:?}", prop.get(&EID(1)))
        }
    }
    fn get_merge_fn(&self) -> &dyn MergeFn {
        &|_eid, old, delta:Value| {
            if let Some(old) = old {
                // Reflect value by enum
                if let (Value::Int(old), Value::Int(delta)) = (old, delta.clone()) {
                    Some(Value::Int(old + delta))
                } else {
                    // Error format in database, just use new.
                    Some(delta)
                }
            } else {
                // No old data, just use new.
                Some(delta)
            }
        }
    }

    fn get_tick_fn(&self) -> &dyn TickFn {
        &|eid, old, prop| {
            if let Value::Int(old) = old {
                Some(Value::Int(old + 1))
            } else {
                None
            }
        }
    }
}
