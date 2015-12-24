use rand;

use chromosome::Chromosome;
use generation::Generation;
use reinsertion::Reinsertion;
use utility::RngExt;

#[derive(Copy, Clone)]
pub struct UniformReinsertion;

impl UniformReinsertion {
    pub fn new() -> Self {
        UniformReinsertion
    }
}

impl<T> Reinsertion<T> for UniformReinsertion where T: Clone {
    fn reinsert(&self, parents: &Generation<T>, mut offspring: Vec<Chromosome<T>>) -> Generation<T> {
        match parents.size().checked_sub(offspring.len()) {
            Some(diff) if diff > 0 => {
                let mut rng = rand::thread_rng();
                let mut random = Vec::with_capacity(diff);
                for _ in 0..diff {
                    random.push(rng.choose1(parents).clone());
                }
                offspring.extend(random);
            },
            _ => {},
        }
        Generation::from(offspring)
    }
}
