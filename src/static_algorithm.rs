use std::iter::FromIterator;
use std::ops::Fn;

use crossover::Crossover;
use generation::Generation;
use mutation::Mutation;
use reinsertion::{Reinsertion, ReinsertIterator};
use selection::Selection;
use termination::Termination;

pub struct StaticAlgorithm<'a, T, C, F, M, R, S> where F: 'a {
    generation: Generation<T>,
    crossover: C,
    fitness: &'a F,
    mutation: M,
    reinsertion: R,
    selection: S,
    mutation_rate: f64,
    crossover_rate: f64,
}

impl<'a, T, C, F, M, R, S> StaticAlgorithm<'a, T, C, F, M, R, S>
    where T: Clone,
          C: Crossover<T>,
          F: 'a + Fn(&[T]) -> f64,
          M: Mutation<T>,
          R: Reinsertion<T>,
          S: Selection<T> {

    pub fn evolve<E, U, I, Term>(&mut self, generation: I, mut termination: Term) -> Vec<U>
        where Self: Sized,
              E: AsRef<T> + Clone,
              U: IntoIterator<Item = E> + FromIterator<T>,
              I: IntoIterator<Item = U>,
              Term: Termination {

        self.generation = Generation::new(generation, self.fitness);

        while !termination.reached() {
            self.generation = self.selection
                .select(&self.generation, &self.crossover)
                .map(|parents| self.crossover.cross(&parents, self.crossover_rate))
                .flat_map(|offspring| offspring.into_iter())
                .map(|child| self.mutation.mutate(child, self.mutation_rate))
                .reinsert(&self.reinsertion, &self.generation);
            self.generation.reevaluate(&self.fitness);
        }

        self.generation.chromosomes.iter().map(|x| {
            x.genes.iter().cloned().collect()
        }).collect()
    }
}
