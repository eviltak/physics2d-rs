use ::math::{Vec2, Cross};
use ::world::{Body};

const PENETRATION_SLOP: f32 = 0.005;
const RESTITUTION_VELOCITY_SLOP: f32 = 0.5;

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
    
    pub fn resolve(&self, a: &mut Body, b: &mut Body, dt: f32) {
        if a.inv_mass + b.inv_mass == 0.0 {
            return;
        }
        
        for contact in self.contacts.iter() {
            let r_a = contact.point - a.transform.position;
            let r_b = contact.point - b.transform.position;
            
            let rel_vel = b.velocity - a.velocity + b.angular_vel.cross(&r_b) - a.angular_vel.cross(&r_a);
            let rel_vel_normal = self.normal.dot(&rel_vel);
            
            // Tangent is perpendicular to normal
            let r_a_normal = r_a.dot(&self.normal);
            let r_a_normal_sqr = r_a_normal * r_a_normal;
            let r_a_tangent_sqr = r_a.sqr_len() - r_a_normal_sqr;
            
            let r_b_normal = r_b.dot(&self.normal);
            let r_b_normal_sqr = r_b_normal * r_b_normal;
            let r_b_tangent_sqr = r_b.sqr_len() - r_b_normal_sqr;
            
            let inv_mass_sum = a.inv_mass + b.inv_mass;
            
            let inv_normal_impulse_factor = inv_mass_sum + r_a_tangent_sqr * a.inv_inertia + r_b_tangent_sqr * b.inv_inertia;
            
            // Impulse
            // TODO: Change
            let e = 0.5f32;
            let res_bias = e * f32::max(0.0, rel_vel_normal - RESTITUTION_VELOCITY_SLOP);
            
            let bias = res_bias;
            let j = -(rel_vel_normal + bias) / inv_normal_impulse_factor;
            
            a.add_impulse_at_pos(-self.normal * j, r_a);
            b.add_impulse_at_pos(self.normal * j, r_b);
            
            // Component of relative velocity perpendicular to normal
            let tangent: Vec2 = (rel_vel - self.normal * rel_vel_normal).normalized();
            let rel_vel_tangent = tangent.dot(&rel_vel);
            
            let inv_tangent_impulse_factor = inv_mass_sum + r_a_normal_sqr * a.inv_inertia + r_b_normal_sqr * b.inv_inertia;
            
            let friction = rel_vel_tangent / inv_tangent_impulse_factor;
            
            // TODO: Change
            let static_friction = 0.3;
            let dynamic_friction = 0.2;
            
            // j > 0; j == normal contact impulse
            let friction = if friction.abs() > static_friction * j {
                dynamic_friction * j
            } else {
                friction
            };
            // Friction always acts against relative velocity
            let friction = -friction;
            
            a.add_impulse_at_pos(-tangent * friction, r_a);
            b.add_impulse_at_pos(tangent * friction, r_b);
            
            // Positional correction
            const BAUMGARTE: f32 = 0.1;
            let correction = f32::max(0.0, BAUMGARTE * (contact.penetration - PENETRATION_SLOP));
            let pos_impulse = self.normal * correction / inv_normal_impulse_factor;
            
            a.transform.position -= pos_impulse * a.inv_mass;
            let rotation = a.transform.rotation() - r_a.cross(pos_impulse) * a.inv_mass;
            a.transform.set_rotation(rotation);
            
            b.transform.position += pos_impulse * b.inv_mass;
            let rotation = b.transform.rotation() + r_b.cross(pos_impulse) * b.inv_mass;
            b.transform.set_rotation(rotation);
        }
    }
}
