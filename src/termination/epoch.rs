use generation::Generation;
use termination::Termination;

#[derive(Copy, Clone)]
pub struct Epoch {
    iterations: isize,
}

impl Epoch {
    pub fn new(iterations: isize) -> Self {
        Epoch { iterations: iterations }
    }
}

impl Termination for Epoch {
    fn reached<T>(&mut self, _: &Generation<T>) -> bool where T: Clone {
        self.iterations -= 1;
        self.iterations == -1
    }
}
