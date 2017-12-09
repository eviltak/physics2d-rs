extern crate sfml;
extern crate physics2d;

mod testbed;

use physics2d::*;

use physics2d::debug::DebugCollision;

struct CircleCollisionsTestbed {
    world: World,
    should_stop: bool,
}

impl CircleCollisionsTestbed {
    pub fn new() -> CircleCollisionsTestbed {
        let circle_a = shapes::Circle::new(5.0);
        let circle_b = shapes::Circle::new(3.0);
        
        let mut a = Body::new(circle_a.into_shape(), 10.0);
        let mut b = Body::new(circle_b.into_shape(), 10.0);
        
        a.transform.position = Vec2::UP * 25.0;
        
        let mut world = World::new();
        
        world.add_body(a);
        world.add_body(b);
        
        CircleCollisionsTestbed {
            world,
            should_stop: false,
        }
    }
}

impl testbed::Testbed for CircleCollisionsTestbed {
    fn sfml_loop(&mut self, input: &testbed::Input, dt: f32) {
        if !self.should_stop {
            let f = Vec2::UP * 7.0 * self.world.bodies[1].mass;
            self.world.bodies[1].add_force(f);
        }
        
        self.world.update(dt);
        
        self.should_stop = self.world.contact_points().len() > 0;
    }
    
    fn sfml_draw(&mut self, canvas: &mut testbed::Canvas, dt: f32) {
        for body in &self.world.bodies {
            canvas.draw_body(body);
        }
    
        for contact in self.world.contact_points().iter() {
            canvas.draw_point(*contact);
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
    
    let mut testbed = CircleCollisionsTestbed::new();
    
    testbed::run(testbed, config);
}
