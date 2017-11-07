
mod runner;
mod canvas;
mod config;

pub use self::runner::run;
pub use self::canvas::Canvas;
pub use self::config::Config;

pub trait Testbed {
    fn sfml_loop(&mut self);
    
    fn sfml_draw(&mut self, canvas: &mut Canvas);
}
