extern crate sfml;
extern crate physics2d;

mod testbed;

use physics2d::*;
use physics2d::math::*;

use physics2d::debug::DebugCollision;

struct CollisionsTestbed {
    world: world::World,
    should_stop: bool,
}

impl CollisionsTestbed {
    pub fn new() -> CollisionsTestbed {
        let circle_a = shapes::Circle::new(5.0);
        let circle_b = shapes::Circle::new(3.0);
        
        let mut a = world::Body::new(circle_a.into_shape(), 10.0);
        let mut b = world::Body::new(circle_b.into_shape(), 10.0);
        
        a.position = Vec2::UP * 25.0;
        
        let mut world = world::World::new();
        
        world.add_body(a);
        world.add_body(b);
        
        CollisionsTestbed {
            world,
            should_stop: false,
        }
    }
}

impl testbed::Testbed for CollisionsTestbed {
    fn sfml_loop(&mut self, dt: f32) {
        if self.should_stop {
            return;
        }
        
        let f = Vec2::UP * 7.0 * self.world.bodies[1].mass;
        self.world.bodies[1].add_force(f);
        
        self.world.update(dt);
        
        self.should_stop = self.world.contact_points().len() > 0;
    }
    
    fn sfml_draw(&mut self, canvas: &mut testbed::Canvas, dt: f32) {
        for body in &self.world.bodies {
            canvas.draw_body(body);
        }
        
        if !self.should_stop {
            return;
        }
    
        for contact in self.world.contact_points().iter() {
            canvas.draw_point(*contact);
            //println!("{:?}", *contact);
        }
    }
}


fn main() {
    let config = testbed::Config {
        title: "Collisions".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let mut testbed = CollisionsTestbed::new();
    
    testbed::run(testbed, config);
}
