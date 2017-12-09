use ::math::{Vec2, Cross};
use ::world::{Body};

pub struct Contact {
    pub point: Vec2,
    pub penetration: f32,
}

pub struct Manifold {
    pub normal: Vec2,
    pub contacts: Vec<Contact>,
}

impl Manifold {
    pub fn new(normal: Vec2, contacts: Vec<Contact>) -> Manifold {
        Manifold {
            normal,
            contacts,
        }
    }
    
    pub fn resolve(&self, a: &mut Body, b: &mut Body) {
        // TODO: Provisions for 0 mass
        for contact in self.contacts.iter() {
            let r_a = contact.point - a.transform.position;
            let r_b = contact.point - b.transform.position;
            
            let rel_vel = b.velocity - a.velocity + b.angular_vel.cross(&r_b) - a.angular_vel.cross(&r_a);
            let rel_vel_normal = self.normal.dot(&rel_vel);
            
            // Tangent is perpendicular to normal
            let r_a_normal = r_a.dot(&self.normal);
            let r_a_perp_sqr = r_a.sqr_len() - r_a_normal * r_a_normal;
    
            let r_b_normal = r_b.dot(&self.normal);
            let r_b_perp_sqr = r_b.sqr_len() - r_b_normal * r_b_normal;
            
            let inv_mass_sum = a.inv_mass + b.inv_mass;
    
            let inv_normal_impulse_factor = inv_mass_sum + r_a_perp_sqr * a.inv_inertia + r_b_perp_sqr * b.inv_inertia;
            // Impulse
            // TODO: Change
            let e = 0.5;
            let j = -(1.0 + e) * rel_vel_normal / inv_normal_impulse_factor;
            
            a.add_impulse_at_pos(-self.normal * j, r_a);
            b.add_impulse_at_pos(self.normal * j, r_b);
        }
    }
}
