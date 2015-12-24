use rand::{self, Rng};

use mutation::Mutation;
use utility::RngExt;

#[derive(Copy, Clone)]
pub struct Shuffle;

impl Shuffle {
    pub fn new() -> Self {
        Shuffle
    }
}

impl<T> Mutation<T> for Shuffle where T: Clone {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        if rng.happens(mutation_rate) {
            rng.shuffle(genes.as_mut());
        }
        genes
    }
}
