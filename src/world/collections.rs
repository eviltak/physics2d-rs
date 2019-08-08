extern crate core;

use world::{BodyId, Body, BodyPair};
use collision::Contact;
use constraint::Constraint;

use fnv::FnvHashMap;

use std::collections::hash_map::Values;

pub type BodyMap = FnvHashMap<BodyId, Body>;

pub type ConstraintsMap<T: Constraint> = FnvHashMap<BodyPair, Vec<T>>;

// TODO: Rename
pub trait ConstraintSolverMap {
    fn initialize_velocity(&mut self, body_map: &BodyMap, dt: f32);
    
    fn warm_start_velocity(&mut self, body_map: &mut BodyMap, dt: f32);
    fn warm_start_position(&mut self, body_map: &mut BodyMap, dt: f32);
    
    fn solve_velocity(&mut self, body_map: &mut BodyMap, dt: f32);
    fn solve_position(&mut self, body_map: &mut BodyMap, dt: f32);
}

impl<T: Constraint> ConstraintSolverMap for ConstraintsMap<T> {
    fn initialize_velocity(&mut self, body_map: &BodyMap, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_ref(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.initialize_velocity(body_a, body_b, dt);
            }
        }
    }
    
    fn warm_start_velocity(&mut self, body_map: &mut BodyMap, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_mut(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.warm_start_velocity(body_a, body_b, dt)
            }
        }
    }
    
    fn warm_start_position(&mut self, body_map: &mut BodyMap, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_mut(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.warm_start_position(body_a, body_b, dt)
            }
        }
    }
    
    fn solve_velocity(&mut self, body_map: &mut BodyMap, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_mut(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.solve_velocity(body_a, body_b, dt)
            }
        }
    }
    fn solve_position(&mut self, body_map: &mut BodyMap, dt: f32) {
        for (body_pair, constraints) in self.iter_mut() {
            let (body_a, body_b) = body_pair.as_mut(body_map);
            
            for constraint in constraints.iter_mut() {
                constraint.solve_position(body_a, body_b, dt)
            }
        }
    }
}
