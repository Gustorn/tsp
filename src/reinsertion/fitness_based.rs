use std::marker::PhantomData;

use chromosome::Chromosome;
use generation::Generation;
use reinsertion::Reinsertion;

#[derive(Copy, Clone)]
pub struct FitnessBased<'a, T, F> where T: Clone, F: 'a + Fn(&[T]) -> f64 {
    fitness: &'a F,
    _gene: PhantomData<T>,
}

impl<'a, T, F> FitnessBased<'a, T, F> where T: Clone, F: 'a + Fn(&[T]) -> f64 {
    pub fn new(fitness: &'a F) -> Self {
        FitnessBased {
            fitness: fitness,
            _gene: PhantomData,
        }
    }
}

impl<'a, T, F> Reinsertion<T> for FitnessBased<'a, T, F> where T: Clone, F: 'a + Fn(&[T]) -> f64 {
    fn reinsert(&self, parents: &Generation<T>, mut offspring: Vec<Chromosome<T>>) -> Generation<T> {
        let length = offspring.len();
        match length.checked_sub(parents.size()) {
            Some(diff) if diff > 0 => {
                for c in offspring.iter_mut() {
                    c.fitness = (*self.fitness)(c);
                }
                offspring.as_mut().sort_by(|a, b| Ord::cmp(a, b).reverse());
                offspring.truncate(length - diff);
            },
            _ => {},
        }
        Generation::from(offspring)
    }
}
