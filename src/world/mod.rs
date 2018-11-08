mod transform;
mod collections;
mod body;
pub mod debug;

pub use self::body::{Body, BodyId, Material};
pub use self::transform::Transform;
pub(crate) use self::body::BodyPair;
pub(crate) use self::collections::{BodyMap, ConstraintsMap, BodiesIter};

use self::collections::{ConstraintSolverMap};
use collision::{ContactConstraint, collide};
use collision::broad_phase::{BroadPhase, NaiveBroadPhase, BoundsTreeBroadPhase};
use joint::Joint;

pub struct World {
    bodies: BodyMap,
    
    broad_phase: BoundsTreeBroadPhase,
    
    contact_constraints: ConstraintsMap<ContactConstraint>,
    joints: ConstraintsMap<Joint>,
    
    body_created_count: usize,
    
    pub velocity_iterations: u8,
    pub position_iterations: u8,
}

impl Default for World {
    fn default() -> World {
        World::new(8, 2)
    }
}

impl World {
    pub fn new(velocity_iterations: u8, position_iterations: u8) -> World {
        World {
            bodies: BodyMap::default(),
            broad_phase: BoundsTreeBroadPhase::new(),
            contact_constraints: ConstraintsMap::default(),
            joints: ConstraintsMap::default(),
            body_created_count: 0,
            velocity_iterations,
            position_iterations,
        }
    }
    
    pub fn add_body(&mut self, mut body: Body) -> BodyId {
        let body_id = self.body_created_count as BodyId;
        self.body_created_count += 1;
        
        body.id = body_id;
        body.proxy_id = self.broad_phase.create_proxy(&body);
        self.bodies.insert(body_id, body);
        
        body_id
    }
    
    pub fn add_joint(&mut self, bodies: (BodyId, BodyId), joint: Joint) {
        let bodies = BodyPair::new(bodies.0, bodies.1);
        let body_joints = self.joints.entry(bodies).or_insert(Vec::new());
        body_joints.push(joint);
    }
    
    pub fn get_joints(&self, bodies: (BodyId, BodyId)) -> Option<&Vec<Joint>> {
        let bodies = BodyPair::new(bodies.0, bodies.1);
        self.joints.get(&bodies)
    }
    
    pub fn get_body(&self, body_id: &BodyId) -> &Body {
        &self.bodies[body_id]
    }
    
    pub fn get_body_mut(&mut self, body_id: &BodyId) -> &mut Body {
        self.bodies.get_mut(body_id).unwrap()
    }
    
    pub fn bodies_iter(&self) -> BodiesIter {
        BodiesIter {
            values: self.bodies.values()
        }
    }
    
    pub fn update(&mut self, dt: f32) {
        for body in self.bodies.values_mut() {
            body.update(dt);
            self.broad_phase.update_proxy(body.proxy_id, body);
        }
        
        {
            let bodies = &self.bodies;
            self.contact_constraints.retain(|pair, constraints| pair.with(bodies, |a, b| a.bounds.intersects(&b.bounds)));
        }
        
        self.broad_phase.new_potential_pairs(&self.bodies, &mut self.contact_constraints);
        
        self.broad_phase.post_update();
        
        {
            let bodies = &self.bodies;
            self.contact_constraints.retain(|pair, constraints| {
                let body_a = &bodies[&pair.0];
                let body_b = &bodies[&pair.1];
                
                if let Some(new_contacts) = collide(body_a, body_b) {
                    let new_constraints =
                        if !constraints.is_empty() {
                            ContactConstraint::with_persistent_contacts(constraints, &new_contacts)
                        } else {
                            ContactConstraint::with_contacts(&new_contacts)
                        };
                    
                    *constraints = new_constraints;
                    
                    true
                } else {
                    false
                }
            });
        }
        
        for body in self.bodies.values_mut() {
            body.integrate_force(dt);
        }
        
        self.contact_constraints.initialize_velocity(&mut self.bodies, dt);
        self.contact_constraints.warm_start_velocity(&mut self.bodies, dt);
        
        self.joints.initialize_velocity(&mut self.bodies, dt);
        self.joints.warm_start_velocity(&mut self.bodies, dt);
        
        for _ in 0..self.velocity_iterations {
            self.joints.solve_velocity(&mut self.bodies, dt);
            
            self.contact_constraints.solve_velocity(&mut self.bodies, dt);
        }
        
        for body in self.bodies.values_mut() {
            body.integrate_velocity(dt);
        }
        
        self.contact_constraints.warm_start_position(&mut self.bodies, dt);
    
        for _ in 0..self.position_iterations {
            self.joints.solve_position(&mut self.bodies, dt);
            
            self.contact_constraints.solve_position(&mut self.bodies, dt);
        }
    }
}
