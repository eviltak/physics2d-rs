mod naive;
mod bounds_tree;

pub use self::naive::NaiveBroadPhase;

use world::{BodyMap, BodyPair, Body};

use fnv::FnvHashSet;

pub type BodyPairSet = FnvHashSet<BodyPair>;

pub type ProxyId = usize;

pub trait BroadPhase {
    fn potential_pairs(&self, bodies: &BodyMap) -> BodyPairSet;
    
    fn create_proxy(&mut self, body: &Body) -> ProxyId;
    fn destroy_proxy(&mut self, proxy_id: ProxyId);
    fn update_proxy(&mut self, proxy_id: ProxyId, body: &Body);
}
