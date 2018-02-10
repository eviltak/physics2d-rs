pub extern crate sfml;

mod runner;
mod canvas;
mod config;
mod input;

pub use self::runner::run;
pub use self::canvas::Canvas;
pub use self::config::Config;
pub use self::input::{Input, Key};

use physics2d::Vec2;

pub trait Testbed {
    fn sfml_loop(&mut self, input: &input::Input, dt: f32);
    
    fn sfml_draw(&mut self, canvas: &mut Canvas, dt: f32);
}

fn sfml_vec2(mut v: Vec2, pixels_per_unit: f32) -> sfml::system::Vector2f {
    v *= pixels_per_unit;
    v.y = -v.y;
    sfml::system::Vector2f::new(v.x, v.y)
}

fn physics2d_vec2(mut v: sfml::system::Vector2f, pixels_per_unit: f32) -> Vec2 {
    v.y = -v.y;
    v /= pixels_per_unit;
    Vec2::new(v.x, v.y)
}
