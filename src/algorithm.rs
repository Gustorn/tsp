use std::ops::Fn;

use rand;

use chromosome::Chromosome;
use crossover::Crossover;
use generation::Generation;
use mutation::Mutation;
use reinsertion::Reinsertion;
use selection::Selection;
use termination::Termination;
use tracking::Tracking;
use utility::RngExt;

pub struct Algorithm<'a, T, C, F, M, R, S, TR> where F: 'a {
    generation: Generation<T>,
    crossover: C,
    fitness: &'a F,
    mutation: M,
    reinsertion: R,
    selection: S,
    tracking: TR,
    mutation_rate: f64,
    crossover_rate: f64,
}

impl<'a, T, C, F, M, R, S, TR> Algorithm<'a, T, C, F, M, R, S, TR>
    where T: Clone,
          C: Crossover<T>,
          F: 'a + Fn(&[T]) -> f64,
          M: Mutation<T>,
          R: Reinsertion<T>,
          S: Selection<T>,
          TR: Tracking<T> {

    pub fn new(fitness: &'a F,
               selection: S,
               (crossover, crossover_rate): (C, f64),
               (mutation, mutation_rate): (M, f64),
               reinsertion: R,
               tracking: TR) -> Self {
        Algorithm {
            generation: Generation::empty(),
            crossover: crossover,
            fitness: fitness,
            mutation: mutation,
            reinsertion: reinsertion,
            selection: selection,
            mutation_rate: mutation_rate,
            crossover_rate: crossover_rate,
            tracking: tracking,
        }
    }

    pub fn evolve<U, I, Term>(&mut self, generation: I, mut termination: Term) -> &TR
        where U: IntoIterator<Item = T>,
              I: IntoIterator<Item = U>,
              Term: Termination {

        let mut rng = rand::thread_rng();
        self.generation = Generation::new(generation, self.fitness);
        self.tracking.register(&self.generation);

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
            self.tracking.register(&self.generation);
        }

        &self.tracking
    }
}

#[macro_export]
macro_rules! genetic_algorithm {
    (fitness:     $fitness:     expr,
     selection:   $selection:   expr,
     crossover:  ($crossover:   expr, rate: $c_rate: expr),
     mutation:   ($mutation :   expr, rate: $m_rate: expr),
     reinsertion: $reinsertion: expr,
     tracking:    $tracking: expr) => {{
        use genetic::algorithm::Algorithm;
        Algorithm::new( $fitness,
                        $selection,
                       ($crossover, $c_rate),
                       ($mutation,  $m_rate),
                        $reinsertion,
                        $tracking)
    }};
}
