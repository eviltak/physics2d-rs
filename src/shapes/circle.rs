use math::{PI, Vec2};
use object::Aabb;
use world::Transform;

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
    
    fn aabb(&self, transform: Option<&Transform>) -> Aabb {
        let center = transform.map_or(Vec2::ZERO, |t| t.position);
        let extents = Vec2::ONE * self.radius;
        
        Aabb::with_extents(center, extents)
    }
}
