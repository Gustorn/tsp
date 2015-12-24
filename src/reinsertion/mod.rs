mod elitist;
mod fitness_based;
mod pure_reinsert;
mod uniform;

use chromosome::Chromosome;
use generation::Generation;

pub use self::elitist::Elitist;
pub use self::fitness_based::FitnessBased;
pub use self::pure_reinsert::Pure;
pub use self::uniform::UniformReinsertion;

pub trait Reinsertion<T> {
    fn reinsert(&self, parents: &Generation<T>, offspring: Vec<Chromosome<T>>) -> Generation<T>;
}
