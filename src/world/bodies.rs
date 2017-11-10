use math::*;
use ::shapes::Shape;

pub struct Body {
    pub position: Vec2,
    pub rotation: f32,
    
    pub velocity: Vec2,
    pub angular_vel: f32,
    
    force: Vec2,
    torque: f32,
    
    pub mass: f32,
    pub inertia: f32,
    
    pub shape: Shape,
}

impl Body {
    pub fn update(&mut self, dt: f32) {
        self.velocity += self.force / self.mass * dt;
        self.position += self.velocity * dt;
        
        self.angular_vel += self.torque / self.inertia * dt;
        self.rotation += self.angular_vel * dt;
        
        self.force = Vec2::ZERO;
        self.torque = 0.0;
    }
}