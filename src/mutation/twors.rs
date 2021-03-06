use rand;

use mutation::Mutation;
use utility::RngExt;

#[derive(Copy, Clone)]
pub struct Twors;

impl Twors {
    pub fn new() -> Self {
        Twors
    }
}

impl<T> Mutation<T> for Twors where T: Clone {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        if rng.happens(mutation_rate) {
            let (p0, p1) = rng.range_indexes(&genes);
            (*genes).as_mut().swap(p0, p1);
        }
        genes
    }
}
