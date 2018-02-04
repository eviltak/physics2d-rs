use ::math::{Vec2, Cross};
use ::world::{Body};
use math::clamp;

const PENETRATION_SLOP: f32 = 0.005;
const RESTITUTION_VELOCITY_SLOP: f32 = 0.5;

pub struct Contact {
    pub position: Vec2,
    pub penetration: f32,
    
    normal_impulse: f32,
    tangent_impulse: f32,
    
    normal_mass: f32,
    tangent_mass: f32,
    
    normal_bias: f32,
    mu: f32,
}

impl Contact {
    pub fn new(position: Vec2, penetration: f32) -> Contact {
        Contact {
            position,
            penetration,
            normal_impulse: 0.0,
            tangent_impulse: 0.0,
            normal_mass: 0.0,
            tangent_mass: 0.0,
            normal_bias: 0.0,
            mu: 0.0,
        }
    }
}

pub struct Manifold {
    pub normal: Vec2,
    pub tangent: Vec2,
    pub contacts: Vec<Contact>,
}

impl Manifold {
    pub fn new(normal: Vec2, contacts: Vec<Contact>) -> Manifold {
        Manifold {
            normal,
            tangent: normal.cross(1.0),
            contacts,
        }
    }
    
    pub fn persist_contacts(&mut self, old: &Manifold) {
        for contact in self.contacts.iter_mut() {
            const PERSISTENT_DISTANCE: f32 = 0.01;
            
            if let Some(nearest_contact) = old.contacts.iter()
                .find(|c| (c.position - contact.position).sqr_len() <= PERSISTENT_DISTANCE) {
                contact.normal_impulse = nearest_contact.normal_impulse;
                contact.tangent_impulse = nearest_contact.tangent_impulse;
            }
        }
    }
    
    pub fn pre_step(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        if a.inv_mass + b.inv_mass == 0.0 {
            return;
        }
        
        for contact in self.contacts.iter_mut() {
            let r_a = contact.position - a.transform.position;
            let r_b = contact.position - b.transform.position;
    
            let rel_vel = b.velocity - a.velocity + b.angular_vel.cross(&r_b) - a.angular_vel.cross(&r_a);
            let rel_vel_normal = self.normal.dot(&rel_vel);
    
            let r_a_normal = r_a.dot(&self.normal);
            let r_a_normal_sqr = r_a_normal * r_a_normal;
            let r_a_tangent_sqr = r_a.sqr_len() - r_a_normal_sqr;
    
            let r_b_normal = r_b.dot(&self.normal);
            let r_b_normal_sqr = r_b_normal * r_b_normal;
            let r_b_tangent_sqr = r_b.sqr_len() - r_b_normal_sqr;
    
            let inv_mass_sum = a.inv_mass + b.inv_mass;
    
            let inv_normal_impulse_factor = inv_mass_sum + r_a_tangent_sqr * a.inv_inertia + r_b_tangent_sqr * b.inv_inertia;
    
            let inv_tangent_impulse_factor = inv_mass_sum + r_a_normal_sqr * a.inv_inertia + r_b_normal_sqr * b.inv_inertia;
            
            contact.normal_mass = 1.0 / inv_normal_impulse_factor;
            contact.tangent_mass = 1.0 / inv_tangent_impulse_factor;
            
            // TODO: Change
            let e = 0.5f32;
            let res_bias = -e * f32::max(0.0, rel_vel_normal - RESTITUTION_VELOCITY_SLOP);
    
            contact.normal_bias = res_bias;
            
            // TODO: Change
            let mu = 0.3f32;
            contact.mu = mu;
            
            // Warm start
            let impulse = contact.normal_impulse * self.normal + contact.tangent_impulse * self.tangent;
            
            a.add_impulse_at_pos(-impulse, r_a);
            b.add_impulse_at_pos(impulse, r_b);
        }
    }
    
    pub fn resolve(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        if a.inv_mass + b.inv_mass == 0.0 {
            return;
        }
        
        for contact in self.contacts.iter_mut() {
            let r_a = contact.position - a.transform.position;
            let r_b = contact.position - b.transform.position;
            
            let rel_vel = b.velocity - a.velocity + b.angular_vel.cross(&r_b) - a.angular_vel.cross(&r_a);
            let rel_vel_normal = self.normal.dot(&rel_vel);
            
            // Impulse
            let j = (-rel_vel_normal + contact.normal_bias) * contact.normal_mass;
            
            let old_impulse = contact.normal_impulse;
            contact.normal_impulse = f32::max(0.0, old_impulse + j);
            let j = contact.normal_impulse - old_impulse;
            
            a.add_impulse_at_pos(-self.normal * j, r_a);
            b.add_impulse_at_pos(self.normal * j, r_b);
            
            // Friction
            let rel_vel_tangent = self.tangent.dot(&rel_vel);
            
            let j_t = -rel_vel_tangent * contact.tangent_mass;
            
            let max_friction = contact.mu * contact.normal_impulse;
            let old_impulse = contact.tangent_impulse;
            contact.tangent_impulse = clamp(old_impulse + j_t, -max_friction, max_friction);
            let j_t = contact.tangent_impulse - old_impulse;
            
            a.add_impulse_at_pos(-self.tangent * j_t, r_a);
            b.add_impulse_at_pos(self.tangent * j_t, r_b);
            
            // Positional correction
            const BAUMGARTE: f32 = 0.1;
            let correction = f32::max(0.0, BAUMGARTE * (contact.penetration - PENETRATION_SLOP));
            let pos_impulse = self.normal * correction * contact.normal_mass;
            
            a.transform.position -= pos_impulse * a.inv_mass;
            let rotation = a.transform.rotation() - r_a.cross(pos_impulse) * a.inv_mass;
            a.transform.set_rotation(rotation);
            
            b.transform.position += pos_impulse * b.inv_mass;
            let rotation = b.transform.rotation() + r_b.cross(pos_impulse) * b.inv_mass;
            b.transform.set_rotation(rotation);
        }
    }
}
