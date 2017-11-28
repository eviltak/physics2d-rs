use ::math::{Vec2};

pub struct Contact {
    pub point: Vec2,
    pub penetration: f32,
}

pub struct Manifold {
    pub normal: Vec2,
    pub contacts: Vec<Contact>,
}

impl Manifold {
    pub fn new(normal: Vec2, contacts: Vec<Contact>) -> Manifold {
        Manifold {
            normal,
            contacts,
        }
    }
}
