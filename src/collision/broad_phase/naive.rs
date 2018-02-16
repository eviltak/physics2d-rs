use world::{BodyMap, ConstraintsMap, BodyPair, Body};
use collision::ContactConstraint;
use super::{BroadPhase, ProxyId};

pub struct NaiveBroadPhase;

impl BroadPhase for NaiveBroadPhase {
    fn new_potential_pairs(&self, bodies: &BodyMap, constraints: &mut ConstraintsMap<ContactConstraint>) {
        for (body_a_id, body_a) in bodies.iter() {
            for (body_b_id, body_b) in bodies.iter() {
                if body_b_id <= body_a_id {
                    continue;
                }
                
                let body_a = &body_a.borrow();
                let body_b = &body_b.borrow();
                
                if body_a.bounds.intersects(&body_b.bounds) && (!body_a.is_static() || !body_b.is_static()) {
                    let body_pair = BodyPair(*body_a_id, *body_b_id);
                    if !constraints.contains_key(&body_pair) {
                        constraints.insert(body_pair, Vec::new());
                    }
                }
            }
        }
    }
    
    fn create_proxy(&mut self, _body: &Body) -> ProxyId {
        ProxyId::default()
    }
    
    fn destroy_proxy(&mut self, _proxy_id: ProxyId) {}
    
    fn update_proxy(&mut self, _proxy_id: ProxyId, _body: &Body) {}
}
