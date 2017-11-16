use math::*;

pub struct Polygon {
    pub vertices: Vec<Vec2>,
    pub normals: Vec<Vec2>,
}

impl super::Matter for Polygon {
    fn mass_and_inertia(&self, density: f32) -> (f32, f32) {
        unimplemented!()
    }
}
