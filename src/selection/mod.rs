mod tournament;

use crossover::Crossover;
use generation::Generation;

pub use self::tournament::Tournament;

pub trait Selection<T> : Clone where T: Clone {
    fn retain_ratio(&self) -> f64;

    fn select<C>(&self, generation: &Generation<T>, crossover: &C) -> Vec<Vec<T>>
        where C: Crossover<T>;
}
