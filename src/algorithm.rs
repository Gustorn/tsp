use std::iter::FromIterator;
use std::ops::Fn;

use rand;

use chromosome::Chromosome;
use crossover::Crossover;
use generation::Generation;
use mutation::Mutation;
use reinsertion::Reinsertion;
use selection::Selection;
use termination::Termination;
use utility::RngExt;

pub struct Algorithm<'a, T, C, F, M, R, S> where F: 'a {
    generation: Generation<T>,
    crossover: C,
    fitness: &'a F,
    mutation: M,
    reinsertion: R,
    selection: S,
    mutation_rate: f64,
    crossover_rate: f64,
}

impl<'a, T, C, F, M, R, S> Algorithm<'a, T, C, F, M, R, S>
    where T: Clone,
          C: Crossover<T>,
          F: 'a + Fn(&[T]) -> f64,
          M: Mutation<T>,
          R: Reinsertion<T>,
          S: Selection<T> {

    pub fn new(f: &'a F, s: S, (c, cr): (C, f64), (m, mr): (M, f64), r: R) -> Self {
        Algorithm {
            generation: Generation::empty(),
            crossover: c,
            fitness: f,
            mutation: m,
            reinsertion: r,
            selection: s,
            mutation_rate: mr,
            crossover_rate: cr,
        }
    }

    pub fn generation(&self) -> &Generation<T> {
        &self.generation
    }

    pub fn evolve<U, I, Term>(&mut self, generation: I, mut termination: Term) -> Vec<U>
        where Self: Sized,
              U: IntoIterator<Item = T> + FromIterator<T>,
              I: IntoIterator<Item = U>,
              Term: Termination {

        let mut rng = rand::thread_rng();
        self.generation = Generation::new(generation, self.fitness);

        while !termination.reached(&self.generation) {
            let offspring = self.selection
                .select(&self.generation, &self.crossover)
                .chunks(self.crossover.parents())
                .map(|parents| {
                    if rng.happens(self.crossover_rate) {
                        self.crossover.cross(&parents)
                    } else {
                        parents.iter().map(|x| (*x).clone()).collect()
                    }
                })
                .flat_map(|offspring| offspring.into_iter())
                .map(|child| self.mutation.mutate(child, self.mutation_rate))
                .map(Chromosome::from)
                .collect();

            self.generation = self.reinsertion.reinsert(&self.generation, offspring);
            self.generation.reevaluate(&self.fitness);
        }

        self.generation.iter().map(|x| {
            x.into_iter().cloned().collect()
        }).collect()
    }
}

#[macro_export]
macro_rules! genetic_algorithm {
    (fitness:     $fitness:     expr,
     selection:   $selection:   expr,
     crossover:  ($crossover:   expr, rate: $c_rate: expr),
     mutation:   ($mutation :   expr, rate: $m_rate: expr),
     reinsertion: $reinsertion: expr) => {{
        Algorithm::new($fitness, $selection, ($crossover, $c_rate),
                                    ($mutation,  $m_rate), $reinsertion)
    }};
}
