extern crate sfml;
extern crate physics2d;

mod testbed;

use physics2d::*;

use physics2d::debug::DebugCollision;

struct PolygonCollisionsTestbed {
    world: World,
    should_stop: bool,
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
        
        let mut a = Body::new(poly_a.into_shape(), 10.0);
        let mut b = Body::new(poly_b.into_shape(), 10.0);
        
        a.transform.position = Vec2::UP * 4.0 + Vec2::RIGHT * 1.0;
        a.transform.set_rotation(math::PI / 5.0);
        
        let mut world = World::new();
        
        world.add_body(a);
        world.add_body(b);
        
        PolygonCollisionsTestbed {
            world,
            should_stop: false,
        }
    }
}

impl testbed::Testbed for PolygonCollisionsTestbed {
    fn sfml_loop(&mut self, dt: f32) {
        if self.should_stop {
            return;
        }
        
        let f = Vec2::UP * 7.0 * self.world.bodies[1].mass;
        self.world.bodies[1].add_force(f);
        
        self.world.update(dt * 1.0);
        
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
    
    let mut testbed = PolygonCollisionsTestbed::new();
    
    testbed::run(testbed, config);
}