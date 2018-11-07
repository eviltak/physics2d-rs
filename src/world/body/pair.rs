use super::{Body, BodyId};
use world::BodyMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyPair(pub BodyId, pub BodyId);

impl BodyPair {
    pub fn new(id_a: BodyId, id_b: BodyId) -> BodyPair {
        let (id_a, id_b) = (id_a.min(id_b), id_a.max(id_b));
        BodyPair(id_a, id_b)
    }
    
    pub fn with<F, R>(&self, bodies: &BodyMap, mut f: F) -> R
        where F: FnMut(&Body, &Body) -> R {
        let body_a = &bodies[&self.0];
        let body_b = &bodies[&self.1];
        
        f(body_a, body_b)
    }
    
    pub fn with_mut<F, R>(&self, bodies: &mut BodyMap, mut f: F) -> R
        where F: FnMut(&mut Body, &mut Body) -> R {
        unsafe {
            let body_a = bodies.get_mut(&self.0).unwrap() as *mut _;
            let body_b = bodies.get_mut(&self.1).unwrap() as *mut _;
    
            f(&mut *body_a, &mut *body_b)
        }
    }
}
