extern crate physics2d;

mod testbed;

use physics2d::*;

use physics2d::debug::DebugCollision;

struct PolygonCollisionsTestbed {
    world: World,
    should_stop: bool,
    body_b: BodyId,
}

impl PolygonCollisionsTestbed {
    pub fn new() -> PolygonCollisionsTestbed {
        fn vert_box(w: f32, h: f32) -> Vec<Vec2> {
            vec![Vec2::ZERO, Vec2::RIGHT * w, Vec2::new(w, h), Vec2::UP * h]
        }
        
        let vert_a = vert_box(5.0, 10.0);
        let poly_a = shapes::Polygon::new(vert_a);
        
        let vert_b = vert_box(10.0, 5.0);
        let poly_b = shapes::Polygon::new(vert_b);
        
        let mut a = Body::new(poly_a.into_shape(), 10.0, Material::default());
        let b = Body::new(poly_b.into_shape(), 10.0, Material::default());
        
        a.transform.position = Vec2::UP * 16.0 + Vec2::RIGHT * 1.0;
        a.transform.set_rotation(math::PI / 5.0);
        
        let mut world = World::default();
        
        world.add_body(a);
        let body_b = world.add_body(b);
        
        PolygonCollisionsTestbed {
            world,
            should_stop: false,
            body_b,
        }
    }
}

impl testbed::Testbed for PolygonCollisionsTestbed {
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
        title: "Polygon Collisions".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let testbed = PolygonCollisionsTestbed::new();
    
    testbed::run(testbed, config);
}
