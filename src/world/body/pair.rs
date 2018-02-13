use super::{Body, BodyId};
use world::BodyMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyPair(pub BodyId, pub BodyId);

impl BodyPair {
    pub fn as_tuple_ref(&self) -> (&BodyId, &BodyId) {
        (&self.0, &self.1)
    }
}
