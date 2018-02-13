extern crate physics2d;

mod testbed;

use physics2d::*;

use physics2d::debug::DebugCollision;

struct PyramidTestbed {
    world: World,
}

impl PyramidTestbed {
    pub fn new(config: &testbed::Config) -> PyramidTestbed {
        let window_width = config.window_width as f32 / config.pixels_per_unit;
        let window_height = config.window_height as f32 / config.pixels_per_unit;
        
        let mut world = World::default();
        
        let ground_width = window_width;
        let ground_height = 1.0;
        let ground_poly = shapes::Polygon::new(box_vertices(ground_width, ground_height));
        
        let mut ground = Body::new(ground_poly.into_shape(), 10.0, Material::new(0.4, 0.4));
        ground.transform.position.y = -window_height / 2.0 + ground_height / 2.0;
        
        ground.set_static();
        world.add_body(ground);
        
        let wall_poly = shapes::Polygon::new(box_vertices(1.0, window_height - ground_height));
        
        let mut left_wall = Body::new(wall_poly.into_shape(), 10.0, Material::new(0.4, 0.4));
        left_wall.transform.position = Vec2::new(-window_width / 2.0 + 1.0, 0.6);
        left_wall.set_static();
        world.add_body(left_wall);
        
        let wall_poly = shapes::Polygon::new(box_vertices(1.0, window_height - ground_height));
        
        let mut right_wall = Body::new(wall_poly.into_shape(), 10.0, Material::new(0.4, 0.4));
        right_wall.transform.position = Vec2::new(window_width / 2.0 - 1.0, 0.6);
        right_wall.set_static();
        world.add_body(right_wall);
        
        const WIDTH: f32 = 3.0;
        let square = shapes::Polygon::new(box_vertices(WIDTH, WIDTH));
        
        //  Pyramid (taken directly from Box2D Lite)
        let mut x = Vec2::new(-window_width / 2.0 + WIDTH * 0.5 + 1.0, -window_height / 2.0 + WIDTH * 0.5 + ground_height);
        
        const N: u32 = 20;
        
        for i in 0..N {
            let mut y = x;
            
            for j in i..N {
                let mut body = Body::new(square.clone().into_shape(), 10.0, Material::new(0.3, 0.3));
                body.transform.position = y;
                world.add_body(body);
                y += Vec2::RIGHT * WIDTH * 1.125;
            }
            
            // x += Vec2(0.5625f, 1.125f);
            x += Vec2::new(0.5625, 1.0) * WIDTH;
        }
        
        PyramidTestbed {
            world,
        }
    }
}

impl testbed::Testbed for PyramidTestbed {
    fn sfml_loop(&mut self, input: &testbed::Input, dt: f32) {
        if input.left_mouse_released {
            let vertices = box_vertices(5.0, 5.0);
            let polygon = shapes::Polygon::new(vertices);
            
            let mut body = Body::new(polygon.into_shape(), 10.0, Material::new(0.3, 0.3));
            
            body.transform.position = input.mouse_position;
            body.transform.set_rotation(0.2);
            
            self.world.add_body(body);
        }
        
        self.world.update(dt);
    }
    
    fn sfml_draw(&mut self, canvas: &mut testbed::Canvas, dt: f32) {
        let bodies = self.world.bodies();
        for body in bodies.iter() {
            canvas.draw_body(body);
        }
        
        canvas.draw_text(format!("FPS: {}", 1.0 / dt), 16);
        canvas.draw_text(format!("Body count: {}", bodies.len()), 16);
        
        for contact in self.world.contacts() {
            canvas.draw_point(contact.position);
            canvas.draw_line(contact.position, contact.position + contact.normal * contact.penetration)
        }
    }
}

fn box_vertices(w: f32, h: f32) -> Vec<Vec2> {
    vec![Vec2::ZERO, Vec2::RIGHT * w, Vec2::new(w, h), Vec2::UP * h]
}

fn main() {
    let config = testbed::Config {
        title: "Pyramid".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let mut testbed = PyramidTestbed::new(&config);
    
    testbed::run(testbed, config);
}
