use std::marker::PhantomData;

use retable::method::{MergeFn, TickFn};


pub struct System<V> {
    init: Box<dyn Prime>,
    wheel: Box<dyn Wheel>,
    merge: Box<dyn MergeFn>,
    tick: Box<dyn TickFn>,
    _v: PhantomData<V>,
}
