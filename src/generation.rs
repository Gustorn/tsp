use std::ops::Fn;
use std::iter::FromIterator;
use itertools::Itertools;

use chromosome::Chromosome;

pub struct Generation<T> {
    pub chromosomes: Vec<Chromosome<T>>,
}

impl<T> Generation<T> where T: Clone {
    pub fn new<F, E, U, I>(generation: I, fitness: &F) -> Self
        where F: Fn(&[T]) -> f64,
              E: AsRef<T> + Clone,
              U: IntoIterator<Item = E> + FromIterator<T>,
              I: IntoIterator<Item = U> {

        let mut chromosomes = generation.into_iter().map(|genome_data| {
            let genes = genome_data
                .into_iter()
                .map(|g| g.as_ref().clone())
                .collect::<Vec<_>>();

            let fitness = fitness(&genes);
            Chromosome::new(genes, fitness)
        }).collect::<Vec<_>>();

        chromosomes.sort_by(|c1, c2| {
                PartialOrd::partial_cmp(&c1.fitness, &c2.fitness)
                    .expect("A fitness value of NaN is not supported").reverse()
        });

        Generation {
            chromosomes: chromosomes,
        }
    }

    pub fn reevaluate<F>(&mut self, fitness: &F) where F: Fn(&[T]) -> f64 {
        for c in self.chromosomes.iter_mut() {
            c.fitness = fitness(&c.genes);
        }
        self.chromosomes.sort_by(|c1, c2| {
                PartialOrd::partial_cmp(&c1.fitness, &c2.fitness)
                    .expect("A fitness value of NaN is not supported").reverse()
        });
    }
}
