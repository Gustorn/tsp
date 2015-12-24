use std::marker::PhantomData;
use rand;

use mutation::Mutation;
use problem::UniformProblem;
use utility::RngExt;

#[derive(Copy, Clone)]
pub struct UniformMutation<T, P> where T: Clone, P: UniformProblem<T> {
    problem: P,
    _gene: PhantomData<T>,
}

impl<T, P> UniformMutation<T, P> where T: Clone, P: UniformProblem<T> {
    pub fn new(problem: P) -> Self {
        UniformMutation {
            problem: problem,
            _gene: PhantomData,
        }
    }
}

impl<T, P> Mutation<T> for UniformMutation<T, P> where T: Clone, P: UniformProblem<T> {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        for (i, c) in genes.iter_mut().enumerate() {
            if rng.happens(mutation_rate) {
                *c = self.problem.generate_gene(i)
            }
        }
        genes
    }
}
