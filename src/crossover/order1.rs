use std::ops::Range;
use rand;
use crossover::Crossover;
use utility::RngExt;

#[derive(Copy, Clone)]
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

    fn children(&self) -> usize {
        2
    }

    fn cross<U>(&self, parents: &[U]) -> Vec<Vec<T>> where U: AsRef<[T]> {
        let mut rng = rand::thread_rng();
        let (parent1, parent2) = (parents[0].as_ref(), parents[1].as_ref());
        let split = self.preset_split.unwrap_or(rng.range_indexes(parent1));

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
    child.extend(remaining.by_ref().take(split_start).cloned());
    child.extend(swath.iter().cloned());
    child.extend(remaining.cloned());

    child
}

#[cfg(test)]
mod tests {
    test_crossover!(order1_cross, i32, Order1::with_preset_split(3..8),
                    parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                    parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),

                    child(0, 4, 7, 3, 6, 2, 5, 1, 8, 9),
                    child(8, 2, 1, 3, 4, 5, 6, 7, 9, 0));

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
