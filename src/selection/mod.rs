mod tournament;

use crossover::Crossover;
use generation::Generation;

pub use self::tournament::Tournament;

pub trait Selection<T> where T: Clone {
    fn retain_ratio(&self) -> f64;

    fn select<C>(&self, generation: &Generation<T>, crossover: &C) -> Vec<Vec<T>>
        where C: Crossover<T>;
}

#[inline]
pub fn selection_size<T, C>(retain_ratio: f64, generation: &Generation<T>, crossover: &C) -> usize
    where C: Crossover<T> {

    let size = generation.size() as f64;
    let (parents, children) = (crossover.parents(), crossover.children());
    let parents_to_children = parents as f64 / children as f64;

    let num_selected = (size * retain_ratio * parents_to_children) as usize;
    (num_selected / parents) * parents
}
