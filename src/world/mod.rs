mod body;
mod transform;

pub use self::body::Body;
pub use self::transform::Transform;

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
        for body in &mut self.bodies {
            body.update(dt);
        }
        
        for i in 0..self.bodies.len() - 1 {
            for j in i+1..self.bodies.len() {
                if let Some(m) = collision::collide(&self.bodies[i], &self.bodies[j]) {
                    self.manifolds.push(m);
                }
            }
        }
    
        for body in &mut self.bodies {
            body.integrate_force(dt);
        }
    
        for body in &mut self.bodies {
            body.integrate_velocity(dt);
        }
    }
}
