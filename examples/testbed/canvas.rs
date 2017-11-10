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
    
    fn get_circle_drawable<'a>(&self, sfml_pos: sfml::system::Vector2f,
                           circle: &shapes::Circle) -> sfml::graphics::CircleShape<'a> {
        const POINT_COUNT: u32 = 60;
    
        let radius = circle.radius * self.pixels_per_unit;
        let drawable_pos = sfml_pos - sfml::system::Vector2f::new(1.0, 1.0) * radius;
    
        let mut circle_shape = sfml::graphics::CircleShape::new(radius,
                                                                POINT_COUNT);
        
        circle_shape.set_position(drawable_pos);
        
        circle_shape
    }
    
    pub fn draw_body(&mut self, body: &world::Body) {
        let sfml_pos = self.sfml_vec2(body.position);
        
        let mut drawable = match body.shape {
            shapes::Shape::Circle(ref circle) => self.get_circle_drawable(sfml_pos, circle)
        };
    
        drawable.set_fill_color(&sfml::graphics::Color::TRANSPARENT);
    
        // TODO: parameter?
        drawable.set_outline_color(&sfml::graphics::Color::CYAN);
    
        // TODO: Input from config?
        drawable.set_outline_thickness(1.0);
    
        self.draw_queue.push(Box::new(drawable));
    }
    
    pub fn draw_queue_to_window(&self, window: &mut sfml::graphics::RenderWindow) {
        window.set_view(&self.view);
        
        for drawable in self.draw_queue.iter() {
            window.draw((*drawable).deref());
        }
    }
}
