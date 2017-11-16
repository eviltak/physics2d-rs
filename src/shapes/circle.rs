use math::*;

pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Circle {
        Circle {
            radius
        }
    }
}

impl super::Matter for Circle {
    fn mass_and_inertia(&self, density: f32) -> (f32, f32) {
        let mass = density * PI * self.radius * self.radius;
        let inertia = 0.5 * mass * self.radius * self.radius;
        
        (mass, inertia)
    }
}
