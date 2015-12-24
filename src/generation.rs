use std::ops::Fn;
use std::slice::Iter;

use itertools::Itertools;

use chromosome::Chromosome;

pub struct Generation<T> {
    chromosomes: Vec<Chromosome<T>>,
    total_fitness: f64,
}

impl<T> Generation<T> {
    pub fn empty() -> Self {
        Generation { chromosomes: Vec::new(), total_fitness: 0.0 }
    }

    pub fn new<F, U, I>(generation: I, fitness: &F) -> Self
        where F: Fn(&[T]) -> f64,
              U: IntoIterator<Item = T>,
              I: IntoIterator<Item = U> {

        let mut chromosomes = generation.into_iter().map(|genome_data| {
            let genes = genome_data
                .into_iter()
                .collect::<Vec<_>>();

            let fitness = fitness(&genes);
            Chromosome::new(genes, fitness)
        }).collect::<Vec<_>>();

        chromosomes.sort_by(|c1, c2| Ord::cmp(&c1, &c2).reverse());
        let total_fitness = chromosomes.iter().fold(0.0, |sum, c| sum + c.fitness);

        Generation { chromosomes: chromosomes, total_fitness: total_fitness }
    }

    pub fn best(&self) -> &Chromosome<T> {
        &self.chromosomes[0]
    }

    pub fn iter(&self) -> Iter<Chromosome<T>> {
        self.chromosomes.iter()
    }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }

    pub fn top(&self, n: usize) -> &[Chromosome<T>] {
        &self.chromosomes[0..n]
    }

    pub fn total_fitness(&self) -> f64 {
        self.total_fitness
    }

    pub fn reevaluate<F>(&mut self, fitness: &F) where F: Fn(&[T]) -> f64  {
        for chromosome in self.chromosomes.iter_mut() {
                chromosome.fitness = fitness(&chromosome);
        }
        self.chromosomes.sort_by(|c1, c2| Ord::cmp(&c1, &c2).reverse());
    }
}

forward_as!(Generation, Chromosome<T>, chromosomes);
forward_index!(Generation, Chromosome<T>, chromosomes);
forward_into_iter!(Generation, Chromosome<T>, chromosomes);

impl<T> Clone for Generation<T> where T: Clone {
    fn clone(&self) -> Self {
        Generation {
            chromosomes: self.chromosomes.clone(),
            total_fitness: self.total_fitness,
        }
    }
}

impl<T> From<Vec<Chromosome<T>>> for Generation<T> {
    fn from(chromosomes: Vec<Chromosome<T>>) -> Self {
        Generation {
            chromosomes: chromosomes,
            total_fitness: 0.0,
        }
    }
}
