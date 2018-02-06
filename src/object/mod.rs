pub mod body;

use math::Vec2;

pub struct Bounds {
    min: Vec2,
    max: Vec2,
}

impl Bounds {
    pub fn new(min: Vec2, max: Vec2) -> Bounds {
        Bounds {
            min,
            max,
        }
    }
    
    pub fn with_extents(center: Vec2, extents: Vec2) -> Bounds {
        Bounds::new(center - extents, center + extents)
    }
    
    pub fn intersects(&self, other: &Bounds) -> bool {
        if self.max.x < other.min.x || self.min.x > other.max.x {
            false
        } else if self.max.y < other.min.y || self.min.y > other.max.y {
            false
        } else {
            true
        }
    }
}
