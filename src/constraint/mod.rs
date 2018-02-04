use world::Body;

pub mod contact;

pub trait Solver<M> {
    fn initialize_constraints(&self, manifold: &mut M, a: &Body, b: &Body, dt: f32);
    fn warm_start(&self, manifold: &mut M, a: &mut Body, b: &mut Body, dt: f32);
    fn solve(&self, manifold: &mut M, a: &mut Body, b: &mut Body, dt: f32);
}