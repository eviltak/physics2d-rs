mod circle_circle;
mod polygon_polygon;
mod circle_polygon;

use crate::math::{Vec2};
use crate::shapes::{Shape, Polygon};
use crate::world::{Body, Transform};
use crate::collision::Contact;

pub trait Collide<T = Self> {
    fn collide(&self, self_body: &Body, other: &T, other_body: &Body) -> Option<Vec<Contact>>;
}

pub fn collide(a: &Body, b: &Body) -> Option<Vec<Contact>> {
    match b.shape {
        Shape::Circle(ref circle) => {
            match_fn_to_shape!(a.shape, collide(a, circle, b))
        },
        Shape::Polygon(ref polygon) => {
            match_fn_to_shape!(a.shape, collide(a, polygon, b))
        },
    }
}

// Utilities for polygon-polygon and circle-polygon intersection tests
struct Face {
    a: Vec2,
    b: Vec2,
    normal: Vec2,
}

impl Face {
    fn new(a: Vec2, b: Vec2, normal: Vec2) -> Face {
        Face {
            a,
            b,
            normal,
        }
    }
    
    fn into_world_face(self, transform: &Transform) -> Face {
        Face::new(
            transform.world_pos(&self.a),
            transform.world_pos(&self.b),
            transform.world_dir(&self.normal)
        )
    }
    
    fn distance(&self, point: &Vec2) -> f32 {
        self.normal.dot(point) - self.normal.dot(&self.a)
    }
}

impl Polygon {
    fn face(&self, index: usize) -> Face {
        Face::new(self.vertices[index], self.vertices[(index + 1) % self.vert_count()], self.normals[index])
    }
}