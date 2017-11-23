mod circle;
mod polygon;

pub use self::circle::Circle;
pub use self::polygon::Polygon;

pub enum Shape {
    Circle(Circle),
    Polygon(Polygon),
}

generate_match_fn_macro_for_enum!(Shape::{Circle, Polygon}; match_fn_to_shape);

pub trait Matter {
    fn mass_and_inertia(&self, density: f32) -> (f32, f32);
}

impl Matter for Shape {
    fn mass_and_inertia(&self, density: f32) -> (f32, f32) {
        match_fn_to_shape!(*self, mass_and_inertia(density))
    }
}
