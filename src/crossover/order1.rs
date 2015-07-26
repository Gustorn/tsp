use std::ops::Range;
use rand::{self, Rng};
use crossover::Crossover;

#[derive(Clone, Copy)]
pub struct Order1 {
    preset_split: Option<(usize, usize)>,
}

impl Order1 {
    pub fn new() -> Self {
        Order1 { preset_split: None }
    }

    pub fn with_preset_split(range: Range<usize>) -> Self {
        if range.start >= range.end {
            panic!("Incorrect range for crossover preset split point");
        }
        Order1 { preset_split: Some((range.start, range.end)) }
    }
}

impl<T> Crossover<T> for Order1 where T: Clone + PartialEq {
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
            None => {
                let start = rng.gen_range(0, length - 1);
                (start, rng.gen_range(start + 1, length))
            },
        };

        vec![order1(parent1, parent2, split),
             order1(parent2, parent1, split)]
    }
}



fn order1<T>(parent1: &[T], parent2: &[T],
                   (split_start, split_end): (usize, usize)) -> Vec<T>
    where T: Clone + PartialEq {

    let length = parent1.len();
    let swath = &parent1[split_start..split_end];

    let mut remaining = parent2.iter().filter(|g| !swath.contains(g));
    let mut child = Vec::with_capacity(length);

    for _ in 0..split_start {
        child.push(remaining.next()
                   .expect("Not enough unique genes in the second parent for the Order1 crossover operator")
                   .clone());
    }

    child.extend(swath.iter().cloned());

    for _ in split_end..length {
        child.push(remaining.next()
                   .expect("Not enough unique genes in the second parent for the Order1 crossover operator")
                   .clone());
    }
    child
}

#[cfg(test)]
mod tests {
    test_crossover!(order1_cross, i32, Order1::with_preset_split(3..8),
                    parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                    parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),

                    child(0, 4, 7, 3, 6, 2, 5, 1, 8, 9),
                    child(8, 2, 1, 3, 4, 5, 6, 7, 9, 0));

    test_crossover_passthrough!(order1_passthrough, i32,
                                Order1::new(),
                                parent(0, 1,  2,  3,  4,  5,  6,  7),
                                parent(8, 9, 10, 11, 12, 13, 14, 15));

    test_crossover_panic!(order1_different_length, i32, Order1::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    test_crossover_panic!(order1_not_enough_genes, i32, Order1::with_preset_split(3..4),
                          parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                          parent(3, 3, 3, 3, 3, 3, 3, 3, 3, 3));

    bench_crossover!(order1_bench, i32, Order1::with_preset_split(3..8),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
