use rand::{self, Rng};
use itertools::Itertools;

use mutation::Mutation;
use utility::{Gather, RngExt};

#[derive(Copy, Clone)]
pub struct Thrors;

impl Thrors {
    pub fn new() -> Self {
        Thrors
    }
}

impl<T> Mutation<T> for Thrors where T: Clone {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        if rng.happens(mutation_rate) {
            let length = genes.len();
            unique_samples!(0, length).take(3).triples()
                .foreach(|(i, j, k)| {
                    (*genes).as_mut().swap(i, j);
                    (*genes).as_mut().swap(i, k);
                });
        }
        genes
    }
}
