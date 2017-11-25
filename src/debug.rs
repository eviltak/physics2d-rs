use ::world::*;
use ::math::*;

pub trait DebugCollision {
    fn contact_points(&self) -> Vec<Vec2>;
}

impl DebugCollision for World {
    fn contact_points(&self) -> Vec<Vec2> {
        self.manifolds.iter()
            .flat_map(
                |ref m|
                    m.contacts.iter()
                     .map(|ref c| c.point))
            .collect()
    }
}