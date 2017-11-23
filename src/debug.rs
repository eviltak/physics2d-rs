use ::world::*;
use ::math::*;

impl World {
    pub fn contact_points(&self) -> Vec<Vec2> {
        self.manifolds.iter()
            .flat_map(
                |ref m|
                    m.contacts.iter()
                     .map(|ref c| c.point))
            .collect()
    }
}