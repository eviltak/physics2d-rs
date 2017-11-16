use math::*;
use ::shapes::{Shape, Matter};

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
    pub fn new(shape: Shape, density: f32) -> Body {
        let (mass, inertia) = shape.mass_and_inertia(density);
        
        Body {
            position: Vec2::ZERO,
            rotation: 0.0,
            velocity: Vec2::ZERO,
            angular_vel: 0.0,
            force: Vec2::ZERO,
            torque: 0.0,
            
            mass,
            inertia,
            shape,
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        // TODO: Make configurable
        const GRAVITY: Vec2 = Vec2{ x: 0.0, y: -9.8 };
        
        self.velocity += (GRAVITY + self.force / self.mass) * dt;
        self.position += self.velocity * dt;
        
        self.angular_vel += self.torque / self.inertia * dt;
        self.rotation += self.angular_vel * dt;
        
        self.force = Vec2::ZERO;
        self.torque = 0.0;
    }
    
    pub fn add_force(&mut self, force: Vec2) {
        self.force += force;
    }
    
    pub fn add_torque(&mut self, torque: f32) {
        self.torque += torque;
    }
    
    pub fn add_force_at_pos(&mut self, force: Vec2, pos: Vec2) {
        self.add_force(force);
        self.add_torque(pos.cross(force));
    }
}