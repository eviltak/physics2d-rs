use ::world::{World};
use ::math::{Vec2};
use collision::Manifold;
use std::collections::hash_map::Values;
use collision::CollisionPair;

pub trait DebugCollision {
    fn contact_points(&self) -> Vec<Vec2>;
    fn manifolds(&self) -> Values<CollisionPair, Manifold>;
}

impl DebugCollision for World {
    fn contact_points(&self) -> Vec<Vec2> {
        self.collision_pairs
            .values()
            .flat_map(
                |ref m|
                    m.contacts.iter()
                     .map(|ref c| c.point))
            .collect()
    }
    
    fn manifolds(&self) -> Values<CollisionPair, Manifold> {
        self.collision_pairs.values()
    }
}