
use ::shapes::{Shape, Circle, Polygon};
use ::world::Body;
use super::{Manifold, Contact};

pub trait Collide<T=Self> {
    fn collide(&self, self_body: &Body, other: &T, other_body: &Body) -> Option<Manifold>;
}

impl Collide for Circle {
    fn collide(&self, self_body: &Body, other: &Circle, other_body: &Body) -> Option<Manifold> {
        let r = self.radius + other.radius;
        let normal = other_body.position - self_body.position;
        
        let mut m = Manifold::new();
        
        if normal.sqr_len() > r * r {
            return None;
        }
        
        let distance = normal.len();
        
        m.normal = normal / distance;
        
        let contact = Contact {
            point: m.normal * self.radius + self_body.position,
            penetration: r - distance,
        };
        
        m.contacts.push(contact);
        Some(m)
    }
}

impl Collide<Polygon> for Circle {
    fn collide(&self, self_body: &Body, other: &Polygon, other_body: &Body) -> Option<Manifold> {
        unimplemented!()
    }
}

impl Collide for Polygon {
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
