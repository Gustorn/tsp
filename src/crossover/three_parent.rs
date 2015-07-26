use crossover::Crossover;

use rand::{self, Rng};

#[derive(Clone, Copy)]
pub struct ThreeParent;

impl ThreeParent {
    pub fn new() -> Self {
        ThreeParent
    }
}

impl<T> Crossover<T> for ThreeParent where T: Clone + PartialEq {
    fn parents(&self) -> usize {
        3
    }

    fn cross(&self, parents: &[Vec<T>], crossover_rate: f64) -> Vec<Vec<T>> {
        assert!(parents.len() == Crossover::<T>::parents(self));
        assert!(parents[0].len() == parents[1].len() &&
                parents[1].len() == parents[2].len());

        let mut rng = rand::thread_rng();
        pre_crossover!(rng, crossover_rate, parents[0], parents[1], parents[2]);

        let (parent1, parent2, parent3) = (&parents[0], &parents[1], &parents[2]);
        let genes = parent1.iter()
            .zip(parent2.iter())
            .zip(parent3.iter())
            .map(|((p1, p2), p3)| {
                if *p1 == *p2 {
                    p1.clone()
                } else {
                    p3.clone()
                }
            });
        vec![genes.collect()]
    }
}

#[cfg(test)]
mod tests {
    test_crossover!(three_parent_cross, i32, ThreeParent::new(),
                    parent(0, 0, 1, 1, 0, 0, 1, 1),
                    parent(0, 1, 0, 1, 0, 1, 0, 1),
                    parent(0, 0, 0, 0, 1, 1, 1, 1),

                    child(0, 0, 0, 1, 0, 1, 1, 1));

    test_crossover_passthrough!(three_parent_passthrough, i32,
                                ThreeParent::new(),
                                parent(0, 1,  2,  3,  4,  5,  6,  7),
                                parent(8, 9, 10, 11, 12, 13, 14, 15),
                                parent(1, 2,  3,  4,  5,  6,  7,  8));

    test_crossover_panic!(three_parent_different_length, i32, ThreeParent::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
                          parent(1, 2, 3));

    bench_crossover!(three_parent_bench, i32, ThreeParent::new(),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(8, 6, 2, 2, 9, 5, 3, 1, 8, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
