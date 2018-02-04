use world::{World, BodyPair};
use ::math::{Vec2};
use collision::ContactManifold;

use std::collections::HashSet;

pub trait DebugCollision {
    fn contact_points(&self) -> Vec<Vec2>;
    fn manifolds(&self) -> Vec<&ContactManifold>;
}

impl DebugCollision for World {
    fn contact_points(&self) -> Vec<Vec2> {
        self.manifolds
            .values()
            .flat_map(
                |ref m|
                    m.contacts.iter()
                     .map(|ref c| c.position))
            .collect()
    }
    
    fn manifolds(&self) -> Vec<&ContactManifold> {
        self.manifolds.values().collect()
    }
}