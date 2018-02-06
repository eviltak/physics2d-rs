use world::body::{Body, BodyId};
use world::BodyMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyPair(pub BodyId, pub BodyId);

impl BodyPair {
    pub fn with<F, R>(&self, bodies: &BodyMap, mut f: F) -> R
        where F: FnMut(&Body, &Body) -> R {
        let body_a = &bodies[&self.0].borrow();
        let body_b = &bodies[&self.1].borrow();
        
        f(body_a, body_b)
    }
    
    pub fn with_mut<F, R>(&self, bodies: &BodyMap, mut f: F) -> R
        where F: FnMut(&mut Body, &mut Body) -> R {
        let body_a = &mut bodies[&self.0].borrow_mut();
        let body_b = &mut bodies[&self.1].borrow_mut();
        
        f(body_a, body_b)
    }
}
