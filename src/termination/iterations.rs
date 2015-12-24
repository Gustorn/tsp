use generation::Generation;
use termination::Termination;

#[derive(Copy, Clone)]
pub struct Iterations {
    iterations: isize,
}

impl Iterations {
    pub fn new(iterations: isize) -> Self {
        Iterations { iterations: iterations }
    }
}

impl Termination for Iterations {
    fn reached<T>(&mut self, _: &Generation<T>) -> bool {
        self.iterations -= 1;
        self.iterations == -1
    }
}
