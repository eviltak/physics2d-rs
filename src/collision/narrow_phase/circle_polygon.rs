use super::{Collide};
use crate::math::{Vec2, clamp01};
use crate::shapes::{Circle, Polygon};
use crate::world::{Body, Transform};
use crate::collision::{Contact};

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
    fn collide(&self, self_body: &Body, other: &Polygon, other_body: &Body) -> Option<Vec<Contact>> {
        let self_transform = &self_body.transform;
        let other_transform = &other_body.transform;
        
        let (face_idx, mut penetration, support) =
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
        
        let rel_contact_point = contact_point - self_transform.position;
        let contact_dist_sqr = rel_contact_point.sqr_len();
        
        if contact_dist_sqr > self.radius * self.radius {
            return None;
        }
        
        if corner_contact {
            penetration = self.radius - contact_dist_sqr.sqrt()
        }
        
        let normal = if corner_contact {
            rel_contact_point / (self.radius - penetration)
        } else {
            -other_transform.world_dir(&face.normal)
        };
        
        let contact = Contact::new(contact_point, penetration, normal);
        
        Some(vec![contact])
    }
}

impl Collide<Circle> for Polygon {
    fn collide(&self, self_body: &Body, other: &Circle, other_body: &Body) -> Option<Vec<Contact>> {
        let contacts = other.collide(other_body, self, self_body);
        
        if contacts.is_none() {
            contacts
        } else {
            // Normal must always point from self to other
            let mut contacts = contacts.unwrap();
            
            for contact in contacts.iter_mut() {
                contact.normal = -contact.normal;
            }
            
            Some(contacts)
        }
    }
}