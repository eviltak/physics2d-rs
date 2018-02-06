use world::{World};
use ::math::{Vec2};
use collision::Contact;

pub trait DebugCollision {
    fn contact_points(&self) -> Vec<Vec2>;
    fn contacts(&self) -> Vec<&Contact>;
}

impl DebugCollision for World {
    fn contact_points(&self) -> Vec<Vec2> {
        self.contacts
            .values()
            .flat_map(
                |ref contacts|
                    contacts.iter()
                     .map(|ref c| c.position))
            .collect()
    }
    
    fn contacts(&self) -> Vec<&Contact> {
        self.contacts.values().flat_map(|contacts| contacts).collect()
    }
}