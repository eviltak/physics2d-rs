pub struct Circle {
    pub radius: f32,
}

impl super::Matter for Circle {
    fn mass_and_inertia(&self, density: f32) -> (f32, f32) {
        unimplemented!()
    }
}
