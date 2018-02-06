mod transform;
mod collections;

pub use object::body::{Body, BodyId, BodyRef, Material};
pub use self::transform::Transform;
pub(crate) use object::body::BodyPair;
pub(crate) use self::collections::{BodyMap, ContactsMap};

use self::collections::{ManifoldMap, ConstraintSolverMap};
use collision::{VelocityConstraintManifold, PositionConstraintManifold, collide};
use collision::broad_phase::{BroadPhase, NaiveBroadPhase};

pub struct World {
    bodies: BodyMap,
    
    broad_phase: NaiveBroadPhase,
    
    // TODO: Why use contacts, store directly in manifolds?
    pub(crate) contacts: ContactsMap,
    velocity_constraint_manifolds: ManifoldMap<VelocityConstraintManifold>,
    position_constraint_manifolds: ManifoldMap<PositionConstraintManifold>,
    
    body_created_count: usize,
}

impl World {
    pub fn new() -> World {
        World {
            bodies: BodyMap::default(),
            broad_phase: NaiveBroadPhase,
            contacts: ContactsMap::default(),
            velocity_constraint_manifolds: ManifoldMap::default(),
            position_constraint_manifolds: ManifoldMap::default(),
            body_created_count: 0,
        }
    }
    
    pub fn add_body(&mut self, mut body: Body) -> BodyId {
        let body_id = self.body_created_count as BodyId;
        self.body_created_count += 1;
        
        body.id = body_id;
        self.bodies.insert(body_id, BodyRef::new(body));
        
        body_id
    }
    
    pub fn body_ref(&self, body_id: &BodyId) -> &BodyRef {
        &self.bodies[body_id]
    }
    
    pub fn bodies(&self) -> Vec<&BodyRef> {
        self.bodies.values().collect()
    }
    
    pub fn update(&mut self, dt: f32) {
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.update(dt);
        }
    
        for pair in self.broad_phase.potential_pairs(&self.bodies) {
            let body_a = &self.bodies[&pair.0].borrow();
            let body_b = &self.bodies[&pair.1].borrow();
            
            if let Some(new_contacts) = collide(body_a, body_b) {
                if self.velocity_constraint_manifolds.contains_key(&pair) {
                    self.velocity_constraint_manifolds.get_mut(&pair).unwrap().update_constraints(&new_contacts);
                } else {
                    self.velocity_constraint_manifolds
                        .insert(pair, VelocityConstraintManifold::new(pair, &new_contacts));
                }
        
                self.position_constraint_manifolds.insert(pair, PositionConstraintManifold::new(&new_contacts));
        
                *self.contacts.entry(pair).or_insert(Vec::new()) = new_contacts;
            } else {
                self.contacts.remove(&pair);
        
                self.velocity_constraint_manifolds.remove(&pair);
                self.position_constraint_manifolds.remove(&pair);
            }
        }
        
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_force(dt);
        }
        
        self.velocity_constraint_manifolds.initialize_constraints(&self.bodies, dt);
        self.velocity_constraint_manifolds.warm_start(&self.bodies, dt);
        self.velocity_constraint_manifolds.solve_constraints(&self.bodies, dt);
        
        for body in self.bodies.values_mut() {
            let body = &mut body.borrow_mut();
            body.integrate_velocity(dt);
        }
        
        self.position_constraint_manifolds.initialize_constraints(&self.bodies, dt);
        self.position_constraint_manifolds.warm_start(&self.bodies, dt);
        self.position_constraint_manifolds.solve_constraints(&self.bodies, dt);
    }
}
