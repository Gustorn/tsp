use std::collections::HashMap;
use std::hash::Hash;

use crossover::Crossover;

#[derive(Copy, Clone)]
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

    fn children(&self) -> usize {
        2
    }

    fn cross<U>(&self, parents: &[U]) -> Vec<Vec<T>> where U: AsRef<[T]> {
        let (parent1, parent2) = (parents[0].as_ref(), parents[1].as_ref());
        let cycles = gather_cycles(parent1, parent2);
        let (mut child1, mut child2) = (Vec::from(parent1), Vec::from(parent2));

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

#[cfg(test)]
mod tests {
    test_crossover!(cycle_cross, i32, Cycle::new(),
                    parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                    parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),

                    child(8, 1, 2, 3, 4, 5, 6, 7, 9, 0),
                    child(0, 4, 7, 3, 6, 2, 5, 1, 8, 9));

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

