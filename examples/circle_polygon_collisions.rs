extern crate sfml;
extern crate physics2d;

mod testbed;

use physics2d::*;

use physics2d::debug::DebugCollision;

struct CirclePolygonCollisionsTestbed {
    world: World,
    should_stop: bool,
}

impl CirclePolygonCollisionsTestbed {
    pub fn new() -> CirclePolygonCollisionsTestbed {
        fn vert_box(w: f32, h: f32) -> Vec<Vec2> {
            vec![Vec2::ZERO, Vec2::RIGHT * w, Vec2::new(w, h), Vec2::UP * h]
        }
        
        let vert_a = vert_box(5.0, 10.0);
        let poly = shapes::Polygon::new(vert_a);
        
        let circle = shapes::Circle::new(5.0);
        
        let mut a = Body::new(circle.into_shape(), 10.0);
        let mut b = Body::new(poly.into_shape(), 10.0);
        
        b.transform.position = Vec2::UP * 18.0 + Vec2::RIGHT * 1.0;
        b.transform.set_rotation(math::PI / 5.0);
        
        let mut world = World::new();
        
        world.add_body(a);
        world.add_body(b);
        
        CirclePolygonCollisionsTestbed {
            world,
            should_stop: false,
        }
    }
}

impl testbed::Testbed for CirclePolygonCollisionsTestbed {
    fn sfml_loop(&mut self, dt: f32) {
        if self.should_stop {
            return;
        }
        
        let f = Vec2::UP * 7.0 * self.world.bodies[0].mass;
        self.world.bodies[0].add_force(f);
        
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
        title: "Circle-Polygon Collisions".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let mut testbed = CirclePolygonCollisionsTestbed::new();
    
    testbed::run(testbed, config);
}