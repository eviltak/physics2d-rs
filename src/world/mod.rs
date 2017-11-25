mod bodies;

pub use self::bodies::Body;

use ::collision;
use ::collision::{Manifold};

pub struct World {
    pub bodies: Vec<Body>,
    pub(crate) manifolds: Vec<Manifold>,
    
}

impl World {
    pub fn new() -> World {
        World {
            bodies: Vec::new(),
            manifolds: Vec::new(),
        }
    }
    
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
    
    pub fn update(&mut self, dt: f32) {
        for i in 0..self.bodies.len() - 1 {
            for j in i+1..self.bodies.len() {
                if let Some(m) = collision::collide(&self.bodies[i], &self.bodies[j]) {
                    self.manifolds.push(m);
                }
            }
        }
        
        
        //self.manifolds.clear();
        
        for body in &mut self.bodies {
            body.update(dt);
        }
    }
}
