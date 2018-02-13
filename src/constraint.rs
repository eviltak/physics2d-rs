use world::{BodyMap, BodyId};

pub trait Constraint {
    /// Initialize position-dependent variables to be used in the constraint velocity solver.
    fn initialize_velocity(&mut self, body_map: &BodyMap, id_a: &BodyId, id_b: &BodyId, dt: f32);
    
    /// Apply accumulated velocity impulses, if any.
    fn warm_start_velocity(&mut self, body_map: &mut BodyMap, id_a: &BodyId, id_b: &BodyId, dt: f32);
    
    /// Apply accumulated position impulses, if any.
    fn warm_start_position(&mut self, body_map: &mut BodyMap, id_a: &BodyId, id_b: &BodyId, dt: f32);
    
    /// Solve the velocity constraint.
    fn solve_velocity(&mut self, body_map: &mut BodyMap, id_a: &BodyId, id_b: &BodyId, dt: f32);
    
    /// Solve the position constraint.
    fn solve_position(&mut self, body_map: &mut BodyMap, id_a: &BodyId, id_b: &BodyId, dt: f32);
}
