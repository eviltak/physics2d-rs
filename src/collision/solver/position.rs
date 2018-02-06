use math::Cross;
use collision::Contact;
use constraint::ConstraintSolver;
use world::Body;

const PENETRATION_SLOP: f32 = 0.005;
const BAUMGARTE: f32 = 0.1;

struct PositionConstraint {
    contact: Contact,
    normal_mass: f32,
}

impl PositionConstraint {
    fn new(contact: Contact) -> PositionConstraint {
        PositionConstraint {
            contact,
            normal_mass: 0.0,
        }
    }
}

pub struct PositionConstraintManifold {
    constraints: Vec<PositionConstraint>,
}

impl PositionConstraintManifold {
    pub fn new(contacts: &Vec<Contact>) -> PositionConstraintManifold {
        PositionConstraintManifold {
            constraints: contacts.iter().map(|contact| PositionConstraint::new(*contact)).collect()
        }
    }
}

impl ConstraintSolver for PositionConstraintManifold {
    fn initialize_constraints(&mut self, a: &Body, b: &Body, _dt: f32) {
        if a.inv_mass + b.inv_mass == 0.0 {
            return;
        }
        
        for constraint in self.constraints.iter_mut() {
            let contact = &constraint.contact;
            let r_a = contact.position - a.transform.position;
            let r_b = contact.position - b.transform.position;
            
            let r_a_tangent = r_a.dot(&contact.tangent);
            let r_a_tangent_sqr = r_a_tangent * r_a_tangent;
            
            let r_b_tangent = r_b.dot(&contact.tangent);
            let r_b_tangent_sqr = r_b_tangent * r_b_tangent;
            
            let inv_mass_sum = a.inv_mass + b.inv_mass;
            
            let inv_normal_impulse_factor = inv_mass_sum + r_a_tangent_sqr * a.inv_inertia + r_b_tangent_sqr * b.inv_inertia;
            
            constraint.normal_mass = 1.0 / inv_normal_impulse_factor;
        }
    }
    
    fn warm_start(&mut self, _a: &mut Body, _b: &mut Body, _dt: f32) {}
    
    fn solve_constraints(&mut self, a: &mut Body, b: &mut Body, _dt: f32) {
        if a.inv_mass + b.inv_mass == 0.0 {
            return;
        }
        
        for constraint in self.constraints.iter_mut() {
            let contact = &constraint.contact;
            let r_a = contact.position - a.transform.position;
            let r_b = contact.position - b.transform.position;
            
            let correction = f32::max(0.0, BAUMGARTE * (contact.penetration - PENETRATION_SLOP));
            let pos_impulse = constraint.normal_mass * contact.normal * correction;
            
            a.transform.position -= pos_impulse * a.inv_mass;
            
            let rotation = a.transform.rotation() - r_a.cross(pos_impulse) * a.inv_mass;
            a.transform.set_rotation(rotation);
            
            b.transform.position += pos_impulse * b.inv_mass;
            
            let rotation = b.transform.rotation() + r_b.cross(pos_impulse) * b.inv_mass;
            b.transform.set_rotation(rotation);
        }
    }
}