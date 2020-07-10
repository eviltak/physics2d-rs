use crate::collision::Contact;
use crate::constraint::Constraint;
use crate::world::Body;
use crate::math::{clamp, Cross};

const PENETRATION_SLOP: f32 = 0.005;
const BAUMGARTE: f32 = 0.1;

const RESTITUTION_VELOCITY_SLOP: f32 = 0.5;

pub struct ContactConstraint {
    pub(crate) contact: Contact,
    
    normal_impulse: f32,
    tangent_impulse: f32,
    
    normal_mass: f32,
    tangent_mass: f32,
    
    restitution: f32,
    friction_coefficient: f32,
}

impl ContactConstraint {
    pub fn new(contact: Contact) -> ContactConstraint {
        ContactConstraint {
            contact,
            
            normal_impulse: 0.0,
            tangent_impulse: 0.0,
            normal_mass: 0.0,
            tangent_mass: 0.0,
            restitution: 0.0,
            friction_coefficient: 0.0,
        }
    }
    
    pub fn with_contacts(new_contacts: &Vec<Contact>) -> Vec<ContactConstraint> {
        new_contacts.iter().map(|contact| ContactConstraint::new(*contact)).collect()
    }
    
    pub fn with_persistent_contacts(old_constraints: &Vec<ContactConstraint>,
                                    new_contacts: &Vec<Contact>) -> Vec<ContactConstraint> {
        let mut new_constraints: Vec<ContactConstraint> =
            new_contacts.iter().map(|contact| ContactConstraint::new(*contact)).collect();
        
        for old_constraint in old_constraints.iter() {
            const PERSISTENT_DISTANCE: f32 = 0.01;
        
            // Persist constraints based on proximity
            if let Some(near_constraint) = new_constraints.iter_mut().find(|c| {
                (c.contact.position - old_constraint.contact.position).sqr_len() <= PERSISTENT_DISTANCE
            }) {
                // Persist constraint data
                near_constraint.normal_impulse = old_constraint.normal_impulse;
                near_constraint.tangent_impulse = old_constraint.tangent_impulse;
            }
        }
        
        new_constraints
    }
}

impl Constraint for ContactConstraint {
    fn initialize_velocity(&mut self, a: &Body, b: &Body, _dt: f32) {
        let contact = &self.contact;
        let r_a = contact.position - a.transform.position;
        let r_b = contact.position - b.transform.position;
        
        let rel_vel = b.velocity - a.velocity + b.angular_vel.cross(&r_b) - a.angular_vel.cross(&r_a);
        let _rel_vel_normal = contact.normal.dot(&rel_vel);
        
        let r_a_normal = r_a.dot(&contact.normal);
        let r_a_normal_sqr = r_a_normal * r_a_normal;
        let r_a_tangent_sqr = r_a.sqr_len() - r_a_normal_sqr;
        
        let r_b_normal = r_b.dot(&contact.normal);
        let r_b_normal_sqr = r_b_normal * r_b_normal;
        let r_b_tangent_sqr = r_b.sqr_len() - r_b_normal_sqr;
        
        let inv_mass_sum = a.inv_mass + b.inv_mass;
        
        let inv_normal_impulse_factor = inv_mass_sum + r_a_tangent_sqr * a.inv_inertia + r_b_tangent_sqr * b.inv_inertia;
        
        let inv_tangent_impulse_factor = inv_mass_sum + r_a_normal_sqr * a.inv_inertia + r_b_normal_sqr * b.inv_inertia;
        
        self.normal_mass = 1.0 / inv_normal_impulse_factor;
        self.tangent_mass = 1.0 / inv_tangent_impulse_factor;
        
        // Arithmetic mean
        self.restitution = 0.5 * (a.material.restitution + b.material.restitution);
        
        // Geometric mean
        self.friction_coefficient = (a.material.friction * b.material.friction).sqrt();
    }
    
    fn warm_start_velocity(&mut self, a: &mut Body, b: &mut Body, _dt: f32) {
        let contact = &self.contact;
        let r_a = contact.position - a.transform.position;
        let r_b = contact.position - b.transform.position;
        
        let impulse = self.normal_impulse * contact.normal + self.tangent_impulse * contact.tangent;
        
        a.add_impulse_at_pos(-impulse, r_a);
        b.add_impulse_at_pos(impulse, r_b);
    }
    
    fn warm_start_position(&mut self, _a: &mut Body, _b: &mut Body, _dt: f32) {}
    
    fn solve_velocity(&mut self, a: &mut Body, b: &mut Body, _dt: f32) {
        let contact = &self.contact;
        let r_a = contact.position - a.transform.position;
        let r_b = contact.position - b.transform.position;
        
        // Solve tangent constraints first because normal constraints (non-penetration) are more important
        // Friction
        let rel_vel = b.velocity - a.velocity + b.angular_vel.cross(&r_b) - a.angular_vel.cross(&r_a);
        
        let rel_vel_tangent = contact.tangent.dot(&rel_vel);
        
        let j_t = -rel_vel_tangent * self.tangent_mass;
        
        let max_friction = self.friction_coefficient * self.normal_impulse;
        
        let old_impulse = self.tangent_impulse;
        self.tangent_impulse = clamp(old_impulse + j_t, -max_friction, max_friction);
        
        let j_t = self.tangent_impulse - old_impulse;
        
        a.add_impulse_at_pos(-contact.tangent * j_t, r_a);
        b.add_impulse_at_pos(contact.tangent * j_t, r_b);
        
        // Impulse
        let rel_vel = b.velocity - a.velocity + b.angular_vel.cross(&r_b) - a.angular_vel.cross(&r_a);
        
        let rel_vel_normal = contact.normal.dot(&rel_vel);
        
        let res_bias = -self.restitution * f32::max(0.0, rel_vel_normal - RESTITUTION_VELOCITY_SLOP);
        let bias = res_bias;
        
        let j = (-rel_vel_normal + bias) * self.normal_mass;
        
        let old_impulse = self.normal_impulse;
        self.normal_impulse = f32::max(0.0, old_impulse + j);
        
        let j = self.normal_impulse - old_impulse;
        
        a.add_impulse_at_pos(-contact.normal * j, r_a);
        b.add_impulse_at_pos(contact.normal * j, r_b);
    }
    
    fn solve_position(&mut self, a: &mut Body, b: &mut Body, _dt: f32) {
        let contact = &self.contact;
        let r_a = contact.position - a.transform.position;
        let r_b = contact.position - b.transform.position;
        
        let r_a_tangent = r_a.dot(&contact.tangent);
        let r_a_tangent_sqr = r_a_tangent * r_a_tangent;
        
        let r_b_tangent = r_b.dot(&contact.tangent);
        let r_b_tangent_sqr = r_b_tangent * r_b_tangent;
        
        let inv_mass_sum = a.inv_mass + b.inv_mass;
        
        let inv_normal_impulse_factor = inv_mass_sum + r_a_tangent_sqr * a.inv_inertia + r_b_tangent_sqr * b.inv_inertia;
        let normal_mass = 1.0 / inv_normal_impulse_factor;
        
        let correction = f32::max(0.0, BAUMGARTE * (contact.penetration - PENETRATION_SLOP));
        let pos_impulse = normal_mass * contact.normal * correction;
        
        a.transform.position -= pos_impulse * a.inv_mass;
        
        let rotation = a.transform.rotation() - r_a.cross(pos_impulse) * a.inv_inertia;
        a.transform.set_rotation(rotation);
        
        b.transform.position += pos_impulse * b.inv_mass;
        
        let rotation = b.transform.rotation() + r_b.cross(pos_impulse) * b.inv_inertia;
        b.transform.set_rotation(rotation);
    }
}
