use rand::{self, Rng};
use mutation::Mutation;

#[derive(Clone)]
pub struct Twors;

impl Twors {
    pub fn new() -> Self {
        Twors
    }
}

impl<T> Mutation<T> for Twors where T: Clone {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        if rng.next_f64() < mutation_rate {
            let length = genes.len();
            let p0 = rng.gen_range(0, length - 1);
            let p1 = rng.gen_range(p0 + 1, length);
            genes.as_mut().swap(p0, p1);
        }
        genes
    }
}
