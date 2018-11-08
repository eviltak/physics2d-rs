extern crate physics2d;

mod testbed;

use physics2d::*;

use physics2d::debug::DebugCollision;

struct SpringsTestbed {
    world: World,
    box_id: BodyId,
    circle_id: BodyId,
}

impl SpringsTestbed {
    pub fn new(config: &testbed::Config) -> SpringsTestbed {
        let window_width = config.window_width as f32 / config.pixels_per_unit;
        let window_height = config.window_height as f32 / config.pixels_per_unit;
        
        let box_width = 2.5;
        let box_height = 2.5;
        
        let box_vertices = box_vertices(box_width, box_height);
        let box_poly = shapes::Polygon::new(box_vertices);
        
        let mut box_body = Body::new(box_poly.into_shape(), 10.0, Material::new(1.2, 0.2));
        box_body.transform.position.y = -20.0;
        
        let circle = shapes::Circle::new(1.5);
    
        let mut circle_body = Body::new(circle.into_shape(), 10.0, Material::new(0.8, 0.8));
        circle_body.set_static();
        
        let mut world = World::default();
    
        let box_id = world.add_body(box_body);
        let circle_id = world.add_body(circle_body);
        
        world.add_joint((box_id, circle_id), Joint::Spring(SpringJoint::new(Vec2::ZERO, Vec2::ZERO, 15.0, 0.25, 0.0)));
        
        SpringsTestbed {
            world,
            box_id,
            circle_id,
        }
    }
}

impl testbed::Testbed for SpringsTestbed {
    fn sfml_loop(&mut self, input: &testbed::Input, dt: f32) {
        if input.left_mouse_released {
            let vertices = box_vertices(5.0, 5.0);
            let polygon = shapes::Polygon::new(vertices);
            
            let mut body = Body::new(polygon.into_shape(), 10.0, Material::new(0.3, 0.3));
            
            body.transform.position = input.mouse_position;
            body.transform.set_rotation(0.0);
            
            self.world.add_body(body);
        }
        
        self.world.update(dt);
    }
    
    fn sfml_draw(&mut self, canvas: &mut testbed::Canvas, dt: f32) {
        let bodies = self.world.bodies_iter();
        let body_count = bodies.len();
        
        for body in bodies {
            canvas.draw_body(body);
        }
        
        canvas.draw_text(format!("FPS: {}", 1.0 / dt), 16);
        canvas.draw_text(format!("Body count: {}", body_count), 16);
        
        for contact in self.world.contacts() {
            canvas.draw_point(contact.position);
            canvas.draw_line(contact.position, contact.position + contact.normal * contact.penetration)
        }
        
        let Joint::Spring(ref joint) = &self.world.get_joints((self.box_id, self.circle_id)).unwrap()[0];
        {
            let b = self.world.get_body(&self.circle_id).transform.world_pos(&joint.local_anchor_b);
            let a = self.world.get_body(&self.box_id).transform.world_pos(&joint.local_anchor_a);
            let n = b - a;
            let n = n.normalized();
            canvas.draw_line(a, a + n * joint.distance);
        }
    }
}

fn box_vertices(w: f32, h: f32) -> Vec<Vec2> {
    vec![Vec2::ZERO, Vec2::RIGHT * w, Vec2::new(w, h), Vec2::UP * h]
}

fn main() {
    let config = testbed::Config {
        title: "Springs".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let mut testbed = SpringsTestbed::new(&config);
    
    testbed::run(testbed, config);
}
