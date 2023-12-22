use bincode_sled::Tree;
use sled::Db;

use crate::basic::{Value, EID};

use super::Prop;

pub(super) fn get_tree(world: &Db, name: &'static str) -> Prop {
    Tree::<EID, Value>::open(world, name)
}
