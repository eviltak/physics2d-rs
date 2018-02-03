mod body;
mod transform;

pub use self::body::{Body, BodyId};
pub use self::transform::Transform;

use ::collision::{Manifold, CollisionPair};

use fnv::FnvHashMap;

use std::cell::RefCell;

pub struct World {
    pub bodies: FnvHashMap<BodyId, RefCell<Body>>,
    
    // TODO: Extract to broadphaser
    pub(crate) collision_pairs: FnvHashMap<CollisionPair, Manifold>,
    
    body_created_count: usize,
}

impl World {
    pub fn new() -> World {
        World {
            bodies: FnvHashMap::default(),
            collision_pairs: FnvHashMap::default(),
            body_created_count: 0,
        }
    }
    
    pub fn add_body(&mut self, body: Body) -> BodyId {
        let body_id = self.body_created_count as BodyId;
        self.bodies.insert(body_id, RefCell::new(body));
        
        self.body_created_count += 1;
        
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
                
                if let Some(manifold) = collision_pair.check_collision(&self.bodies) {
                    // TODO: Do not replace manifold for cached contacts
                    self.collision_pairs.insert(collision_pair, manifold);
                } else {
                    self.collision_pairs.remove(&collision_pair);
                }
            }
        }
    
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_force(dt);
        }
    
        for (collision_pair, manifold) in self.collision_pairs.iter() {
            collision_pair.resolve_collision(&mut self.bodies, manifold, dt);
        }
    
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_velocity(dt);
        }
    }
}
