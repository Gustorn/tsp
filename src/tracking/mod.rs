mod best_solution;

use generation::Generation;

pub use self::best_solution::BestSolution;

pub trait Tracking<T> {
    fn register(&mut self, generation: &Generation<T>);
}

