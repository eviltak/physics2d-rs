use super::{Collide, Face};
use crate::math::{Vec2};
use crate::shapes::{Polygon};
use crate::world::{Body, Transform};
use crate::collision::{Contact};

impl Face {
    fn clip_points_below(&self, points: &[Vec2; 2]) -> ([Vec2; 2], usize) {
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
    /// Returns the furthest point of the shape along `dir` (which is relative to this shape).
    fn support_point(&self, dir: &Vec2) -> Vec2 {
        *self.vertices.iter()
            // a.dot(dir) returns the signed distance of the vertex a along dir
             .max_by(|a, b| a.dot(dir).partial_cmp(&b.dot(dir)).unwrap())
             .unwrap()
    }
    
    /// Returns the face of this polygon for which the penetration of the other polygon is least, and
    /// the penetration itself.
    ///
    /// This functions as a Separating Axis Theorem (SAT) test.  For a shape to intersect with a convex polygon,
    /// some portion of that shape must lie below _all_ the faces of the polygon. If the furthest point of the shape
    /// (support point) in a direction opposite to the normal of a face lies _above_ the face, the whole shape lies
    /// above that face and its penetration for that face is negative.
    ///
    /// If the returned penetration is negative, the polygons do not intersect.
    fn least_penetration_face(&self, self_transform: &Transform,
                              other: &Polygon, other_transform: &Transform) -> (usize, f32) {
        use std::f32::INFINITY;
        
        let mut face_index = 0usize;
        let mut min_pen = INFINITY;
        
        for i in 0..self.vert_count() {
            // Vertex and normal describing ith face of self, relative to other
            let normal = other_transform.local_dir(&self_transform.world_dir(&self.normals[i]));
            let vertex = other_transform.local_pos(&self_transform.world_pos(&self.vertices[i]));
            
            // Vertex of other which is furthest below the face
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
}

impl Collide for Polygon {
    fn collide(&self, self_body: &Body, other: &Polygon, other_body: &Body) -> Option<Vec<Contact>> {
        let self_transform = &self_body.transform;
        let other_transform = &other_body.transform;
        
        let (self_face_idx, self_pen) = self.least_penetration_face(self_transform, other, other_transform);
        
        if self_pen <= 0.0 {
            return None;
        }
        
        let (other_face_idx, other_pen) = other.least_penetration_face(other_transform,
                                                                       self, self_transform);
        
        if other_pen <= 0.0 {
            return None;
        }
        
        let (ref_poly, ref_body, ref_face_idx): (&Polygon, &Body, usize);
        
        let (inc_poly, inc_body): (&Polygon, &Body);
        
        let self_is_ref_poly = other_pen >= 0.95 * self_pen + 0.01 * other_pen;
        
        if self_is_ref_poly {
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
            .incident_face(&inc_transform.local_dir(&ref_face.normal))
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
        
        let normal = if self_is_ref_poly { ref_face.normal } else { -ref_face.normal };
        
        // For each incident point, create a contact
        let contacts = inc_points.iter().filter_map(|inc_point| {
            let d = ref_face.distance(inc_point);
            // Only keep points behind the reference face
            if d > 0.0 {
                None
            } else {
                Some(Contact::new(*inc_point, -d, normal))
            }
        }).collect();
        
        Some(contacts)
    }
}