use std::ops::Not;
use rand;

use mutation::Mutation;
use utility::RngExt;

#[derive(Copy, Clone)]
pub struct FlipBit;

impl FlipBit {
    pub fn new() -> Self {
        FlipBit
    }
}

impl<T> Mutation<T> for FlipBit where T: Clone + Not<Output = T> {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        if rng.happens(mutation_rate) {
            let position = rng.index(&genes);
            genes[position] = !(genes[position].clone());
        }
        genes
    }
}
