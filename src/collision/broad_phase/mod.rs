mod naive;

pub use self::naive::NaiveBroadPhase;

use world::{BodyMap, BodyPair};

use fnv::FnvHashSet;

pub type BodyPairSet = FnvHashSet<BodyPair>;

pub trait BroadPhase {
    fn potential_pairs(&self, bodies: &BodyMap) -> BodyPairSet;
}