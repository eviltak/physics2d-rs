use math::{Vec2, Cross};
use world::{Body, BodyId};
use math::clamp;
use world::{BodyPair, BodyMap};
use constraint::ConstraintSolver;
use collision::Contact;

const PENETRATION_SLOP: f32 = 0.005;
const RESTITUTION_VELOCITY_SLOP: f32 = 0.5;

struct VelocityConstraint {
    contact: Contact,
    
    normal_impulse: f32,
    tangent_impulse: f32,
    
    normal_mass: f32,
    tangent_mass: f32,
    
    normal_bias: f32,
    mu: f32,
}

impl VelocityConstraint {
    pub fn new(contact: Contact) -> VelocityConstraint {
        VelocityConstraint {
            contact,
            
            normal_impulse: 0.0,
            tangent_impulse: 0.0,
            normal_mass: 0.0,
            tangent_mass: 0.0,
            normal_bias: 0.0,
            mu: 0.0,
        }
    }
}

pub struct VelocityConstraintManifold {
    pub body_a: BodyId,
    pub body_b: BodyId,
    
    constraints: Vec<VelocityConstraint>,
}

impl VelocityConstraintManifold {
    pub fn new(body_pair: BodyPair, contacts: &Vec<Contact>) -> VelocityConstraintManifold {
        VelocityConstraintManifold {
            body_a: body_pair.0,
            body_b: body_pair.1,
            constraints: contacts.iter().map(|contact| VelocityConstraint::new(*contact)).collect(),
        }
    }
    
    pub fn update_constraints(&mut self, new_contacts: &Vec<Contact>) {
        let mut new_constraints: Vec<VelocityConstraint> =
            new_contacts.iter().map(|contact| VelocityConstraint::new(*contact)).collect();
        
        for old_constraint in self.constraints.iter_mut() {
            const PERSISTENT_DISTANCE: f32 = 0.01;
            
            // Persist constraints based on proximity
            if let Some(near_constraint) =
            new_constraints.iter_mut()
                           .find(|c| (c.contact.position - old_constraint.contact.position).sqr_len() <= PERSISTENT_DISTANCE) {
                // Persist contact
                near_constraint.normal_impulse = old_constraint.normal_impulse;
                near_constraint.tangent_impulse = old_constraint.tangent_impulse;
            }
        }
        
        self.constraints = new_constraints;
    }
}

impl ConstraintSolver for VelocityConstraintManifold {
    fn initialize_constraints(&mut self, a: &Body, b: &Body, dt: f32) {
        if a.inv_mass + b.inv_mass == 0.0 {
            return;
        }
        
        for constraint in self.constraints.iter_mut() {
            let contact = &constraint.contact;
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
            
            constraint.normal_mass = 1.0 / inv_normal_impulse_factor;
            constraint.tangent_mass = 1.0 / inv_tangent_impulse_factor;
            
            // TODO: Change
            let e = 0.5f32;
            let res_bias = -e * f32::max(0.0, rel_vel_normal - RESTITUTION_VELOCITY_SLOP);
            
            constraint.normal_bias = res_bias;
            
            // TODO: Change
            let mu = 0.3f32;
            constraint.mu = mu;
        }
    }
    
    fn warm_start(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        if a.inv_mass + b.inv_mass == 0.0 {
            return;
        }
        
        for constraint in self.constraints.iter() {
            let contact = &constraint.contact;
            let r_a = contact.position - a.transform.position;
            let r_b = contact.position - b.transform.position;
            
            let impulse = constraint.normal_impulse * contact.normal + constraint.tangent_impulse * contact.tangent;
            
            a.add_impulse_at_pos(-impulse, r_a);
            b.add_impulse_at_pos(impulse, r_b);
        }
    }
    
    fn solve_constraints(&mut self, a: &mut Body, b: &mut Body, dt: f32) {
        // TODO: Remove this via precondition
        if a.inv_mass + b.inv_mass == 0.0 {
            return;
        }
        
        for constraint in self.constraints.iter_mut() {
            let contact = &constraint.contact;
            let r_a = contact.position - a.transform.position;
            let r_b = contact.position - b.transform.position;
            
            let rel_vel = b.velocity - a.velocity + b.angular_vel.cross(&r_b) - a.angular_vel.cross(&r_a);
            let rel_vel_normal = contact.normal.dot(&rel_vel);
            
            // Impulse
            let j = (-rel_vel_normal + constraint.normal_bias) * constraint.normal_mass;
            
            let old_impulse = constraint.normal_impulse;
            constraint.normal_impulse = f32::max(0.0, old_impulse + j);
            
            let j = constraint.normal_impulse - old_impulse;
            
            a.add_impulse_at_pos(-contact.normal * j, r_a);
            b.add_impulse_at_pos(contact.normal * j, r_b);
            
            // Friction
            let rel_vel_tangent = contact.tangent.dot(&rel_vel);
            
            let j_t = -rel_vel_tangent * constraint.tangent_mass;
            
            let max_friction = constraint.mu * constraint.normal_impulse;
            
            let old_impulse = constraint.tangent_impulse;
            constraint.tangent_impulse = clamp(old_impulse + j_t, -max_friction, max_friction);
            
            let j_t = constraint.tangent_impulse - old_impulse;
            
            a.add_impulse_at_pos(-contact.tangent * j_t, r_a);
            b.add_impulse_at_pos(contact.tangent * j_t, r_b);
        }
    }
}

