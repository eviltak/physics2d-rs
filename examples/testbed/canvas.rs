extern crate physics2d;

use crate::testbed::sfml;
use crate::testbed::sfml::graphics::{RenderTarget, Transformable};

use std::{env, path};

use physics2d::*;
use crate::testbed::config::Config;
use crate::testbed::sfml_vec2;

use std::ops::Deref;

fn get_font_dir() -> path::PathBuf {
    let mut font_dir = env::current_exe().unwrap();
    
    font_dir.pop();     // project/target/debug/examples
    font_dir.pop();     // project/target/debug
    font_dir.pop();     // project/target
    font_dir.pop();     // project/
    
    font_dir.push("examples");
    font_dir.push("assets");
    font_dir.push("fonts");
    font_dir.push("RobotoMono.ttf");
    
    font_dir
}

pub struct Canvas {
    draw_queue: Vec<Box<dyn sfml::graphics::Drawable>>,
    text_queue: Vec<(String, u32)>,
    
    view: sfml::graphics::View,
    font: sfml::graphics::Font,
    
    pixels_per_unit: f32,
}

impl Canvas {
    pub fn new(config: &Config) -> Canvas {
        Canvas {
            draw_queue: Vec::new(),
            text_queue: Vec::new(),
            view: sfml::graphics::View::new(sfml::system::Vector2f::new(0.0, 0.0),
                                            sfml::system::Vector2f::new(config.window_width as f32,
                                                                        config.window_height as f32)),
            font: sfml::graphics::Font::from_file(get_font_dir().to_str().unwrap()).unwrap(),
            pixels_per_unit: config.pixels_per_unit,
        }
    }
    
    fn config_shape<'a, T: sfml::graphics::Shape<'a>>(&self, shape: &mut T) {
        shape.set_fill_color(&sfml::graphics::Color::TRANSPARENT);
        
        // TODO: parameter?
        shape.set_outline_color(&sfml::graphics::Color::CYAN);
        
        // TODO: Input from config?
        shape.set_outline_thickness(1.0);
    }
    
    fn draw_circle(&mut self, _sfml_pos: sfml::system::Vector2f,
                   transform: &Transform,
                   circle: &shapes::Circle) {
        const POINT_COUNT: u32 = 30;
        
        let mut vertex_array = sfml::graphics::VertexArray::default();
        vertex_array.set_primitive_type(sfml::graphics::PrimitiveType::LineStrip);
        
        for i in 0..POINT_COUNT {
            let angle = 2.0f32 * math::PI * i as f32 / POINT_COUNT as f32;
            let p = transform.position + Vec2::new(angle.cos(), angle.sin()) * circle.radius;
            let sfml_vertex = sfml::graphics::Vertex::new(
                sfml_vec2(p, self.pixels_per_unit),
                // TODO: Parameter?
                sfml::graphics::Color::CYAN,
                sfml::system::Vector2f::new(0.0, 0.0)
            );
            vertex_array.append(&sfml_vertex);
        }
        
        let first_vertex = vertex_array[0];
        vertex_array.append(&first_vertex);
        
        self.draw_queue.push(Box::new(vertex_array));
    }
    
    fn draw_polygon(&mut self, _sfml_pos: sfml::system::Vector2f,
                    transform: &Transform,
                    polygon: &shapes::Polygon) {
    
        let mut vertex_array = sfml::graphics::VertexArray::default();
        vertex_array.set_primitive_type(sfml::graphics::PrimitiveType::LineStrip);
        
        for vertex in polygon.vertices.iter() {
            let sfml_vertex = sfml::graphics::Vertex::new(
                sfml_vec2(transform.world_pos(vertex), self.pixels_per_unit),
                // TODO: Parameter?
                sfml::graphics::Color::CYAN,
                sfml::system::Vector2f::new(0.0, 0.0)
            );
            vertex_array.append(&sfml_vertex);
        }
        
        let first_vertex = vertex_array[0];
        vertex_array.append(&first_vertex);
        
        self.draw_queue.push(Box::new(vertex_array));
    }
    
    pub fn draw_body(&mut self, body: &Body) {
        let sfml_pos = sfml_vec2(body.transform.position, self.pixels_per_unit);
        
        match body.shape {
            shapes::Shape::Circle(ref circle) => self.draw_circle(sfml_pos, &body.transform, circle),
            shapes::Shape::Polygon(ref polygon) => self.draw_polygon(sfml_pos, &body.transform, polygon),
        };
    }
    
    pub fn draw_point(&mut self, point: Vec2) {
        let sfml_pos = sfml_vec2(point, self.pixels_per_unit);
        const WIDTH: f32 = 2.0;
        let sfml_offsets = vec![
            sfml::system::Vector2f::new(WIDTH, WIDTH),
            sfml::system::Vector2f::new(WIDTH, -WIDTH),
            sfml::system::Vector2f::new(-WIDTH, -WIDTH),
            sfml::system::Vector2f::new(-WIDTH, WIDTH),
        ];
        
        let mut quad = sfml::graphics::VertexArray::new(sfml::graphics::PrimitiveType::Quads, 4);
        
        // TODO: Color parameter?
        for sfml_offset in sfml_offsets {
            let color = sfml::graphics::Color::RED;
            quad.append(&sfml::graphics::Vertex::new(sfml_pos - sfml_offset, color, sfml_offset));
        }
        
        self.draw_queue.push(Box::new(quad));
    }
    
    pub fn draw_line(&mut self, a: Vec2, b: Vec2) {
        let sfml_a = sfml_vec2(a, self.pixels_per_unit);
        let sfml_b = sfml_vec2(b, self.pixels_per_unit);
        
        let mut line_array = sfml::graphics::VertexArray::new(sfml::graphics::PrimitiveType::Lines, 2);
        
        // TODO: Color parameter?
        let color = sfml::graphics::Color::RED;
        line_array.append(&sfml::graphics::Vertex::new(sfml_a, color,
                                                       sfml::system::Vector2f::new(0.0, 0.0)));
        line_array.append(&sfml::graphics::Vertex::new(sfml_b, color,
                                                       sfml::system::Vector2f::new(10.0, 10.0)));
        
        self.draw_queue.push(Box::new(line_array));
    }
    
    pub fn draw_text(&mut self, text: String, size: u32) {
        self.text_queue.push((text, size));
    }
    
    pub fn process_draw_queue(&mut self, window: &mut sfml::graphics::RenderWindow) {
        // TODO: Move outside fn
        window.set_view(&self.view);
        
        for drawable in self.draw_queue.iter() {
            window.draw((*drawable).deref());
        }
        
        const PADDING: f32 = 8.0;
        
        let width = self.view.size().x;
        let height = self.view.size().y;
        
        let mut text_position = sfml::system::Vector2f::new(-width / 2.0 + PADDING, -height / 2.0 + PADDING);
        
        for text_item in self.text_queue.iter() {
            let (ref text, size) = *text_item;
            let mut text_drawable = sfml::graphics::Text::new(&text, &self.font, size);
            text_drawable.set_position(text_position);
            window.draw(&text_drawable);
            
            text_position.y += size as f32 + PADDING;
        }
        
        self.draw_queue.clear();
        self.text_queue.clear();
    }
}
