use world::{World};
use ::math::{Vec2};
use collision::Contact;

pub trait DebugCollision {
    fn contacts(&self) -> Vec<&Contact>;
}

impl DebugCollision for World {
    fn contacts(&self) -> Vec<&Contact> {
        self.contact_constraints
            .values()
            .flat_map(|constraints| constraints.iter().map(|constraint| &constraint.contact))
            .collect()
    }
}