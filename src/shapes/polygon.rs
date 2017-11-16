use math::*;

pub struct Polygon {
    pub vertices: Vec<Vec2>,
    pub normals: Vec<Vec2>,
}

impl Polygon {
    
    #[inline]
    pub fn vert_count(&self) -> usize {
        self.vertices.len()
    }
}


impl super::Matter for Polygon {
    fn mass_and_inertia(&self, density: f32) -> (f32, f32) {
        let mut area = 0f32;
        let mut density_inertia = 0f32;
        
        for i in 0..self.vert_count() {
            let j: usize = (i + 1) % self.vert_count();
            
            let p1 = self.vertices[i];
            let p2 = self.vertices[j];
            
            let tri_area = 0.5 * p1.cross(p2).abs();
            let tri_inertia = tri_area * (p1.sqr_len() + p2.sqr_len() + p1.dot(p2)) / 6.0;
            
            area += tri_area;
            density_inertia += tri_inertia;
        }
    
        (area * density, density_inertia * density)
    }
}
