use approx::{self, Abs};

use generation::Generation;
use termination::Termination;

#[derive(Copy, Clone)]
pub struct FitnessConvergence {
    previous: Option<f64>,
    target_generations: usize,
    elapsed_generations: usize,
    epsilon: f64,
}

impl FitnessConvergence {
    pub fn new(target_generations: usize, epsilon: f64) -> Self {
        FitnessConvergence {
            previous: None,
            target_generations: target_generations,
            elapsed_generations: 0,
            epsilon: epsilon
        }
    }
}

impl Termination for FitnessConvergence {
    fn reached<T>(&mut self, generation: &Generation<T>) -> bool {
        let best = generation.best().fitness;
        let previous = match self.previous {
            None => {
                self.previous = Some(best);
                return false;
            },
            Some(previous) => previous,
        };

        if approx::eq(&previous, &best, Abs::tol(self.epsilon)) {
            self.elapsed_generations += 1;
            self.elapsed_generations >= self.target_generations
        } else {
            self.elapsed_generations = 0;
            false
        }
    }
}
