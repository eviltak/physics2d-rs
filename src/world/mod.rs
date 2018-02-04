mod body;
mod transform;

pub use self::body::{Body, BodyId};
pub use self::transform::Transform;

use ::collision::{Manifold, CollisionPair};

use fnv::{FnvHashMap, FnvHashSet};

use std::cell::RefCell;

pub(crate) type BodyMap = FnvHashMap<BodyId, RefCell<Body>>;

pub struct World {
    pub bodies: BodyMap,
    
    // TODO: Extract to broadphaser
    pub(crate) manifolds: FnvHashMap<CollisionPair, Manifold>,
    
    body_created_count: usize,
}

impl World {
    pub fn new() -> World {
        World {
            bodies: FnvHashMap::default(),
            manifolds: FnvHashMap::default(),
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
        
        for body_a in self.bodies.keys() {
            for body_b in self.bodies.keys() {
                if body_b <= body_a {
                    continue;
                }
                
                let collision_pair = CollisionPair::new(*body_a, *body_b);
                
                if let Some(new_contacts) = collision_pair.check_collision(&self.bodies) {
                    // If we already have a manifold with the given bodies, update contacts
                    if self.manifolds.contains_key(&collision_pair) {
                        let manifold = self.manifolds.get_mut(&collision_pair).unwrap();
                        manifold.update_contacts(new_contacts);
                    } else {
                        self.manifolds.insert(collision_pair, Manifold::new(collision_pair, new_contacts));
                    }
                } else {
                    self.manifolds.remove(&collision_pair);
                }
            }
        }
        
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_force(dt);
        }
        
        // TODO: Get rid of CollisionPair/Manifold, store everything in Contact
        for (collision_pair, manifold) in self.manifolds.iter_mut() {
            collision_pair.pre_step(&mut self.bodies, manifold, dt);
        }
        
        for (collision_pair, manifold) in self.manifolds.iter_mut() {
            collision_pair.resolve_collision(&mut self.bodies, manifold, dt);
        }
        
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_velocity(dt);
        }
    }
}
