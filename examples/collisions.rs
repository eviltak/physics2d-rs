extern crate physics2d;

mod testbed;

use physics2d::*;

use physics2d::debug::DebugCollision;

struct CollisionsTestbed {
    world: World,
}

impl CollisionsTestbed {
    pub fn new(config: &testbed::Config) -> CollisionsTestbed {
        let window_width = config.window_width as f32 / config.pixels_per_unit;
        let window_height = config.window_height as f32 / config.pixels_per_unit;
        
        let ground_width = window_width / 2.0;
        let ground_height = window_height / 10.0;
        
        let ground_vertices = box_vertices(ground_width, ground_height);
        let ground_poly = shapes::Polygon::new(ground_vertices);
        
        let mut ground = Body::new(ground_poly.into_shape(), 10.0, Material::new(1.2, 0.2));
        ground.transform.position.y = -window_height / 2.0 + ground_height / 2.0 + 0.1;
        
        ground.set_static();
        
        let mut world = World::default();
        
        let obs_circle = shapes::Circle::new(5.0);
        
        let mut obstacle = Body::new(obs_circle.into_shape(), 0.0, Material::new(0.8, 0.8));
        obstacle.set_static();
        
        world.add_body(ground);
        world.add_body(obstacle);
        
        CollisionsTestbed {
            world,
        }
    }
}

impl testbed::Testbed for CollisionsTestbed {
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
        let bodies = self.world.bodies_iter();
        let body_count = self.world.body_count();
    
        for body in bodies {
            canvas.draw_body(body);
        }
        
        canvas.draw_text(format!("FPS: {}", 1.0 / dt), 16);
        canvas.draw_text(format!("Body count: {}", body_count), 16);
        
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
        title: "Collisions".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let testbed = CollisionsTestbed::new(&config);
    
    testbed::run(testbed, config);
}
