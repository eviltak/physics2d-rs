use collision::broad_phase::BroadPhase;
use world::{BodyMap, BodyPair};
use super::BodyPairSet;

pub struct NaiveBroadPhase;

impl BroadPhase for NaiveBroadPhase {
    fn potential_pairs(&self, bodies: &BodyMap) -> BodyPairSet {
        let mut pairs = BodyPairSet::default();
        
        for (body_a_id, body_a) in bodies.iter() {
            for (body_b_id, body_b) in bodies.iter() {
                if body_b_id <= body_a_id {
                    continue;
                }
                
                let body_a = &body_a.borrow();
                let body_b = &body_b.borrow();
                let body_pair = BodyPair(*body_a_id, *body_b_id);
                
                if body_a.bounds.intersects(&body_b.bounds) {
                    pairs.insert(body_pair);
                }
            }
        }
        
        pairs
    }
}