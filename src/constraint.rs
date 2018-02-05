use world::Body;

pub trait ConstraintSolver {
    fn initialize_constraints(&mut self, a: &Body, b: &Body, dt: f32);
    fn warm_start(&mut self, a: &mut Body, b: &mut Body, dt: f32);
    fn solve_constraints(&mut self, a: &mut Body, b: &mut Body, dt: f32);
}