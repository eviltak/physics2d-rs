extern crate physics2d;

mod testbed;

use physics2d::*;

struct CirclesTestbed {
    pub world: World,
}

impl CirclesTestbed {
    pub fn new() -> CirclesTestbed {
        CirclesTestbed {
            world: World::default(),
        }
    }
}

impl testbed::Testbed for CirclesTestbed {
    fn sfml_loop(&mut self, _input: &testbed::Input, dt: f32) {
        self.world.update(dt);
    }
    
    fn sfml_draw(&mut self, canvas: &mut testbed::Canvas, _dt: f32) {
        for body in self.world.bodies_iter() {
            canvas.draw_body(body);
        }
    }
}

fn main() {
    let config = testbed::Config {
        title: "Circles".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let mut testbed = CirclesTestbed::new();
    
    let body = Body::new(shapes::Circle::new(2.5).into_shape(), 1.0, Material::default());
    
    testbed.world.add_body(body);
    
    testbed::run(testbed, config);
}
