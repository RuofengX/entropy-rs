use std::collections::HashMap;

use crate::{entity::Entity, scaler::{grid::Grid, ID}};

pub struct World{
    pub entities: HashMap<ID, Entity>,
    pub grid: Grid,
}
impl World{
    fn tick(&mut self){
        for e in self.entities.iter_mut(){
        }
    }
}
