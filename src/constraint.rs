use world::Body;

pub trait Constraint {
    /// Initialize position-dependent variables to be used in the constraint velocity solver.
    fn initialize_velocity(&mut self, a: &Body, b: &Body, dt: f32);
    
    /// Apply accumulated velocity impulses, if any.
    fn warm_start_velocity(&mut self, a: &mut Body, b: &mut Body, dt: f32);
    
    /// Apply accumulated position impulses, if any.
    fn warm_start_position(&mut self, a: &mut Body, b: &mut Body, dt: f32);
    
    /// Solve the velocity constraint.
    fn solve_velocity(&mut self, a: &mut Body, b: &mut Body, dt: f32);
    
    /// Solve the position constraint.
    fn solve_position(&mut self, a: &mut Body, b: &mut Body, dt: f32);
}
/*
impl<T: Constraint + ?Sized> Constraint for Box<T> {
    fn initialize_velocity(&mut self, a: &Body, b: &Body, dt: f32) {
        self.as_mut().initialize_velocity(a, b, dt)
    }
    
    fn warm_start_velocity(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        self.as_mut().warm_start_velocity(a, b, dt)
    }
    
    fn warm_start_position(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        self.as_mut().warm_start_position(a, b, dt)
    }
    
    fn solve_velocity(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        self.as_mut().warm_start_position(a, b, dt);
    }
    
    fn solve_position(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        self.as_mut().warm_start_position(a, b, dt);
    }
}*/
