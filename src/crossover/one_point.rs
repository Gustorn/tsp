use rand::{self, Rng};
use crossover::Crossover;


#[derive(Clone, Copy)]
pub struct OnePoint {
    preset_split: Option<usize>,
}

impl OnePoint {
    pub fn new() -> Self {
        OnePoint { preset_split: None }
    }

    pub fn with_preset_split(split: usize) -> Self {
        OnePoint { preset_split: Some(split) }
    }
}

impl<T> Crossover<T> for OnePoint where T: Clone {
    fn parents(&self) -> usize {
        2
    }

    fn cross(&self, parents: &[Vec<T>], crossover_rate: f64) -> Vec<Vec<T>> {
        assert!(parents.len() == Crossover::<T>::parents(self));
        assert!(parents[0].len() == parents[1].len());

        let mut rng = rand::thread_rng();
        pre_crossover!(rng, crossover_rate, parents[0], parents[1]);

        let (parent1, parent2) = (&parents[0], &parents[1]);
        let length = parent1.len();
        let split = match self.preset_split {
            Some(preset_split) => preset_split,
            None => rand::thread_rng().gen_range(0, length),
        };
        vec![join!(parent1[0..split], parent2[split..length]),
             join!(parent2[0..split], parent1[split..length])]
    }
}

#[cfg(test)]
mod tests {
    test_crossover!(one_point_cross, i32, OnePoint::with_preset_split(3),
                    parent(0, 1, 2, 3,  4,  5),
                    parent(6, 7, 8, 9, 10, 11),

                    child(0, 1, 2, 9, 10, 11),
                    child(6, 7, 8, 3,  4,  5));

    test_crossover_passthrough!(one_point_passthrough, i32,
                                OnePoint::new(),
                                parent(0, 1,  2,  3,  4,  5,  6,  7),
                                parent(8, 9, 10, 11, 12, 13, 14, 15));

    test_crossover_panic!(one_point_different_length, i32, OnePoint::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    bench_crossover!(one_point_bench, i32, OnePoint::with_preset_split(3),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
