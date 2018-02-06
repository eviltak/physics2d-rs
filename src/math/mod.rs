mod vec2;
mod mat2;
mod bounds;

pub use self::vec2::{Vec2, Cross};
pub use self::mat2::Mat2;
pub use self::bounds::Bounds;

use std::f32;

/// The mathematical constant Pi.
pub const PI: f32 = f32::consts::PI;

/// Infinity; a value that is larger than all other possible values.
pub const INFINITY: f32 = f32::INFINITY;

/// Clamps (limits) the value of `x` in the inclusive range from `min` to `max`.
///
/// If `x < min`, `min` is returned. If `x > max`, `max` is returned.
///
/// # Examples
///
/// ```
/// # use physics2d::math::clamp;
///
/// assert_eq!(clamp(-10.0, -3.0, 5.0), -3.0);
/// assert_eq!(clamp(1.0, -3.0, 5.0), 1.0);
/// assert_eq!(clamp(999999.0, -3.0, 5.0), 5.0);
/// ```
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Clamps (limits) the value of `x` in the inclusive range from `0.0` to `1.0`.
///
/// If `x < 0.0`, `0.0` is returned. If `x > 1.0`, `1.0` is returned.
///
/// # Examples
///
/// ```
/// # use physics2d::math::clamp01;
///
/// assert_eq!(clamp01(-10.0), 0.0);
/// assert_eq!(clamp01(0.12345678), 0.12345678);
/// assert_eq!(clamp01(999.8989), 1.0);
/// ```
pub fn clamp01(x: f32) -> f32 {
    clamp(x, 0.0, 1.0)
}

