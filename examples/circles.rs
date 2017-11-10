extern crate sfml;
extern crate physics2d;

mod testbed;

use physics2d::*;

struct CirclesTestbed {
    pub world: world::World,
}

impl CirclesTestbed {
    pub fn new() -> CirclesTestbed {
        CirclesTestbed {
            world: world::World::new(),
        }
    }
}

impl testbed::Testbed for CirclesTestbed {
    fn sfml_loop(&mut self, dt: f32) {
        self.world.update(dt);
    }
    
    fn sfml_draw(&mut self, canvas: &mut testbed::Canvas, dt: f32) {
        for body in &self.world.bodies {
            canvas.draw_body(body);
        }
    }
}

fn main() {
    let config = testbed::Config {
        title: "Main".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let mut testbed = CirclesTestbed::new();
    
    testbed::run(testbed, config);
}
