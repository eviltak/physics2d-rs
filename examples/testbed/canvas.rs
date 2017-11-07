use sfml;
use sfml::graphics::{Shape, RenderTarget, Transformable};

use physics2d::*;
use testbed::config::Config;

use std::ops::Deref;

pub struct Canvas {
    draw_queue: Vec<Box<sfml::graphics::Drawable>>,
    view: sfml::graphics::View,
    
    pixels_per_unit: f32,
}

impl Canvas {
    pub fn new(config: &Config) -> Canvas {
        Canvas {
            draw_queue: vec![],
            view: sfml::graphics::View::new(sfml::system::Vector2f::new(0.0, 0.0),
                                            sfml::system::Vector2f::new(config.window_width as f32,
                                                                        config.window_height as f32)),
            pixels_per_unit: config.pixels_per_unit,
        }
    }
    
    fn sfml_vec2(&self, mut v: math::Vec2) -> sfml::system::Vector2f {
        v *= self.pixels_per_unit;
        v.y = -v.y;
        sfml::system::Vector2f::new(v.x, v.y)
    }
    
    // TODO: Use circle engine shape instead
    pub fn draw_circle(&mut self, pos: math::Vec2, mut rad: f32) {
        const POINT_COUNT: u32 = 60;
        
        rad *= self.pixels_per_unit;
        
        let mut sfml_pos = self.sfml_vec2(pos);
        sfml_pos -= sfml::system::Vector2f::new(1.0, 1.0) * rad;
        
        let mut circle_shape = Box::new(sfml::graphics::CircleShape::new(rad, POINT_COUNT));
        circle_shape.set_position(sfml_pos);
        
        circle_shape.set_fill_color(&sfml::graphics::Color::TRANSPARENT);
        
        // TODO: parameter?
        circle_shape.set_outline_color(&sfml::graphics::Color::CYAN);
        
        // TODO: Input from config?
        circle_shape.set_outline_thickness(1.0);
        
        self.draw_queue.push(circle_shape);
    }
    
    pub fn draw_queue_to_window(&self, window: &mut sfml::graphics::RenderWindow) {
        window.set_view(&self.view);
        
        for drawable in self.draw_queue.iter() {
            window.draw((*drawable).deref());
        }
    }
}
