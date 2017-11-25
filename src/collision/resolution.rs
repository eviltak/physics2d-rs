use ::math::{Vec2};

pub struct Contact {
    pub point: Vec2,
    pub penetration: f32,
}

pub struct Manifold {
    pub contacts: Vec<Contact>,
    pub normal: Vec2,
}

impl Manifold {
    pub fn new() -> Manifold {
        Manifold {
            contacts: Vec::new(),
            normal: Vec2::ZERO
        }
    }
}
