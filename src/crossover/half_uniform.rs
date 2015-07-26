use rand::{self, Rng};
use crossover::Crossover;

/// Swaps exactly half of the number of different genes between the parents.
#[derive(Clone, Copy)]
pub struct HalfUniform;

impl HalfUniform {
    pub fn new() -> Self {
        HalfUniform
    }
}

impl<T> Crossover<T> for HalfUniform where T: Clone + PartialEq {
    fn parents(&self) -> usize {
        2
    }

    fn cross(&self, parents: &[Vec<T>], crossover_rate: f64) -> Vec<Vec<T>> {
        assert!(parents.len() == Crossover::<T>::parents(self));
        assert!(parents[0].len() == parents[1].len());

        let mut rng = rand::thread_rng();
        pre_crossover!(rng, crossover_rate, parents[0], parents[1]);

        let (parent1, parent2) = (&parents[0], &parents[1]);
        let (mut child1, mut child2) = (parent1.clone(), parent2.clone());
        let mut indices = parent1.iter().enumerate()
            .zip(parent2.iter())
            .filter_map(|((i, p1), p2)| {
                if p1 != p2 {
                    Some(i)
                } else {
                    None
                }})
            .collect::<Vec<_>>();

        rng.shuffle(indices.as_mut());
        for i in 0..(indices.len() / 2) {
            child1[i] = parent2[i].clone();
            child2[i] = parent1[i].clone();
        }
        vec![child1, child2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossover::Crossover;

    test_crossover_panic!(half_uniform_different_length, i32, HalfUniform::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    test_crossover_passthrough!(half_uniform_passthrough, i32,
                                HalfUniform::new(),
                                parent(0, 1,  2,  3,  4,  5,  6,  7),
                                parent(8, 9, 10, 11, 12, 13, 14, 15));

    bench_crossover!(half_uniform_bench, i32, HalfUniform::new(),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    #[test]
    fn half_uniform_cross() {
        let length = 10000;
        let epsilon = 0.01;
        // By using Chromosomes with genes that differ in every position this will act as a
        // less effective version of the Uniform crossover method with a probabilty of 0.5
        let parents: Vec<Vec<i32>> = vec![vec![0; length],
                                          vec![1; length]];

        let half_uniform = HalfUniform::new();
        let children = half_uniform.cross(&parents, 1.0);

        let check_genes = |child: &Vec<i32>| {
            let zeros = child.iter().fold(0, |acc, x| if *x == 0 { acc + 1 } else { acc });
            let ones = child.iter().fold(0, |acc, x| if *x == 1 { acc + 1 } else { acc });
            let test0 = zeros as f64 / length as f64;
            let test1 = ones as f64 / length as f64;
            assert_approx_eq!(test0, 0.5, epsilon);
            assert_approx_eq!(test1, 0.5, epsilon);
        };
        check_genes(&children[0]);
        check_genes(&children[1]);
    }
}
