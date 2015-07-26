use termination::Termination;

pub struct Epoch {
    iterations: usize,
}

impl Epoch {
    pub fn new(iterations: usize) -> Self {
        Epoch { iterations: iterations }
    }
}

impl Termination for Epoch {
    fn reached(&mut self) -> bool {
        let reached = self.iterations == 0;
        self.iterations -= 1;
        reached
    }
}
