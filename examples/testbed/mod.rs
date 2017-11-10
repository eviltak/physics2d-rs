
mod runner;
mod canvas;
mod config;

pub use self::runner::run;
pub use self::canvas::Canvas;
pub use self::config::Config;

pub trait Testbed {
    fn sfml_loop(&mut self, dt: f32);
    
    fn sfml_draw(&mut self, canvas: &mut Canvas, dt: f32);
}
