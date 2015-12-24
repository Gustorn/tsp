use generation::Generation;
use termination::Termination;

#[derive(Copy, Clone)]
pub struct FitnessThreshold {
    threshold: f64,
}

impl FitnessThreshold {
    pub fn new(threshold: f64) -> Self {
        FitnessThreshold { threshold: threshold }
    }
}

impl Termination for FitnessThreshold {
    fn reached<T>(&mut self, generation: &Generation<T>) -> bool {
        generation.best().fitness >= self.threshold
    }
}
