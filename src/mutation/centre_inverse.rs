use rand;

use mutation::Mutation;
use utility::RngExt;

#[derive(Copy, Clone)]
pub struct CentreInverse {
    preset_split: Option<usize>,
}

impl CentreInverse {
    pub fn new() -> Self {
        CentreInverse { preset_split: None }
    }

    pub fn with_preset_split(split: usize) -> Self {
        CentreInverse { preset_split: Some(split) }
    }
}

impl<T> Mutation<T> for CentreInverse where T: Clone {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        if rng.happens(mutation_rate) {
            let length = genes.len();
            let split = self.preset_split.unwrap_or(rng.index(&genes));
            genes[0..split].reverse();
            genes[split..length].reverse();
        }
        genes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mutation::Mutation;

    #[test]
    fn centre_inverse() {
        let mutation = CentreInverse::with_preset_split(4);
        let genes = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![4, 3, 2, 1, 6, 5];
        assert!(expected == mutation.mutate(genes, 1.0));
    }
}
