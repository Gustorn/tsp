use rand::{self, Rng};

use chromosome::Chromosome;
use crossover::Crossover;
use generation::Generation;
use selection::Selection;
use utility::RngExt;

#[derive(Clone)]
pub struct Tournament {
    retain_ratio: f64,
    size: usize,
}

impl Tournament {
    pub fn new(retain_ratio: f64, tournament_size: usize) -> Self {
        Tournament {
            retain_ratio: retain_ratio,
            size: tournament_size
        }
    }
}

impl<T> Selection<T> for Tournament where T: Clone {
    fn retain_ratio(&self) -> f64 {
        self.retain_ratio
    }

    fn select<C>(&self, generation: &Generation<T>, crossover: &C) -> Vec<Vec<T>>
        where C: Crossover<T> {

        let mut rng = rand::thread_rng();
        let size = generation.size() as f64;
        let (parents, children) = (crossover.parents(), crossover.children());

        let parent_to_children = parents as f64 / children as f64;

        let num_selected = (size * self.retain_ratio * parent_to_children) as usize;
        let num_selected = (num_selected / parents) * parents;
        let mut selected = Vec::with_capacity(num_selected);

        for _ in 0..num_selected {
            selected.push(hold_tournament(&generation, self.size, &mut rng));
        }
        selected
    }
}

fn hold_tournament<'a, T, R>(chromosomes: &'a [Chromosome<T>],
                             tournament_size: usize, rng: &mut R) -> Vec<T>
    where T: Clone, R: Rng {

    assert!(tournament_size <= chromosomes.len(), "Tournament size is too big for this generation");
    let mut max = rng.choose1(chromosomes);
    for _ in 0..tournament_size {
        let current = rng.choose1(chromosomes);
        if current > max {
            max = current;
        }
    }

    max.into_iter().cloned().collect()
}
