use world::{World};
use ::math::{Vec2};
use collision::Contact;

pub trait DebugCollision {
    fn contact_points(&self) -> Vec<Vec2>;
    fn contacts(&self) -> Vec<&Contact>;
}

impl DebugCollision for World {
    fn contact_points(&self) -> Vec<Vec2> {
        self.velocity_constraint_manifolds
            .values()
            .flat_map(|manifold| manifold.constraints.iter().map(|constraint| constraint.contact.position))
            .collect()
    }
    
    fn contacts(&self) -> Vec<&Contact> {
        self.velocity_constraint_manifolds
            .values()
            .flat_map(|manifold| manifold.constraints.iter().map(|constraint| &constraint.contact))
            .collect()
    }
}