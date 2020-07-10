mod spring;

pub use self::spring::SpringJoint;
use crate::constraint::Constraint;
use crate::world::Body;

#[derive(Clone)]
pub enum Joint {
    Spring(SpringJoint),
}

generate_match_mut_fn_macro_for_enum!(Joint::{Spring}; match_fn_to_joint);

impl Constraint for Joint {
    fn initialize_velocity(&mut self, a: &Body, b: &Body, dt: f32) {
        match_fn_to_joint!(*self, initialize_velocity(a, b, dt))
    }
    
    fn warm_start_velocity(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        match_fn_to_joint!(*self, warm_start_velocity(a, b, dt))
    }
    
    fn warm_start_position(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        match_fn_to_joint!(*self, warm_start_position(a, b, dt))
    }
    
    fn solve_velocity(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        match_fn_to_joint!(*self, solve_velocity(a, b, dt))
    }
    
    fn solve_position(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        match_fn_to_joint!(*self, solve_position(a, b, dt))
    }
}
