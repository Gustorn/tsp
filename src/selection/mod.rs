use std::slice::Chunks;

use crossover::Crossover;
use generation::Generation;

pub trait Selection<T> : Clone + Copy where T: Clone {
    fn select<C>(&self, generation: &Generation<T>, crossover: &C) -> Chunks<Vec<T>>
        where C: Crossover<T>;
}
