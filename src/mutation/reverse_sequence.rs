use std::ops::Range;
use rand::{self, Rng};
use mutation::Mutation;

#[derive(Clone)]
pub struct ReverseSequence {
    preset_split: Option<(usize, usize)>,
}

impl ReverseSequence {
    pub fn new() -> Self {
        ReverseSequence { preset_split: None }
    }

    pub fn with_preset_split(range: Range<usize>) -> Self {
        if range.start >= range.end {
            panic!("Incorrect range for Crossover operator split point");
        }
        ReverseSequence { preset_split: Some((range.start, range.end)) }
    }
}

impl<T> Mutation<T> for ReverseSequence where T: Clone {
    fn mutate(&self, mut genes: Vec<T>, mutation_rate: f64) -> Vec<T> {
        let mut rng = rand::thread_rng();
        if rng.next_f64() < mutation_rate {
            let length = genes.len();
            let (from, to) = match self.preset_split {
                Some(split) => split,
                None => {
                    let from = rng.gen_range(0, length - 1);
                    let to = rng.gen_range(from + 1, length + 1);
                    (from, to)
                },
            };
            genes[from..to].reverse()
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
        let mutation = ReverseSequence::with_preset_split(1..5);
        let genes = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![1, 5, 4, 3, 2, 6];
        assert!(expected == mutation.mutate(genes, 1.0));
    }
}
