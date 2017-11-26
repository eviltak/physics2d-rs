
use ::math::{Vec2};
use ::shapes::{Shape, Circle, Polygon};
use ::world::{Body, Transform};
use super::{Manifold, Contact};

pub trait Collide<T=Self> {
    fn collide(&self, self_body: &Body, other: &T, other_body: &Body) -> Option<Manifold>;
}

impl Collide for Circle {
    fn collide(&self, self_body: &Body, other: &Circle, other_body: &Body) -> Option<Manifold> {
        let r = self.radius + other.radius;
        let normal = other_body.transform.position - self_body.transform.position;
        
        let mut m = Manifold::new();
        
        if normal.sqr_len() > r * r {
            return None;
        }
        
        let distance = normal.len();
        
        m.normal = normal / distance;
        
        let contact = Contact {
            point: m.normal * self.radius + self_body.transform.position,
            penetration: r - distance,
        };
        
        m.contacts.push(contact);
        Some(m)
    }
}

// Utilities for polygon-polygon and circle-polygon intersection tests
struct Face {
    a: Vec2,
    b: Vec2,
    normal: Vec2,
}

impl Face {
    pub fn new(a: Vec2, b: Vec2, normal: Vec2) -> Face {
        Face {
            a, b, normal,
        }
    }
    
    pub fn into_world_face(self, transform: &Transform) -> Face {
        Face::new(
            transform.world_pos(&self.a),
            transform.world_pos(&self.b),
            transform.world_dir(&self.normal)
        )
    }
    
    pub fn distance(&self, point: &Vec2) -> f32 {
        self.normal.dot(point) - self.normal.dot(&self.a)
    }
    
    pub fn clip_points_below(&self, points: &[Vec2; 2]) -> ([Vec2; 2], usize) {
        let d1 = self.distance(&points[0]);
        let d2 = self.distance(&points[1]);
        
        let mut clipped = [Vec2::ZERO; 2];
        let mut clip_count = 0usize;
        
        // If below face, leave as-is
        if d1 <= 0.0 {
            clipped[clip_count] = points[0];
            clip_count += 1
        }
        
        if d2 <= 0.0 {
            clipped[clip_count] = points[1];
            clip_count += 1;
        }
        
        if clip_count >= 2 {
            return (clipped, clip_count);
        }
        
        // If one point above and other below face, clip
        if d1 * d2 < 0.0 {
            let t = d1 / (d1 - d2);
            clipped[clip_count] = points[0] + (points[1] - points[0]) * t;
            clip_count += 1;
        }
        
        (clipped, clip_count)
    }

}

impl Polygon {
    /// Returns the furthest point (vertex) of the polygon along `dir` (which is relative to the polygon).
    fn support_point(&self, dir: &Vec2) -> Vec2 {
        *self.vertices.iter()
            // a.dot(dir) returns the signed distance of the vertex a along dir
             .max_by(|a, b| a.dot(dir).partial_cmp(&b.dot(dir)).unwrap())
             .unwrap()
    }
    
    /// Returns the face of this polygon for which the penetration of the other polygon is least, and
    /// the penetration itself.
    ///
    /// This functions as a Separating Axis Theorem (SAT) test.  For any two convex polygons to intersect,
    /// some portion of one polygon must lie below _all_ the faces of the other polygon. If the returned
    /// penetration is negative, the polygons do not intersect.
    fn least_penetration_face(&self, self_transform: &Transform,
                              other: &Polygon, other_transform: &Transform) -> (usize, f32) {
        use std::f32::INFINITY;
        
        let mut face_index = 0usize;
        let mut min_pen = INFINITY;
    
        for i in 0..self.vert_count() {
            // Vertex and normal describing ith face of self, relative to other
            let normal = other_transform.local_dir(&self_transform.world_dir(&self.normals[i]));
            let vertex = other_transform.local_pos(&self_transform.world_pos(&self.vertices[i]));
            
            // World space vertex of other which is furthest below the face
            let support = other.support_point(&-normal);
            
            // Penetration wrt this face is negative of the distance of support from this face
            let penetration = -normal.dot(&(support - vertex));
            
            if penetration < min_pen {
                min_pen = penetration;
                face_index = i;
            }
        }
        
        (face_index, min_pen)
    }
    
    fn incident_face(&self, ref_face_normal: &Vec2) -> Face {
        use std::f32::INFINITY;
        
        let mut min_dot = INFINITY;
        let mut inc_face_idx = 0usize;
    
        for i in 0..self.vert_count() {
            let dot = self.normals[i].dot(ref_face_normal);
            
            if dot < min_dot {
                min_dot = dot;
                inc_face_idx = i;
            }
        }
        
        self.face(inc_face_idx)
    }
    
    fn face(&self, index: usize) -> Face {
        Face::new(self.vertices[index], self.vertices[(index + 1) % self.vert_count()], self.normals[index])
    }
}


impl Collide for Polygon {
    fn collide(&self, self_body: &Body, other: &Polygon, other_body: &Body) -> Option<Manifold> {
        let self_transform = &self_body.transform;
        let other_transform = &other_body.transform;
        
        let (self_face_idx, self_pen) = self.least_penetration_face(self_transform, other, other_transform);
        
        if self_pen <= 0.0 {
            return None;
        }
        
        let (other_face_idx, other_pen)= other.least_penetration_face(other_transform,
                                                                  self,self_transform);
        
        if other_pen <= 0.0 {
            return None;
        }
        
        let (mut ref_poly, mut ref_body, mut ref_face_idx): (&Polygon, &Body, usize);
    
        let (mut inc_poly, mut inc_body): (&Polygon, &Body);
    
        // TODO: Weighted check to ensure uniform direction when penetrations equal, i.e. normal incidence
        if self_pen < other_pen {
            ref_poly = self;
            ref_body = self_body;
            ref_face_idx = self_face_idx;
            
            inc_poly = other;
            inc_body = other_body;
        } else {
            ref_poly = other;
            ref_body = other_body;
            ref_face_idx = other_face_idx;
            
            inc_poly = self;
            inc_body = self_body;
        }
        
        let (ref_transform, inc_transform) = (&ref_body.transform, &inc_body.transform);
        
        let ref_face = ref_poly.face(ref_face_idx).into_world_face(ref_transform);
        
        let inc_face = inc_poly
            .incident_face(
                &inc_transform.local_dir(
                    &ref_transform.world_dir(&ref_face.normal)
                )
            )
            .into_world_face(inc_transform);
        
        let mut inc_points = [inc_face.a, inc_face.b];
        
        let side_faces_idx = [(ref_face_idx + 1) % ref_poly.vert_count(),
            (ref_face_idx + ref_poly.vert_count() - 1) % ref_poly.vert_count()];
        
        // Clip by side faces of ref face (not ref face itself)
        for side_face_idx in side_faces_idx.iter() {
            // If less than 2 points clipped, floating point error, return
            let (clipped, clip_count) =
                ref_poly.face(*side_face_idx)
                    .into_world_face(ref_transform)
                    .clip_points_below(&inc_points);
            
            if clip_count < 2 {
                return None;
            }
            
            inc_points = clipped;
        }
        
        let mut m = Manifold::new();
        
        for p in inc_points.iter() {
            let d = ref_face.distance(p);
    
            // Only keep points behind the reference face
            if d > 0.0 {
                continue;
            }
            
            let contact = Contact {
                point: *p,
                penetration: -d,
            };
            
            m.contacts.push(contact)
        }
        
        Some(m)
    }
}

impl Collide<Polygon> for Circle {
    fn collide(&self, self_body: &Body, other: &Polygon, other_body: &Body) -> Option<Manifold> {
        unimplemented!()
    }
}

impl Collide<Circle> for Polygon {
    fn collide(&self, self_body: &Body, other: &Circle, other_body: &Body) -> Option<Manifold> {
        unimplemented!()
    }
}

pub fn collide(a: &Body, b: &Body) -> Option<Manifold> {
    match b.shape {
        Shape::Circle(ref circle) => {
            match_fn_to_shape!(a.shape, collide(a, circle, b))
        },
        Shape::Polygon(ref polygon) => {
            match_fn_to_shape!(a.shape, collide(a, polygon, b))
        },
    }
}
