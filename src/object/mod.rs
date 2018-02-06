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
    
    pub fn with_extents(center: Vec2, extents: Vec2) -> Aabb {
        Aabb::new(center - extents, center + extents)
    }
}
