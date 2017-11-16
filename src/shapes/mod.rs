mod circle;
mod polygon;

pub use self::circle::Circle;
pub use self::polygon::Polygon;

pub enum Shape {
    Circle(Circle),
    Polygon(Polygon),
}

pub trait Matter {
    fn mass_and_inertia(&self, density: f32) -> (f32, f32);
}

impl Matter for Shape {
    fn mass_and_inertia(&self, density: f32) -> (f32, f32) {
        match *self {
            Shape::Circle(ref circle) => circle.mass_and_inertia(density),
            Shape::Polygon(ref polygon) => polygon.mass_and_inertia(density),
        }
    }
}
