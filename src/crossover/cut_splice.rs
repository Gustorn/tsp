use rand::{self, Rng};
use crossover::Crossover;

/// An operator that results in a change in the length of the children
///
/// This crossover variant chooses the split points on the parents independently, and joins the
/// resulting segments together, ignoring the length of the original parents. Because of this
/// property, it can be used for parents of differing lengths.
#[derive(Clone, Copy)]
pub struct CutAndSplice {
    preset_split: Option<(usize, usize)>,
}

impl CutAndSplice {
    /// Constructs a new CutAndSplice crossover that chooses its split points randomly.
    /// These split points will be randomly selected every time a crossover is requested.
    pub fn new() -> Self {
        CutAndSplice { preset_split: None }
    }

    /// Creates a
    pub fn with_preset_split(first: usize, second: usize) -> Self {
        CutAndSplice { preset_split: Some((first, second)) }
    }
}

impl<T> Crossover<T> for CutAndSplice where T: Clone {
    fn parents(&self) -> usize {
        2
    }

    fn cross(&self, parents: &[Vec<T>], crossover_rate: f64) -> Vec<Vec<T>> {
        assert!(parents.len() == Crossover::<T>::parents(self));

        let mut rng = rand::thread_rng();
        pre_crossover!(rng, crossover_rate, parents[0], parents[1]);

        let (parent1, parent2) = (&parents[0], &parents[1]);
        let (length1, length2) = (parent1.len(), parent2.len());
        let (split1, split2) = match self.preset_split {
            Some(preset_split) => preset_split,
            None => {
                (rng.gen_range(0, length1), rng.gen_range(0, length2))
            },
        };

        vec![join!(parent1[0..split1], parent2[split2..length2]),
             join!(parent2[0..split2], parent1[split1..length1])]
    }
}

#[cfg(test)]
mod tests {
    test_crossover!(cut_splice_cross, i32, CutAndSplice::with_preset_split(3, 5),
                    parent(0, 1,  2,  3,  4,  5,  6,  7),
                    parent(8, 9, 10, 11, 12, 13, 14, 15),

                    child(0, 1, 2, 13, 14, 15),
                    child(8, 9, 10, 11, 12, 3, 4, 5, 6, 7));

    test_crossover_passthrough!(cut_splice_passthrough, i32,
                                CutAndSplice::with_preset_split(3, 5),
                                parent(0, 1,  2,  3,  4,  5,  6,  7),
                                parent(8, 9, 10, 11, 12, 13, 14, 15));

    bench_crossover!(cut_splice_bench, i32, CutAndSplice::new(),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
