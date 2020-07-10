use super::{Body, BodyId};
use crate::world::Bodies;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyPair(pub BodyId, pub BodyId);

impl BodyPair {
    pub fn new(id_a: BodyId, id_b: BodyId) -> BodyPair {
        let (id_a, id_b) = (id_a.min(id_b), id_a.max(id_b));
        BodyPair(id_a, id_b)
    }

    pub fn as_ref<'a>(&self, bodies: &'a Bodies) -> (&'a Body, &'a Body) {
        (&bodies[self.0], &bodies[self.1])
    }

    pub fn as_mut<'a>(&self, bodies: &'a mut Bodies) -> (&'a mut Body, &'a mut Body) {
        unsafe {
            let body_a = bodies.get_mut(self.0).unwrap() as *mut _;
            let body_b = bodies.get_mut(self.1).unwrap() as *mut _;

            (&mut *body_a, &mut *body_b)
        }
    }

    pub fn with<F, R>(&self, bodies: &Bodies, mut f: F) -> R
        where F: FnMut(&Body, &Body) -> R {
        let (body_a, body_b) = self.as_ref(bodies);

        f(body_a, body_b)
    }

    pub fn with_mut<F, R>(&self, bodies: &mut Bodies, mut f: F) -> R
        where F: FnMut(&mut Body, &mut Body) -> R {
        let (body_a, body_b) = self.as_mut(bodies);

        f(body_a, body_b)
    }
}
