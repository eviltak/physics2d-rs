use ::math::{Vec2, Cross};
use world::{Body, BodyId};
use math::clamp;
use world::BodyPair;

use std::hash::{Hash, Hasher};

const PENETRATION_SLOP: f32 = 0.005;
const RESTITUTION_VELOCITY_SLOP: f32 = 0.5;

pub struct Contact {
    pub position: Vec2,
    pub penetration: f32,
    
    pub normal: Vec2,
    tangent: Vec2,
    
    normal_impulse: f32,
    tangent_impulse: f32,
    
    normal_mass: f32,
    tangent_mass: f32,
    
    normal_bias: f32,
    mu: f32,
}

impl Contact {
    pub fn new(position: Vec2, penetration: f32, normal: Vec2) -> Contact {
        Contact {
            position,
            penetration,
            normal,
            tangent: normal.cross(1.0),
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
    pub body_a: BodyId,
    pub body_b: BodyId,
    
    pub contacts: Vec<Contact>,
}

impl PartialEq for Manifold {
    fn eq(&self, other: &Manifold) -> bool {
        self.body_a == other.body_a && self.body_b == other.body_b
    }
}

impl Eq for Manifold {}

impl Hash for Manifold {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.body_a.hash(state);
        self.body_b.hash(state);
    }
}

impl Manifold {
    pub fn new(body_pair: BodyPair, contacts: Vec<Contact>) -> Manifold {
        Manifold {
            body_a: body_pair.0,
            body_b: body_pair.1,
            contacts,
        }
    }
    
    pub fn update_contacts(&mut self, mut new_contacts: Vec<Contact>) {
        for old_contact in self.contacts.iter_mut() {
            const PERSISTENT_DISTANCE: f32 = 0.01;
            // Persist contacts based on proximity
            if let Some(nearest_contact) = new_contacts.iter_mut()
                .find(|c| (c.position - old_contact.position).sqr_len() <= PERSISTENT_DISTANCE) {
                // Persist contact
                nearest_contact.normal_impulse = old_contact.normal_impulse;
                nearest_contact.tangent_impulse = old_contact.tangent_impulse;
            }
        }
        
        self.contacts = new_contacts;
    }
    
    pub fn pre_step(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        if a.inv_mass + b.inv_mass == 0.0 {
            return;
        }
        
        for contact in self.contacts.iter_mut() {
            let r_a = contact.position - a.transform.position;
            let r_b = contact.position - b.transform.position;
    
            let rel_vel = b.velocity - a.velocity + b.angular_vel.cross(&r_b) - a.angular_vel.cross(&r_a);
            let rel_vel_normal = contact.normal.dot(&rel_vel);
    
            let r_a_normal = r_a.dot(&contact.normal);
            let r_a_normal_sqr = r_a_normal * r_a_normal;
            let r_a_tangent_sqr = r_a.sqr_len() - r_a_normal_sqr;
    
            let r_b_normal = r_b.dot(&contact.normal);
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
            let impulse = contact.normal_impulse * contact.normal + contact.tangent_impulse * contact.tangent;
            
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
            let rel_vel_normal = contact.normal.dot(&rel_vel);
            
            // Impulse
            let j = (-rel_vel_normal + contact.normal_bias) * contact.normal_mass;
            
            let old_impulse = contact.normal_impulse;
            contact.normal_impulse = f32::max(0.0, old_impulse + j);
            let j = contact.normal_impulse - old_impulse;
            
            a.add_impulse_at_pos(-contact.normal * j, r_a);
            b.add_impulse_at_pos(contact.normal * j, r_b);
            
            // Friction
            let rel_vel_tangent = contact.tangent.dot(&rel_vel);
            
            let j_t = -rel_vel_tangent * contact.tangent_mass;
            
            let max_friction = contact.mu * contact.normal_impulse;
            let old_impulse = contact.tangent_impulse;
            contact.tangent_impulse = clamp(old_impulse + j_t, -max_friction, max_friction);
            let j_t = contact.tangent_impulse - old_impulse;
            
            a.add_impulse_at_pos(-contact.tangent * j_t, r_a);
            b.add_impulse_at_pos(contact.tangent * j_t, r_b);
            
            // Positional correction
            const BAUMGARTE: f32 = 0.1;
            let correction = f32::max(0.0, BAUMGARTE * (contact.penetration - PENETRATION_SLOP));
            let pos_impulse = contact.normal * correction * contact.normal_mass;
            
            a.transform.position -= pos_impulse * a.inv_mass;
            let rotation = a.transform.rotation() - r_a.cross(pos_impulse) * a.inv_mass;
            a.transform.set_rotation(rotation);
            
            b.transform.position += pos_impulse * b.inv_mass;
            let rotation = b.transform.rotation() + r_b.cross(pos_impulse) * b.inv_mass;
            b.transform.set_rotation(rotation);
        }
    }
}
