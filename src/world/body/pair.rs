use world::body::{Body, BodyId};
use world::BodyMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyPair(pub BodyId, pub BodyId);
