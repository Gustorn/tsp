mod epoch;
mod fitness_convergence;
mod fitness_threshold;

use generation::Generation;

pub use self::epoch::Epoch;
pub use self::fitness_convergence::FitnessConvergence;
pub use self::fitness_threshold::FitnessThreshold;

pub trait Termination {
    fn reached<T>(&mut self, generation: &Generation<T>) -> bool where T: Clone;
}
