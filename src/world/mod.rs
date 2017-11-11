mod bodies;

pub use self::bodies::Body;

pub struct World {
    pub bodies: Vec<Body>,
}

impl World {
    pub fn new() -> World {
        World {
            bodies: vec!()
        }
    }
    
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
    
    pub fn update(&mut self, dt: f32) {
        for body in &mut self.bodies {
            body.update(dt);
        }
    }
}
