extern crate physics2d;

mod testbed;

use physics2d::*;

use physics2d::debug::DebugCollision;

struct CircleCollisionsTestbed {
    world: World,
    should_stop: bool,
    body_b: BodyId,
}

impl CircleCollisionsTestbed {
    pub fn new() -> CircleCollisionsTestbed {
        let circle_a = shapes::Circle::new(5.0);
        let circle_b = shapes::Circle::new(3.0);
        
        let mut a = Body::new(circle_a.into_shape(), 10.0, Material::new(0.3, 0.3));
        let b = Body::new(circle_b.into_shape(), 10.0, Material::new(0.3, 0.3));
        
        a.transform.position = Vec2::UP * 25.0;
        
        let mut world = World::default();
        
        world.add_body(a);
        let body_b = world.add_body(b);
        
        CircleCollisionsTestbed {
            world,
            should_stop: false,
            body_b,
        }
    }
}

impl testbed::Testbed for CircleCollisionsTestbed {
    fn sfml_loop(&mut self, _input: &testbed::Input, dt: f32) {
        if !self.should_stop {
            let body = self.world.get_body_mut(self.body_b);
            let f = Vec2::UP * 7.0 * body.mass;
            body.add_force(f);
        }
        
        self.world.update(dt);
        
        self.should_stop = self.world.contacts().len() > 0;
    }
    
    fn sfml_draw(&mut self, canvas: &mut testbed::Canvas, _dt: f32) {
        for body in self.world.bodies_iter() {
            canvas.draw_body(body);
        }
        
        for contact in self.world.contacts().iter() {
            canvas.draw_point(contact.position);
        }
    }
}


fn main() {
    let config = testbed::Config {
        title: "Circle Collisions".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let testbed = CircleCollisionsTestbed::new();
    
    testbed::run(testbed, config);
}
