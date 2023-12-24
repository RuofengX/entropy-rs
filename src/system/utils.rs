#![allow(unused)]
use bincode_sled::Tree;
use sled::Db;

use crate::basic::{Value, EID};

use super::Prop;

pub(crate) fn get_prop(world: &Db, prop: &'static str) -> Prop {
    Tree::<EID, Value>::open(world, prop)
}

pub(crate) fn set_entity(world: &Db, prop: &'static str, eid: EID, value: Value) {
    get_prop(world, prop).insert(&eid, &value).unwrap();
}

pub(crate) fn set_entity_if_no_exists(world: &Db, prop: &'static str, eid: EID, value: Value) {
    let prop = get_prop(world, prop);
    if prop.contains_key(&eid).unwrap() {
        prop.insert(&eid, &value).unwrap();
    }
}
