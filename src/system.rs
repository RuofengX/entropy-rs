use std::marker::PhantomData;

use retable::method::{MergeFn, TickFn};

use crate::wheel::{Prime, Wheel};

pub struct System<V> {
    init: Box<dyn Prime>,
    wheel: Box<dyn Wheel>,
    merge: Box<dyn MergeFn>,
    tick: Box<dyn TickFn>,
    _v: PhantomData<V>,
}
