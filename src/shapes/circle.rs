use crate::math::{PI, Vec2};
use crate::math::Bounds;
use crate::world::Transform;

#[derive(Clone)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle {
            radius
        }
    }
    
    pub fn into_shape(self) -> super::Shape {
        super::Shape::Circle(self)
    }
}

impl super::Matter for Circle {
    fn mass_and_inertia(&self, density: f32) -> (f32, f32) {
        let mass = density * PI * self.radius * self.radius;
        let inertia = 0.5 * mass * self.radius * self.radius;
        
        (mass, inertia)
    }
    
    fn bounds(&self, transform: Option<&Transform>) -> Bounds {
        let center = transform.map_or(Vec2::ZERO, |t| t.position);
        let extents = Vec2::ONE * self.radius;
        
        Bounds::center_extents(center, extents)
    }
}
