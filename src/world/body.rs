use math::{Vec2, Cross};
use ::shapes::{Shape, Matter};
use ::world::Transform;

pub type BodyId = usize;

pub struct Body {
    pub transform: Transform,
    
    pub velocity: Vec2,
    pub angular_vel: f32,
    
    force: Vec2,
    torque: f32,
    
    pub mass: f32,
    pub inertia: f32,
    
    pub inv_mass: f32,
    pub inv_inertia: f32,
    
    pub shape: Shape,
}

impl Body {
    pub fn new(shape: Shape, density: f32) -> Body {
        let (mass, inertia) = shape.mass_and_inertia(density);
        
        let inv_mass = if mass != 0.0 { 1.0 / mass } else { 0.0f32 };
        let inv_inertia = if inertia != 0.0 { 1.0 / inertia } else { 0.0f32 };
        
        Body {
            transform: Transform::new(Vec2::ZERO, 0.0),
            velocity: Vec2::ZERO,
            angular_vel: 0.0,
            force: Vec2::ZERO,
            torque: 0.0,
            
            mass,
            inertia,
            inv_mass,
            inv_inertia,
            
            shape,
        }
    }
    
    pub(crate) fn integrate_force(&mut self, dt: f32) {
        // TODO: Make configurable
        const GRAVITY: Vec2 = Vec2 { x: 0.0, y: -9.8 };
        
        if self.is_static() {
            return;
        }
        
        self.velocity += (GRAVITY + self.force * self.inv_mass) * dt;
        self.angular_vel += self.torque * self.inv_inertia * dt;
        
        self.force = Vec2::ZERO;
        self.torque = 0.0;
    }
    
    pub(crate) fn integrate_velocity(&mut self, dt: f32) {
        if self.is_static() {
            return;
        }
        
        self.transform.position += self.velocity * dt;
        
        let new_rotation = self.transform.rotation() + self.angular_vel * dt;
        
        self.transform.set_rotation(new_rotation);
    }
    
    pub(crate) fn update(&mut self, dt: f32) {}
    
    pub fn set_static(&mut self) {
        self.inv_inertia = 0.0;
        self.inertia = 0.0;
        self.mass = 0.0;
        self.inv_mass = 0.0;
    }
    
    pub fn is_static(&self) -> bool {
        self.inv_mass == 0.0 && self.inv_inertia == 0.0
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
    
    pub fn add_impulse_at_pos(&mut self, impulse: Vec2, pos: Vec2) {
        self.velocity += impulse * self.inv_mass;
        self.angular_vel += pos.cross(impulse) * self.inv_inertia;
    }
}