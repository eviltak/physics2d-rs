use world::{BodyId, BodyRef, BodyPair};
use collision::Contact;
use constraint::ConstraintSolver;

use fnv::FnvHashMap;

pub type BodyMap = FnvHashMap<BodyId, BodyRef>;
pub type ContactsMap = FnvHashMap<BodyPair, Vec<Contact>>;

pub type ManifoldMap<T: ConstraintSolver> = FnvHashMap<BodyPair, T>;

pub trait ConstraintSolverMap {
    fn initialize_constraints(&mut self, body_map: &BodyMap, dt: f32);
    fn warm_start(&mut self, body_map: &BodyMap, dt: f32);
    fn solve_constraints(&mut self, body_map: &BodyMap, dt: f32);
}

impl<T: ConstraintSolver> ConstraintSolverMap for ManifoldMap<T> {
    fn initialize_constraints(&mut self, body_map: &BodyMap, dt: f32) {
        for (body_pair, manifold) in self.iter_mut() {
            body_pair.with(body_map, |body_a, body_b| manifold.initialize_constraints(body_a, body_b, dt));
        }
    }
    fn warm_start(&mut self, body_map: &BodyMap, dt: f32) {
        for (body_pair, manifold) in self.iter_mut() {
            body_pair.with_mut(body_map, |body_a, body_b| manifold.warm_start(body_a, body_b, dt));
        }
    }
    
    fn solve_constraints(&mut self, body_map: &BodyMap, dt: f32) {
        for (body_pair, manifold) in self.iter_mut() {
            body_pair.with_mut(body_map, |body_a, body_b| manifold.solve_constraints(body_a, body_b, dt));
        }
    }
}