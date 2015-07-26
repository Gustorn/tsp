use std::marker::PhantomData;
use rand::{self, Rng};

use mutation::Mutation;
use problem::UniformProblem;

pub struct Uniform<T, P> where T: Clone, P: UniformProblem<T> {
    problem: P,
    _gene: PhantomData<T>,
}

impl<T, P> Clone for Uniform<T, P> where T: Clone, P: UniformProblem<T> {
    fn clone(&self) -> Self {
        Uniform {
            problem: self.problem.clone(),
            _gene: PhantomData,
        }
    }
}

impl<T, P> Uniform<T, P> where T: Clone, P: UniformProblem<T> {
    pub fn new(problem: P) -> Self {
        Uniform {
            problem: problem,
            _gene: PhantomData,
        }
    }
}

impl<T, P> Mutation<T> for Uniform<T, P> where T: Clone, P: UniformProblem<T> {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        for (i, c) in genes.iter_mut().enumerate() {
            if rng.next_f64() < mutation_rate {
                *c = self.problem.generate_gene(i)
            }
        }
        genes
    }
}
