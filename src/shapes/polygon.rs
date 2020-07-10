use crate::math::{Vec2, Cross, INFINITY};
use crate::world::Transform;
use crate::math::Bounds;

#[derive(Clone)]
pub struct Polygon {
    pub vertices: Vec<Vec2>,
    pub normals: Vec<Vec2>,
}

impl Polygon {
    pub fn new(mut vertices: Vec<Vec2>) -> Polygon {
        let mut area = 0f32;
        
        for i in 0..vertices.len() {
            let j: usize = (i + 1) % vertices.len();
            
            let p1 = vertices[i];
            let p2 = vertices[j];
            
            area += 0.5 * p1.cross(p2);
        }
        
        let mut centroid = Vec2::ZERO;
        
        for i in 0..vertices.len() {
            let j: usize = (i + 1) % vertices.len();
            
            let p1 = vertices[i];
            let p2 = vertices[j];
            
            centroid += (p1 + p2) * p1.cross(p2) / (6.0 * area);
        }
        
        for vertex in vertices.iter_mut() {
            *vertex -= centroid;
        }
        
        let mut normals: Vec<Vec2> = Vec::with_capacity(vertices.len());
        
        for i in 0..vertices.len() {
            let j: usize = (i + 1) % vertices.len();
            
            let edge: Vec2 = vertices[j] - vertices[i];
            
            normals.push(edge.normalized().cross(1.0));
        }
        
        Polygon {
            vertices,
            normals,
        }
    }
    
    #[inline]
    pub fn vert_count(&self) -> usize {
        self.vertices.len()
    }
    
    pub fn into_shape(self) -> super::Shape {
        super::Shape::Polygon(self)
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
            let tri_inertia = tri_area * (p1.sqr_len() + p2.sqr_len() + p1.dot(&p2)) / 6.0;
            
            area += tri_area;
            density_inertia += tri_inertia;
        }
        
        (area * density, density_inertia * density)
    }
    
    fn bounds(&self, transform: Option<&Transform>) -> Bounds {
        let mut min = Vec2::ONE * INFINITY;
        let mut max = -Vec2::ONE * INFINITY;
        
        for vertex in self.vertices.iter() {
            let vertex = if let Some(t) = transform { t.world_pos(&vertex) } else { *vertex };
            min = min.min(&vertex);
            max = max.max(&vertex);
        }
        
        Bounds::new(min, max)
    }
}
