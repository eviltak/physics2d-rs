extern crate physics2d;

mod testbed;

use physics2d::*;

struct PolygonsTestbed {
    pub world: World,
}

impl PolygonsTestbed {
    pub fn new() -> PolygonsTestbed {
        PolygonsTestbed {
            world: World::default(),
        }
    }
}

impl testbed::Testbed for PolygonsTestbed {
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
        title: "Polygons".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let mut testbed = PolygonsTestbed::new();
    
    let (w, h) = (5.0f32 * 2.0, 5.0f32 / 2.0);
    
    let mut body = Body::new(shapes::Polygon::new(
        vec![Vec2::ZERO, Vec2::RIGHT * w, Vec2::new(w, h), Vec2::UP * h])
                                 .into_shape(),
                             1.0, Material::new(0.3, 0.3));
    
    let torque = math::PI * 5000.0 * body.inertia;
    body.add_torque(torque);
    
    testbed.world.add_body(body);
    
    testbed::run(testbed, config);
}
