mod roulette_wheel;
mod rank_based;

pub use self::roulette_wheel::*;
pub use self::rank_based::*;
use crate::*;

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}
