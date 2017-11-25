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
    
    fn sfml_vec2(&self, mut v: Vec2) -> sfml::system::Vector2f {
        v *= self.pixels_per_unit;
        v.y = -v.y;
        sfml::system::Vector2f::new(v.x, v.y)
    }
    
    fn config_shape<'a, T: sfml::graphics::Shape<'a>>(&self, shape: &mut T) {
        shape.set_fill_color(&sfml::graphics::Color::TRANSPARENT);
    
        // TODO: parameter?
        shape.set_outline_color(&sfml::graphics::Color::CYAN);
    
        // TODO: Input from config?
        shape.set_outline_thickness(1.0);
    }
    
    fn get_circle_drawable(&self, sfml_pos: sfml::system::Vector2f,
                           circle: &shapes::Circle) -> Box<sfml::graphics::Drawable> {
        const POINT_COUNT: u32 = 60;
    
        let radius = circle.radius * self.pixels_per_unit;
        let drawable_pos = sfml_pos - sfml::system::Vector2f::new(1.0, 1.0) * radius;
    
        let mut circle_shape = sfml::graphics::CircleShape::new(radius,
                                                                POINT_COUNT);
        
        circle_shape.set_position(drawable_pos);
        
        self.config_shape(&mut circle_shape);
        
        Box::new(circle_shape)
    }
    
    fn get_polygon_drawable<'a>(&self, sfml_pos: sfml::system::Vector2f,
                                body: &Body,
                                polygon: &shapes::Polygon) -> Box<sfml::graphics::Drawable> {
        let mut convex_shape = sfml::graphics::ConvexShape::new(polygon.vert_count() as u32);
    
        for i in 0..polygon.vert_count() {
            convex_shape.set_point(i as u32,
                                   self.sfml_vec2(
                                       // TODO: Use transform instead?
                                       Mat2::rotation(body.rotation) * polygon.vertices[i])
            );
        }
        
        let bounds = convex_shape.local_bounds();
        let drawable_pos = sfml_pos - 0.5 * sfml::system::Vector2f::new(bounds.width, bounds.height);
        
        convex_shape.set_position(drawable_pos);
        
        self.config_shape(&mut convex_shape);
        
        Box::new(convex_shape)
    }
    
    fn get_body_drawable(&self, body: &Body) -> Box<sfml::graphics::Drawable> {
        let sfml_pos = self.sfml_vec2(body.position);
        
        let drawable = match body.shape {
            shapes::Shape::Circle(ref circle) => self.get_circle_drawable(sfml_pos, circle),
            shapes::Shape::Polygon(ref polygon) => self.get_polygon_drawable(sfml_pos, body, polygon),
        };
        
        drawable
    }
    
    pub fn draw_body(&mut self, body: &Body) {
        let drawable = self.get_body_drawable(body);
    
        self.draw_queue.push(drawable);
    }
    
    pub fn draw_point(&mut self, point: Vec2) {
        let sfml_pos = self.sfml_vec2(point);
        
        let drawable = self.get_circle_drawable(sfml_pos, &shapes::Circle::new(0.2));
        
        self.draw_queue.push(drawable);
    }
    
    pub fn process_draw_queue(&mut self, window: &mut sfml::graphics::RenderWindow) {
        window.set_view(&self.view);
        
        for drawable in self.draw_queue.iter() {
            window.draw((*drawable).deref());
        }
        
        self.draw_queue.clear();
    }
}
