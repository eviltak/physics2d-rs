mod body;
mod transform;

pub use self::body::{Body, BodyId};
pub use self::transform::Transform;
pub(crate) use self::body::BodyPair;

use collision::{ContactManifold, collide};
use constraint::contact::VelocityContactSolver;
use constraint::Solver;

use fnv::{FnvHashMap, FnvHashSet};

use std::cell::RefCell;

pub(crate) type BodyMap = FnvHashMap<BodyId, RefCell<Body>>;

pub struct World {
    pub bodies: BodyMap,
    
    // TODO: Extract to broadphaser
    pub(crate) manifolds: FnvHashMap<BodyPair, ContactManifold>,
    
    velocity_contact_solver: VelocityContactSolver,
    
    body_created_count: usize,
}

impl World {
    pub fn new() -> World {
        World {
            bodies: FnvHashMap::default(),
            manifolds: FnvHashMap::default(),
            velocity_contact_solver: VelocityContactSolver,
            body_created_count: 0,
        }
    }
    
    pub fn add_body(&mut self, mut body: Body) -> BodyId {
        let body_id = self.body_created_count as BodyId;
        self.body_created_count += 1;
        
        body.id = body_id;
        self.bodies.insert(body_id, RefCell::new(body));
        
        body_id
    }
    
    pub fn update(&mut self, dt: f32) {
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.update(dt);
        }
        
        for body_a_id in self.bodies.keys() {
            for body_b_id in self.bodies.keys() {
                if body_b_id <= body_a_id {
                    continue;
                }
                
                let body_pair = BodyPair(*body_a_id, *body_b_id);
    
                let body_a = &self.bodies[&body_pair.0].borrow();
                let body_b = &self.bodies[&body_pair.1].borrow();
                
                if let Some(new_contacts) = collide(body_a, body_b) {
                    // If we already have a manifold with the given bodies, update contacts
                    if self.manifolds.contains_key(&body_pair) {
                        let manifold = self.manifolds.get_mut(&body_pair).unwrap();
                        manifold.update_contacts(new_contacts);
                    } else {
                        self.manifolds.insert(body_pair, ContactManifold::new(body_pair, new_contacts));
                    }
                } else {
                    self.manifolds.remove(&body_pair);
                }
            }
        }
        
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_force(dt);
        }
        
        for (body_pair, manifold) in self.manifolds.iter_mut() {
            let body_a = &self.bodies[&body_pair.0].borrow();
            let body_b = &self.bodies[&body_pair.1].borrow();
            
            self.velocity_contact_solver.initialize(manifold, body_a, body_b, dt);
        }
    
        for (body_pair, manifold) in self.manifolds.iter_mut() {
            let body_a = &mut self.bodies[&body_pair.0].borrow_mut();
            let body_b = &mut self.bodies[&body_pair.1].borrow_mut();
        
            self.velocity_contact_solver.warm_start(manifold, body_a, body_b, dt);
        }
        
        for (body_pair, manifold) in self.manifolds.iter_mut() {
            let body_a = &mut self.bodies[&body_pair.0].borrow_mut();
            let body_b = &mut self.bodies[&body_pair.1].borrow_mut();
        
            self.velocity_contact_solver.solve(manifold, body_a, body_b, dt);
        }
        
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_velocity(dt);
        }
    }
}
