use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

use rand;

use crossover::Crossover;
use utility::RngExt;

#[derive(Copy, Clone)]
pub struct PartiallyMapped {
    preset_split: Option<(usize, usize)>,
}

impl PartiallyMapped {
    pub fn new() -> Self {
        PartiallyMapped { preset_split: None }
    }

    pub fn with_preset_split(range: Range<usize>) -> Self {
        if range.start >= range.end {
            panic!("Incorrect range for crossover preset split point");
        }
        PartiallyMapped { preset_split: Some((range.start, range.end)) }
    }
}

impl<T> Crossover<T> for PartiallyMapped where T: Clone + Eq + Hash {
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

        vec![partially_mapped(parent1, parent2, split),
             partially_mapped(parent2, parent1, split)]
    }
}

fn partially_mapped<T>(parent1: &[T], parent2: &[T],
                (split_start, split_end): (usize, usize)) -> Vec<T>
    where T: Clone + Eq + Hash {

    let length = parent1.len();
    let lookup = parent2.iter().enumerate().map(|(i, v)| (v, i)).collect::<HashMap<_,_>>();
    if lookup.len() != length {
        panic!("Partially Mapped crossover is only supported for \
                parents with a unique set of genes");
    }

    let swath = &parent1[split_start..split_end];
    let mut child: Vec<_> = parent1.into();
    let mut visit = vec![true; length];

    for i in split_start..split_end {
        if swath.contains(&parent2[i]) {
            visit[i] = false;
        }
    }

    for i in split_start..split_end {
        if !visit[i] {
            continue;
        }

        let mut index = i;
        let value = parent2[index].clone();

        while index >= split_start && index < split_end {
            visit[i] = false;
            let new_index = *lookup.get(&parent1[index])
                .expect("Partially Mapped crossover is only supported for parents \
                         with an identical set of genes");
            if new_index == index {
                panic!("Partially Mapped crossover encountered an infinite loop. \
                        It's very likely there's something wrong with the parent Chromosomes.");
            }
            index = new_index;
        }
        child[index] = value;
        visit[index] = false;
    }

    for i in (0..length).filter(|i| visit[*i]) {
        child[i] = parent2[i].clone();
    }

    child
}

#[cfg(test)]
mod tests {
    test_crossover!(partially_mapped_cross, i32,
                    PartiallyMapped::with_preset_split(3..8),
                    parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                    parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),

                    child(0, 7, 4, 3, 6, 2, 5, 1, 8, 9),
                    child(8, 2, 1, 3, 4, 5, 6, 7, 9, 0));

    test_crossover_panic!(partially_mapped_different_length, i32, PartiallyMapped::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    test_crossover_panic!(partially_mapped_different_gene_sets, i32,
                          PartiallyMapped::with_preset_split(3..4),
                          parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                          parent(3, 3, 3, 3, 3, 3, 3, 3, 3, 3));

    bench_crossover!(partially_mapped_bench, i32,
                     PartiallyMapped::with_preset_split(3..8),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
