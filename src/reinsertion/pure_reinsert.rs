use chromosome::Chromosome;
use generation::Generation;
use reinsertion::Reinsertion;

#[derive(Copy, Clone)]
pub struct Pure;

impl Pure {
    pub fn new() -> Self {
        Pure
    }
}

impl<T> Reinsertion<T> for Pure where T: Clone {
    fn reinsert(&self, _: &Generation<T>, offspring: Vec<Chromosome<T>>) -> Generation<T> {
        Generation::from(offspring)
    }
}
