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

}