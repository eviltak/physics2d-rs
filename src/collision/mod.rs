mod detection;
mod resolution;

pub use self::detection::{Collide};
pub use self::resolution::{Manifold, Contact};

use self::detection::collide;
use ::world::{Body, BodyId};

use fnv::FnvHashMap;

use std::cell::RefCell;

#[derive(Hash, PartialEq, Eq)]
pub struct CollisionPair {
    pub body_id_pair: (BodyId, BodyId),
}

impl CollisionPair {
    pub fn new(id_a: BodyId, id_b: BodyId) -> CollisionPair {
        assert_ne!(id_a, id_b);
        
        // Smallest id is always first in pair
        let (id_a, id_b) = (id_a.min(id_b), id_a.max(id_b));
        
        CollisionPair {
            body_id_pair: (id_a, id_b),
        }
    }
    
    pub fn check_collision(&self, bodies: &FnvHashMap<BodyId, RefCell<Body>>) -> Option<Manifold> {
        collide(&bodies[&self.body_id_pair.0].borrow(), &bodies[&self.body_id_pair.1].borrow())
    }
    
    pub fn resolve_collision(&self, bodies: &mut FnvHashMap<BodyId, RefCell<Body>>, manifold: &Manifold, dt: f32) {
        let body_a = &mut bodies[&self.body_id_pair.0].borrow_mut();
        let body_b = &mut bodies[&self.body_id_pair.1].borrow_mut();
        
        manifold.resolve(body_a, body_b, dt);
    }
}
