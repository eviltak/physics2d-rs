pub mod body;

use math::Vec2;

pub struct Aabb {
    min: Vec2,
    max: Vec2,
}

impl Aabb {
    pub fn new(min: Vec2, max: Vec2) -> Aabb {
        Aabb {
            min,
            max,
        }
    }
}
