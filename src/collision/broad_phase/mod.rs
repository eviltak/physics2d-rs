mod naive;
mod bounds_tree;

pub use self::naive::NaiveBroadPhase;
pub use self::bounds_tree::BoundsTreeBroadPhase;

use crate::world::{Bodies, ConstraintsMap, Body};
use crate::collision::ContactConstraint;

pub type ProxyId = usize;

pub trait BroadPhase {
    fn new_potential_pairs(&self, bodies: &Bodies, constraints: &mut ConstraintsMap<ContactConstraint>);
    
    fn create_proxy(&mut self, body: &Body) -> ProxyId;
    fn destroy_proxy(&mut self, proxy_id: ProxyId);
    fn update_proxy(&mut self, proxy_id: ProxyId, body: &Body);
}
