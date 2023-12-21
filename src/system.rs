use basic::Value;
use bincode_sled::Tree;
use sled::Db;

use crate::basic::{Valuable, EID, self};

pub trait System {

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
        self.get_self_tree(world).set_merge_operator(Self::merge_fn)
    }

    fn name(&self) -> &str;
    fn ignite(&self, world: &mut Db);
    fn rolling(&self, world: &Db) -> !;
    fn merge_fn(eid: EID, old: Option<Value>, delta: Value) -> Option<Value>;
    fn tick_fn(
        prop: Tree<EID, Value>,
        eid: EID,
        old: Value,
        system: &Self,
    ) -> Option<Value>;
}

pub struct NothingSyetem {}
impl System for NothingSyetem {

    fn name(&self) -> &str {
        "nothing"
    }

    fn ignite(&self, world: &mut Db) {
        ()
    }

    fn rolling(&self, world: &Db) -> ! {
        loop {}
    }

    fn merge_fn(eid: EID, old: Option<Value>, delta: Value) -> Option<Value> {
        old
    }

    fn tick_fn(
        prop: Tree<EID, Value>,
        eid: EID,
        old: Value,
        system: &Self,
    ) -> Option<Value> {
        Some(old)
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

    fn rolling(&self, world: &Db) -> ! {
        loop {
            println!("{:?}", self.get_value(world, EID(1)))
        }
    }

    fn merge_fn(eid: EID, old: Option<Value>, delta: Value) -> Option<Value> {
        if let Some(old) = old {
            // Reflect value by enum
            if let (Value::Int(old), Value::Int(delta)) = (old, delta.clone()){
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

    fn tick_fn(
        prop: Tree<EID, Value>,
        eid: EID,
        old: Value,
        system: &Self,
    ) -> Option<Value> {
        if let Value::Int(old) = old{
            Some(Value::Int(old + 1))
        } else {
            None
        }
    }
}
