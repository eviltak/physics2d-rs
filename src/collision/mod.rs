
mod detection;
mod resolution;

pub use self::detection::{Collide};
pub use self::resolution::{Manifold, Contact};

use self::detection::collide;
use ::world::Body;

#[derive(Hash, PartialEq, Eq)]
pub struct CollisionPair {
    pub body_id_pair: (usize, usize),
}


impl CollisionPair {
    pub fn new(id_a: usize, id_b: usize) -> CollisionPair {
        CollisionPair {
            body_id_pair: (id_a, id_b),
        }
    }
    
    pub fn check_collision(&self, bodies: &[Body]) -> Option<Manifold> {
        collide(&bodies[self.body_id_pair.0], &bodies[self.body_id_pair.1])
    }
    
    pub fn resolve_collision(&self, bodies: &mut [Body], manifold: &Manifold) {
        let (slice_a, slice_b) = bodies.split_at_mut(self.body_id_pair.0 + 1);
        manifold.resolve(&mut slice_a[self.body_id_pair.0], &mut slice_b[self.body_id_pair.1 - self.body_id_pair.0 - 1]);
    }
}
