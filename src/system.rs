use retable::{EID, Props, PropStorage, PropValue};
use rustc_hash::FxHashSet;

pub struct World<T: PropStorage>{
    pub entities: FxHashSet<EID>,
    pub data: Props<T>,
    pub system: Vec<System<T>>,
}

impl <T:PropStorage> Default for World<T>{
    fn default() -> Self {
        World{
            entities: FxHashSet::default(),
            data: Props::new(),
            system: Vec::new(),
        }
    }
}
pub struct System<T: PropStorage>{
    /// Prop层面的tick
    prop_ticker: Box<dyn FnMut(&mut T)>,
    /// Atom层面的tick
    atom_ticker: Box<dyn FnMut(&mut PropValue)>, 
}
impl <T: PropStorage>System<T>{
    pub fn tick_prop(&mut self, prop: &mut T){
        (self.prop_ticker)(prop)
    }
    pub fn tick_atom(&mut self, prop: &mut T){
        prop.tick(&mut self.atom_ticker)
    }
}
