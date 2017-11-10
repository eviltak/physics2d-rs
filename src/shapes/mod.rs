mod circle;

pub use self::circle::Circle;

pub enum Shape {
    Circle(Circle),
}