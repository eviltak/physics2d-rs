mod body;
mod transform;

pub use self::body::Body;
pub use self::transform::Transform;

use ::collision::{Manifold, CollisionPair};

use std::collections::HashMap;

pub struct World {
    pub bodies: Vec<Body>,
    
    // TODO: Use FnvHashmap
    // TODO: Extract to broadphaser
    pub(crate) collision_pairs: HashMap<CollisionPair, Manifold>,
    
}

impl World {
    pub fn new() -> World {
        World {
            bodies: Vec::new(),
            collision_pairs: HashMap::new(),
        }
    }
    
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
    
    pub fn update(&mut self, dt: f32) {
        for body in &mut self.bodies {
            body.update(dt);
        }
        
        for i in 0..self.bodies.len() - 1 {
            for j in i+1..self.bodies.len() {
                let collision_pair = CollisionPair::new(i, j);
                if let Some(manifold) = collision_pair.check_collision(&self.bodies) {
                    // TODO: Do not replace manifold for cached contacts
                    self.collision_pairs.insert(collision_pair, manifold);
                } else {
                    self.collision_pairs.remove(&collision_pair);
                }
            }
        }
    
        for body in &mut self.bodies {
            body.integrate_force(dt);
        }
    
        for (collision_pair, manifold) in self.collision_pairs.iter() {
            collision_pair.resolve_collision(&mut self.bodies, manifold);
        }
    
        for body in &mut self.bodies {
            body.integrate_velocity(dt);
        }
    }
}
