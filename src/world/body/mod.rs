mod pair;

pub use self::pair::BodyPair;

use crate::math::{Vec2, Cross};
use crate::shapes::{Shape, Matter};
use crate::world::Transform;
use crate::math::Bounds;


use crate::collision::broad_phase;

/// The identifier used for looking up a `Body` in a `World`.
///
/// An identifier is used to avoid the need for an `Rc` - the `Body` can be looked up on demand
/// through the `World`.
pub type BodyId = usize;

#[derive(Default)]
pub struct Material {
    pub restitution: f32,
    pub friction: f32,
}

impl Material {
    pub fn new(friction: f32, restitution: f32) -> Material {
        Material {
            restitution,
            friction,
        }
    }
}

pub struct Body {
    pub id: BodyId,
    pub(crate) proxy_id: broad_phase::ProxyId,
    
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
    pub bounds: Bounds,
    
    pub material: Material,
}

impl Body {
    pub fn new(shape: Shape, density: f32, material: Material) -> Body {
        let transform = Transform::new(Vec2::ZERO, 0.0);
        let aabb = shape.bounds(Some(&transform));
        let (mass, inertia) = shape.mass_and_inertia(density);
        
        let inv_mass = if mass != 0.0 { 1.0 / mass } else { 0.0f32 };
        let inv_inertia = if inertia != 0.0 { 1.0 / inertia } else { 0.0f32 };
        
        Body {
            id: BodyId::default(),
            proxy_id: broad_phase::ProxyId::default(),
            transform,
            velocity: Vec2::ZERO,
            angular_vel: 0.0,
            force: Vec2::ZERO,
            torque: 0.0,
            
            mass,
            inertia,
            inv_mass,
            inv_inertia,
            
            shape,
            bounds: aabb,
            material,
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
    
    pub(crate) fn update(&mut self, _dt: f32) {
        self.bounds = self.shape.bounds(Some(&self.transform));
    }
    
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
