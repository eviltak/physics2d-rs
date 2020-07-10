use super::Collide;
use crate::shapes::{Circle};
use crate::world::{Body};
use crate::collision::{Contact};

impl Collide for Circle {
    fn collide(&self, self_body: &Body, other: &Circle, other_body: &Body) -> Option<Vec<Contact>> {
        let r = self.radius + other.radius;
        let normal = other_body.transform.position - self_body.transform.position;
        
        if normal.sqr_len() > r * r {
            return None;
        }
        
        let distance = normal.len();
        let normal = normal / distance;
        let contact_position = normal * self.radius + self_body.transform.position;
        
        let contact = Contact::new(contact_position, r - distance, normal);
        
        Some(vec![contact])
    }
}