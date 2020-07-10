extern crate core;

use crate::world::{BodyId, Body, BodyPair};

use crate::constraint::Constraint;

use fnv::FnvHashMap;


use std::ops::{Index, IndexMut};

#[derive(Default)]
pub struct Bodies {
    bodies: Vec<Option<Body>>,
    len: usize,
}

impl Bodies {
    pub fn new() -> Bodies {
        Bodies {
            bodies: Vec::new(),
            len: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&Body> {
        self.bodies.iter().filter_map(|body| body.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Body> {
        self.bodies.iter_mut().filter_map(|body| body.as_mut())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, id: BodyId) -> Option<&Body> {
        self.bodies.get(id).and_then(|body| body.as_ref())
    }

    pub fn get_mut(&mut self, id: BodyId) -> Option<&mut Body> {
        self.bodies.get_mut(id).and_then(|body| body.as_mut())
    }

    pub fn add(&mut self, mut body: Body) -> BodyId {
        self.len += 1;

        match self.bodies.iter().position(|entry| entry.is_none()) {
            Some(index) => {
                body.id = index;
                self.bodies[index] = Some(body);
                index
            }
            None => {
                let index = self.bodies.len();
                body.id = index;
                self.bodies.push(Some(body));
                index
            }
        }
    }

    pub fn remove(&mut self, id: BodyId) -> Option<Body> {
        self.len -= 1;
        self.bodies.remove(id)
    }
}

impl Index<BodyId> for Bodies {
    type Output = Body;

    fn index(&self, index: BodyId) -> &Self::Output {
        match self.get(index) {
            Some(body) => body,
            None => {
                println!("Index: {}, Count: {}", index, self.len);
                panic!("Invalid body id")
            }
        }
    }
}

impl IndexMut<BodyId> for Bodies {
    fn index_mut(&mut self, index: BodyId) -> &mut Self::Output {
        match self.get_mut(index) {
            Some(body) => body,
            None => panic!("Invalid body id")
        }
    }
}

pub type ConstraintsMap<T> = FnvHashMap<BodyPair, Vec<T>>;

// TODO: Rename
pub trait ConstraintSolverMap {
    fn initialize_velocity(&mut self, body_map: &Bodies, dt: f32);
    
    fn warm_start_velocity(&mut self, body_map: &mut Bodies, dt: f32);
    fn warm_start_position(&mut self, body_map: &mut Bodies, dt: f32);
    
    fn solve_velocity(&mut self, body_map: &mut Bodies, dt: f32);
    fn solve_position(&mut self, body_map: &mut Bodies, dt: f32);
}

impl<T: Constraint> ConstraintSolverMap for ConstraintsMap<T> {
    fn initialize_velocity(&mut self, body_map: &Bodies, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_ref(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.initialize_velocity(body_a, body_b, dt);
            }
        }
    }
    
    fn warm_start_velocity(&mut self, body_map: &mut Bodies, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_mut(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.warm_start_velocity(body_a, body_b, dt)
            }
        }
    }
    
    fn warm_start_position(&mut self, body_map: &mut Bodies, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_mut(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.warm_start_position(body_a, body_b, dt)
            }
        }
    }
    
    fn solve_velocity(&mut self, body_map: &mut Bodies, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_mut(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.solve_velocity(body_a, body_b, dt)
            }
        }
    }
    fn solve_position(&mut self, body_map: &mut Bodies, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_mut(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.solve_position(body_a, body_b, dt)
            }
        }
    }
}
