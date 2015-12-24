mod fitness_convergence;
mod fitness_threshold;
mod iterations;

use generation::Generation;

pub use self::fitness_convergence::FitnessConvergence;
pub use self::fitness_threshold::FitnessThreshold;
pub use self::iterations::Iterations;

pub trait Termination {
    fn reached<T>(&mut self, generation: &Generation<T>) -> bool;
}
