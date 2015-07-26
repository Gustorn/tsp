use rand::{self, Rng};
use mutation::Mutation;

#[derive(Clone)]
pub struct PartialShuffle {
    n: usize,
}

impl PartialShuffle {
    pub fn new(n: usize) -> Self {
        PartialShuffle { n: n }
    }
}

impl<T> Mutation<T> for PartialShuffle where T: Clone {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        for i in 0..self.n {
            if rng.next_f64() < mutation_rate {
                let j = rng.gen_range(0, self.n);
                genes.as_mut().swap(i, j);
            }
        }
        genes
    }
}
