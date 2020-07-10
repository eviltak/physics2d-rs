use crate::constraint::Constraint;
use crate::world::Body;
use crate::math::{Vec2, PI, Cross};
use crate::joint::Joint;

const SPRING_DISPLACEMENT_SLOP: f32 = 0.05;

#[derive(Clone)]
pub struct SpringJoint {
    pub local_anchor_a: Vec2,
    pub local_anchor_b: Vec2,
    
    pub distance: f32,
    pub frequency: f32,
    pub damping: f32,
    
    impulse: f32,
    mass: f32,
    
    r_a: Vec2,
    r_b: Vec2,
    
    x: f32,
    normal: Vec2,
    softness: f32,
    k: f32,
    beta: f32,
    damp_coeff: f32,
    omega: f32,
}

impl SpringJoint {
    pub fn new(local_anchor_a: Vec2, local_anchor_b: Vec2,
               distance: f32, frequency: f32, damping: f32) -> SpringJoint {
        SpringJoint {
            local_anchor_a,
            local_anchor_b,
            distance,
            frequency,
            damping,
            
            impulse: 0.0,
            mass: 0.0,
            r_a: Vec2::ZERO,
            r_b: Vec2::ZERO,
            x: 0.0,
            normal: Vec2::ZERO,
            softness: 0.0,
            k: 0.0,
            beta: 0.0,
            damp_coeff: 0.0,
            omega: 0.0
        }
    }
    
    pub fn into_joint(self) -> Joint {
        Joint::Spring(self)
    }
}

impl Constraint for SpringJoint {
    fn initialize_velocity(&mut self, a: &Body, b: &Body, dt: f32) {
        self.r_a = a.transform.world_dir(&self.local_anchor_a);
        self.r_b = b.transform.world_dir(&self.local_anchor_b);
        
        self.omega = 2.0 * PI * self.frequency;
        
        let displacement = b.transform.position + self.r_b - a.transform.position - self.r_a;
        let length = displacement.len();
        
        self.x = length - self.distance;
        
        self.normal = if length != 0.0 {
            displacement / length
        } else {
            // If the anchor points are together, push upward
            Vec2::UP
        };
    
        let inv_mass_sum = a.inv_mass + b.inv_mass;
        let r_a_normal_perp = self.r_a.cross(self.normal);
        let r_b_normal_perp = self.r_b.cross(self.normal);
    
        let inv_reduced_mass = inv_mass_sum + r_a_normal_perp * r_a_normal_perp * a.inv_inertia +
            r_b_normal_perp * r_b_normal_perp * b.inv_inertia;
        
        // Initially reduced mass
        self.mass = if inv_reduced_mass != 0.0 { 1.0 / inv_reduced_mass } else { 0.0 };
        
        self.k = self.mass * self.omega * self.omega;
        self.damp_coeff = 2.0 * self.mass * self.omega * self.damping;
    
        // softness = gamma = 1 / dt(b + dt * k)
        let inv_softness = dt * (self.damp_coeff + dt * self.k);
        self.softness = if inv_softness != 0.0 { 1.0 / inv_softness } else { 0.0 };
    
        // baumgarte beta = dt^2 * k / dt(b + dt * k) = dt^2 * k * softness
        self.beta = dt * dt * self.k * self.softness;
        
        // Add softness to inv. constraint mass (since softness * impulse is on LHS of impulse eqn)
        // inv_red_mass * impulse = -rel_vel_normal - bias - softness * total_impulse
        // inv_red_mass * impulse = -rel_vel_normal - bias - softness * (accum_impulse + impulse)
        // (inv_red_mass + softness) * impulse = -rel_vel_normal - bias - softness * accum_impulse
        // impulse = -(rel_vel_normal + bias + softness*accum_impulse)/(inv_red_mass + softness)
        // Thus equivalent mass == 1.0 / (inv_red_mass + softness)
        let inv_reduced_mass = inv_reduced_mass + self.softness;
        
        self.mass = if inv_reduced_mass != 0.0 { 1.0 / inv_reduced_mass } else { 0.0 };
    }
    
    fn warm_start_velocity(&mut self, a: &mut Body, b: &mut Body, _dt: f32) {
        let impulse = self.impulse * self.normal;
    
        a.add_impulse_at_pos(-impulse, self.r_a);
        b.add_impulse_at_pos(impulse, self.r_b);
    }
    
    fn warm_start_position(&mut self, _a: &mut Body, _b: &mut Body, _dt: f32) {}
    
    fn solve_velocity(&mut self, a: &mut Body, b: & mut Body, dt: f32) {
        let rel_vel = b.velocity - a.velocity +
            b.angular_vel.cross(&self.r_b) - a.angular_vel.cross(&self.r_a);
    
        let rel_vel_normal = self.normal.dot(&rel_vel);
        
        // baumgarte bias = beta * x / dt
        let bias = self.beta * f32::max(self.x.abs() - SPRING_DISPLACEMENT_SLOP, 0.0) * self.x.signum() / dt;
        
        let impulse = -(rel_vel_normal + bias + self.softness * self.impulse) * self.mass;
        
        self.impulse += impulse;
    
        a.add_impulse_at_pos(-impulse * self.normal, self.r_a);
        b.add_impulse_at_pos(impulse * self.normal, self.r_b);
    }
    
    fn solve_position(&mut self, _a: &mut Body, _b: &mut Body, _dt: f32) {}
}
