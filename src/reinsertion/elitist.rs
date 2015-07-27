use chromosome::Chromosome;
use generation::Generation;
use reinsertion::Reinsertion;

#[derive(Copy, Clone)]
pub struct Elitist;

impl Elitist {
    pub fn new() -> Self {
        Elitist
    }
}

impl<T> Reinsertion<T> for Elitist where T: Clone {
    fn reinsert(&self, parents: &Generation<T>, mut offspring: Vec<Chromosome<T>>) -> Generation<T> {
        match parents.size().checked_sub(offspring.len()) {
            Some(diff) if diff > 0 => offspring.extend(parents.top(diff).iter().cloned()),
            _ => {},
        }
        Generation::from(offspring)
    }
}
