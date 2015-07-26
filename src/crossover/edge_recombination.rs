use std::hash::Hash;

use itertools::Itertools;
use linear_map::LinearMap;
use rand::{self, Rng};

use crossover::Crossover;

/// The [Edge Recombination](https://en.wikipedia.org/wiki/Edge_recombination_operator) creates a path that is similar to that of the parents, by adding as few new edges as possible.
///
/// This usually performs very well, but it's a relatively expensive operator.
#[derive(Clone, Copy)]
pub struct EdgeRecombination;

impl EdgeRecombination {
    pub fn new() -> Self {
        EdgeRecombination
    }
}

impl<T> Crossover<T> for EdgeRecombination where T: Clone + Eq + Hash {
    fn parents(&self) -> usize {
        2
    }

    fn cross(&self, parents: &[Vec<T>], crossover_rate: f64) -> Vec<Vec<T>> {
        assert!(parents.len() == Crossover::<T>::parents(self));
        assert!(parents[0].len() == parents[1].len());
        assert!(parents[0].len() > 2);

        let mut rng = rand::thread_rng();
        pre_crossover!(rng, crossover_rate, parents[0], parents[1]);

        let (parent1, parent2) = (&parents[0], &parents[1]);
        let length = parent1.len();
        let adjacency = adjacency_matrix(parent1, parent2);

        let start0 = rng.choose(parent1)
            .expect("Edge Recombination is not supported on empty parents");
        let start1 = rng.choose(parent2)
            .expect("Edge Recombination is not supported on empty parents");

        vec![edge_recombination(start0, length, &adjacency, &mut rng),
             edge_recombination(start1, length, &adjacency, &mut rng)]
    }
}

fn edge_recombination<T, R>(start: &T, length: usize, adjacency: &LinearMap<T, Vec<T>>, rng: &mut R) -> Vec<T>
    where T: Clone + Eq + Hash,
          R: Rng {

    let mut adjacency = adjacency.clone();
    let mut child = Vec::with_capacity(length);
    let mut gene = start.clone();

    loop {
        child.push(gene.clone());

        if child.len() >= length {
            break;
        }

        if adjacency.is_empty() {
            panic!("Edge Recombination crossover ran out of genes to use. \
                    This is very likely the result of incorrect parents")
        }

        adjacency.iter_mut().foreach(|(_, mut n)| { n.retain(|g| *g != gene); });
        if !adjacency[&gene].is_empty() {
            let previous = gene.clone();
            gene = best_neighbor(&gene, &adjacency, rng);
            adjacency.remove(&previous);
        } else {
            adjacency.remove(&gene);
            gene = random_neighbor(&adjacency, rng);
        }
    }

    child
}

fn best_neighbor<T, R>(current: &T, adjacency_matrix: &LinearMap<T, Vec<T>>, rng: &mut R) -> T
    where T: Clone + Eq + Hash,
          R: Rng {

    if adjacency_matrix[current].len() == 1 {
        adjacency_matrix[current].iter().next().unwrap().clone()
    } else {
        let sorted = adjacency_matrix[current].iter()
            .map(|n| (n, adjacency_matrix[n].len()))
            .sort_by(|a, b| Ord::cmp(&a.1, &b.1));
        let min = sorted[0].1;
        let min_group = sorted.into_iter()
            .group_by(|a| a.1 == min)
            .map(|(_, v)| v)
            .next()
            .unwrap();
        rng.choose(min_group.as_ref()).unwrap().0.clone()
    }
}

fn random_neighbor<T, R>(adjacency_matrix: &LinearMap<T, Vec<T>>, rng: &mut R) -> T
    where T: Clone + Eq + Hash,
          R: Rng {
    let length = adjacency_matrix.len();
    adjacency_matrix.keys()
        .nth(rng.gen_range(0, length))
        .expect("Edge Recombination crossover ran out of genes to use. \
                 This is very likely the result of incorrect parents")
        .clone()
}

fn neighbors<T>(parent: &[T], index: usize) -> (T, T)
    where T: Clone {

    let last = parent.len() - 1;
    if index == 0 {
        (parent[last].clone(), parent[index + 1].clone())
    } else if index == last {
        (parent[index - 1].clone(), parent[0].clone())
    } else {
        (parent[index - 1].clone(), parent[index + 1].clone())
    }
}

fn adjacency_matrix<T>(parent1: &[T], parent2: &[T]) -> LinearMap<T, Vec<T>>
    where T: Clone + Eq + Hash {

    let mut ret: LinearMap<T, Vec<T>> = LinearMap::with_capacity(parent1.len());
    let length = parent1.len();

    for i in 0..length {
        let n = neighbors(parent1, i);
        ret.insert(parent1[i].clone(), vec![n.0, n.1]);
    }

    for i in 0..length {
        let n = neighbors(parent2, i);
        let p = &parent2[i];
        let row = ret.get_mut(p).unwrap();
        if !row.contains(&n.0) {
            row.push(n.0);
        }
        if !row.contains(&n.1) {
            row.push(n.1);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    test_crossover_passthrough!(edge_recombination_passthrough, i32,
                                EdgeRecombination::new(),
                                parent(0, 1,  2,  3,  4,  5,  6,  7),
                                parent(8, 9, 10, 11, 12, 13, 14, 15));

    bench_crossover!(edge_recombination_bench, i32, EdgeRecombination::new(),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}

