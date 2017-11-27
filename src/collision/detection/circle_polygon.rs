use super::{Collide};
use ::math::{Vec2, clamp01};
use ::shapes::{Circle, Polygon};
use ::world::{Body, Transform};
use ::collision::{Contact, Manifold};

impl Circle {
    /// Returns the face for which the penetration of the circle is least, the penetration and the corresponding
    /// support point on the circle.
    fn least_penetration_support_point(&self, self_transform: &Transform,
                                       other: &Polygon, other_transform: &Transform) -> (usize, f32, Vec2) {
        use std::f32::INFINITY;
    
        let mut face_index = 0usize;
        let mut min_pen = INFINITY;
        let mut min_support = Vec2::ZERO;
        
        let self_local_pos = other_transform.local_pos(&self_transform.position);
    
        for i in 0..other.vert_count() {
            // Vertex and normal describing ith face of other
            let normal = other.normals[i];
            let vertex = other.vertices[i];
            
            // Point on self furthest below the face
            let support = self_local_pos - normal * self.radius;
        
            // Penetration wrt this face is negative of the distance of support from this face
            let penetration = -normal.dot(&(support - vertex));
        
            if penetration < min_pen {
                min_pen = penetration;
                face_index = i;
                min_support = support;
            }
        }
    
        (face_index, min_pen, min_support)
    }
}

impl Collide<Polygon> for Circle {
    fn collide(&self, self_body: &Body, other: &Polygon, other_body: &Body) -> Option<Manifold> {
        let self_transform = &self_body.transform;
        let other_transform = &other_body.transform;
        
        let (face_idx, penetration, support) =
            self.least_penetration_support_point(self_transform, other, other_transform);
        
        if penetration < 0.0 {
            return None;
        }
        
        let face = other.face(face_idx);
        
        // The contact point is the clamped projection of support on face
        let face_vec = face.b - face.a;
        
        let t = (support - face.a).dot(&face_vec) / face_vec.sqr_len();
        
        let corner_contact = t < 0.0 || t > 1.0;
        
        let t = clamp01(t);
        
        let contact_point = face.a + face_vec * t;
        let contact_point = other_transform.world_pos(&contact_point);
        
        if (contact_point - self_transform.position).sqr_len() > self.radius * self.radius {
            return None;
        }
        
        let contact = Contact {
            point: contact_point,
            penetration,
        };
        
        let mut manifold = Manifold::new();
        
        manifold.normal =
            if corner_contact {
                (contact_point - self_transform.position).normalized()
            } else {
                other_transform.world_dir(&face.normal)
            };
        
        manifold.contacts.push(contact);
        
        Some(manifold)
    }
}

impl Collide<Circle> for Polygon {
    fn collide(&self, self_body: &Body, other: &Circle, other_body: &Body) -> Option<Manifold> {
        unimplemented!()
    }
}