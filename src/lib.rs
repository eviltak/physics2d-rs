extern crate fnv;

#[macro_use]
mod util;

#[macro_use]
pub mod shapes;

pub mod math;

mod collision;
mod constraint;
mod world;
mod joint;

pub use crate::world::debug;

pub use crate::math::{Vec2, Mat2, Cross, Bounds};
pub use crate::world::{World, Body, BodyId, Material, Transform};
pub use crate::joint::{Joint, SpringJoint};
