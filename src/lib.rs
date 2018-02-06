extern crate fnv;

#[macro_use]
mod util;

#[macro_use]
pub mod shapes;

pub mod math;

mod collision;
mod constraint;
mod world;

pub use world::debug;

pub use math::{Vec2, Mat2, Cross, Bounds};
pub use world::{World, Body, BodyRef, BodyId, Material, Transform};
