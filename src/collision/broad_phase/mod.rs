mod naive;

pub use self::naive::NaiveBroadPhase;

use world::{BodyMap, BodyPair};

pub trait BroadPhase {
    fn potential_pairs(&self, bodies: &BodyMap) -> Vec<BodyPair>;
}