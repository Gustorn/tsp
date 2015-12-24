use chromosome::Chromosome;
use generation::Generation;
use tracking::Tracking;

pub struct BestSolution<T> {
    best: Option<Chromosome<T>>,
}

impl<T> BestSolution<T> where T: Clone {
    pub fn new() -> Self {
        BestSolution { best: None }
    }

    pub fn best(&self) -> (Vec<T>, f64) {
        if let Some(ref best) = self.best {
            (Vec::from(best.genes()), best.fitness)
        } else {
            unimplemented!()
        }
    }
}

impl<T> Tracking<T> for BestSolution<T> where T: Clone {
    fn register(&mut self, generation: &Generation<T>) {
        if let Some(ref mut best) = self.best {
            if generation.best().fitness > best.fitness {
                *best = generation.best().clone();
            }
        } else {
            self.best = Some(generation.best().clone());
        }
    }
}


