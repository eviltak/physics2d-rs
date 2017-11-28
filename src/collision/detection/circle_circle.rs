use super::Collide;
use ::math::{Vec2};
use ::shapes::{Circle};
use ::world::{Body, Transform};
use ::collision::{Contact, Manifold};

impl Collide for Circle {
    fn collide(&self, self_body: &Body, other: &Circle, other_body: &Body) -> Option<Manifold> {
        let r = self.radius + other.radius;
        let normal = other_body.transform.position - self_body.transform.position;
        
        if normal.sqr_len() > r * r {
            return None;
        }
        
        let distance = normal.len();
        let normal = normal / distance;
        
        let contact = Contact {
            point: normal * self.radius + self_body.transform.position,
            penetration: r - distance,
        };
        
        Some(Manifold::new(normal, vec![contact]))
    }
}