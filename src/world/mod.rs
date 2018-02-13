mod transform;
mod collections;
mod body;
pub mod debug;

pub use self::body::{Body, BodyId, BodyRef, Material};
pub use self::transform::Transform;
pub(crate) use self::body::BodyPair;
pub(crate) use self::collections::{BodyMap, BodiesIter};

use self::collections::{ConstraintsMap, ConstraintSolverMap};
use collision::{ContactConstraint, collide};
use collision::broad_phase::{BroadPhase, NaiveBroadPhase};

pub struct World {
    bodies: BodyMap,
    
    broad_phase: NaiveBroadPhase,
    
    contact_constraints: ConstraintsMap<ContactConstraint>,
    
    body_created_count: usize,
    
    pub velocity_iterations: u8,
    pub position_iterations: u8,
}

impl Default for World {
    fn default() -> World {
        World::new(8, 2)
    }
}

impl World {
    pub fn new(velocity_iterations: u8, position_iterations: u8) -> World {
        World {
            bodies: BodyMap::default(),
            broad_phase: NaiveBroadPhase,
            contact_constraints: ConstraintsMap::default(),
            body_created_count: 0,
            velocity_iterations,
            position_iterations,
        }
    }
    
    pub fn add_body(&mut self, mut body: Body) -> BodyId {
        let body_id = self.body_created_count as BodyId;
        self.body_created_count += 1;
        
        body.id = body_id;
        self.bodies.insert(body_id, BodyRef::new(body));
        
        body_id
    }
    
    pub fn get_body(&self, body_id: &BodyId) -> &BodyRef {
        &self.bodies[body_id]
    }
    
    pub fn bodies_iter(&self) -> BodiesIter {
        BodiesIter {
            values: self.bodies.values()
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.update(dt);
        }
        
        let potential_pairs = self.broad_phase.potential_pairs(&self.bodies);
        
        // TODO: Make hashmap return new pairs only, retain by overlap check
        self.contact_constraints.retain(|pair, _v| potential_pairs.contains(&pair));
    
        for pair in potential_pairs {
            let body_a = &self.bodies[&pair.0].borrow();
            let body_b = &self.bodies[&pair.1].borrow();
            
            if let Some(new_contacts) = collide(body_a, body_b) {
                let new_constraints = ContactConstraint::with_persistent_contacts(
                    self.contact_constraints.get(&pair),
                    &new_contacts
                );
                
                self.contact_constraints.insert(pair, new_constraints);
            } else {
                self.contact_constraints.remove(&pair);
            }
        }
        
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_force(dt);
        }
        
        self.contact_constraints.initialize_velocity(&self.bodies, dt);
        self.contact_constraints.warm_start_velocity(&self.bodies, dt);
        
        for _ in 0..self.velocity_iterations {
            self.contact_constraints.solve_velocity(&self.bodies, dt);
        }
        
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_velocity(dt);
        }
        
        self.contact_constraints.warm_start_position(&self.bodies, dt);
    
        for _ in 0..self.position_iterations {
            self.contact_constraints.solve_position(&self.bodies, dt);
        }
    }
}
