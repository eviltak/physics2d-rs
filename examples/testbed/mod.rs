
pub mod runner;
pub mod config;
pub mod canvas;

use sfml;

pub trait Testbed {
    fn config(&self) -> &config::Config;

    fn sfml_loop(&mut self);
}
