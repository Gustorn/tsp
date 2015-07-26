use std::collections::HashMap;
use std::hash::Hash;

use rand::{self, Rng};

use crossover::Crossover;

/// Builds the children using the cycles in the parent chromosomes.
///
/// Identifies cycles in the parent chromosomes, then to produce the children the genes of Parent 1
/// and Parent 2 are copied in an alternating manner, based on the cycles gathered:
/// - Child 1: the genes of Cycle 1 is copied from Parent 1, Cycle 2 from Parent 2, Cycle 3 from
/// Parent 1
/// - Child 2: Cycle 1 from Parent 2, Cycle 2 from Parent 1, Cycle 3 from Parent 2
///
/// A cycle is defined as the following:
/// 0. Let the value at the starting index in Parent 1 be the terminator value. The current index
///    is the starting index.
/// 1. The current value is the value of Parent 2 at the current index.
/// 2. If the current value is equal to the terminator, the cycle is complete.
/// 3. If not, look up the current value in Parent 1. Let its index be the current index, and jump
///    to 1.
#[derive(Clone, Copy)]
pub struct Cycle;

impl Cycle {
    pub fn new() -> Self {
        Cycle
    }
}

impl<T> Crossover<T> for Cycle where T: Clone + Eq + Hash {
    fn parents(&self) -> usize {
        2
    }

    fn cross(&self, parents: &[Vec<T>], crossover_rate: f64) -> Vec<Vec<T>> {
        assert!(parents.len() == Crossover::<T>::parents(self));
        assert!(parents[0].len() == parents[1].len());

        let mut rng = rand::thread_rng();
        pre_crossover!(rng, crossover_rate, parents[0], parents[1]);

        let (parent1, parent2) = (&parents[0], &parents[1]);
        let cycles = gather_cycles(parent1, parent2);
        let (mut child1, mut child2) = (parent1.clone(), parent2.clone());

        for (i, cycle) in cycles.iter().enumerate() {
            let from = if i % 2 == 0 {
                (parent1, parent2)
            } else {
                (parent2, parent1)
            };

            for j in cycle {
                child1[*j] = from.0[*j].clone();
                child2[*j] = from.1[*j].clone();
            }
        }
        vec![child1, child2]
    }
}

fn find_cycle<T>(starting_index: usize, parent1: &[T], parent2: &[T], lookup: &HashMap<&T, usize>) -> Vec<usize>
    where T: Clone + Eq + Hash {

    let mut cycle = Vec::new();
    let terminator = &parent1[starting_index];
    cycle.push(starting_index);

    let mut current = &parent2[starting_index];
    while *current != *terminator {
         let upper = *lookup.get(current)
             .expect("Cycle crossover is only supported for parents with \
                      an identical set of genes");
         current = &parent2[upper];
         cycle.push(upper);
    }
    cycle
}

fn gather_cycles<T>(parent1: &[T], parent2: &[T]) -> Vec<Vec<usize>>
    where T: Clone + Eq + Hash {

    let lookup = parent1.iter().enumerate().map(|(i, v)| (v, i)).collect::<HashMap<_, _>>();
    if lookup.len() != parent1.len() {
        panic!("Cycle crossover is only supported for parents with a unique set of genes");
    }

    let length = parent1.len();
    // This vectors tracks if an index has already been included in a cycle.
    let mut visit = vec![true; length];
    let mut cycles = Vec::new();

    for i in 0..length {
        if !visit[i] {
            continue;
        }

        let result = find_cycle(i, parent1, parent2, &lookup);
        for index in result.iter() {
            visit[*index] = false;
        }
        cycles.push(result);
    }
    cycles
}

#[cfg(test)]
mod tests {
    test_crossover!(cycle_cross, i32, Cycle::new(),
                    parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                    parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),

                    child(8, 1, 2, 3, 4, 5, 6, 7, 9, 0),
                    child(0, 4, 7, 3, 6, 2, 5, 1, 8, 9));

    test_crossover_passthrough!(cycle_passthrough, i32,
                                Cycle::new(),
                                parent(0, 1,  2,  3,  4,  5,  6,  7),
                                parent(8, 9, 10, 11, 12, 13, 14, 15));

    test_crossover_panic!(cycle_different_length, i32, Cycle::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    test_crossover_panic!(cycle_different_gene_sets, i32, Cycle::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 100),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    bench_crossover!(cycle_bench, i32, Cycle::new(),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}

