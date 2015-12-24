use rand::{self, Rng};

use chromosome::Chromosome;
use crossover::Crossover;
use generation::Generation;
use selection::{Selection, selection_size};
use utility::RngExt;

#[derive(Copy, Clone)]
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

        let selection_size = selection_size(self.retain_ratio, generation, crossover);
        let mut selected = Vec::with_capacity(selection_size);
        for _ in 0..selection_size {
            selected.push(hold_tournament(&generation, self.size, &mut rng));
        }
        selected
    }
}

fn hold_tournament<T, R>(chromosomes: &[Chromosome<T>],
                         tournament_size: usize, rng: &mut R) -> Vec<T>
    where T: Clone, R: Rng {

    let mut max = rng.choose1(chromosomes);
    for _ in 0..tournament_size {
        let current = rng.choose1(chromosomes);
        if current > max {
            max = current;
        }
    }

    max.into_iter().cloned().collect()
}
