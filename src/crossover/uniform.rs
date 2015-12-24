use rand;

use crossover::Crossover;
use utility::RngExt;

#[derive(Copy, Clone)]
pub struct UniformCrossover {
    pub probability: f64
}

impl UniformCrossover {
    pub fn new(probability: f64) -> Self {
        UniformCrossover { probability: probability }
    }
}

impl<T> Crossover<T> for UniformCrossover where T: Clone {
    fn parents(&self) -> usize {
        2
    }

    fn children(&self) -> usize {
        2
    }

    fn cross<U>(&self, parents: &[U]) -> Vec<Vec<T>> where U: AsRef<[T]> {
        let mut rng = rand::thread_rng();
        let (parent1, parent2) = (parents[0].as_ref(), parents[1].as_ref());
        let length = parent1.len();
        let (mut child1, mut child2) = (Vec::with_capacity(length), Vec::with_capacity(length));

        for i in 0..length {
            if rng.happens(self.probability) {
                child1.push(parent2[i].clone());
                child2.push(parent1[i].clone());
            } else {
                child1.push(parent1[i].clone());
                child2.push(parent2[i].clone());
            }
        }

        vec![child1, child2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossover::Crossover;

    test_crossover!(uniform_cross_prob0, i32, UniformCrossover::new(0.0),
                    parent(0, 1,  2,  3,  4,  5,  6,  7),
                    parent(8, 9, 10, 11, 12, 13, 14, 15),

                    child(0, 1,  2,  3,  4,  5,  6,  7),
                    child(8, 9, 10, 11, 12, 13, 14, 15));

    test_crossover!(uniform_cross_prob1, i32, UniformCrossover::new(1.0),
                    parent(0, 1,  2,  3,  4,  5,  6,  7),
                    parent(8, 9, 10, 11, 12, 13, 14, 15),

                    child(8, 9, 10, 11, 12, 13, 14, 15),
                    child(0, 1,  2,  3,  4,  5,  6,  7));

    test_crossover_panic!(uniform_cross_different_length, i32, UniformCrossover::new(0.0),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    bench_crossover!(uniform_bench, i32, UniformCrossover::new(0.5),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    #[test]
    fn uniform_cross_prob05() {
        let length = 10000;
        let epsilon = 0.03;
        let parents: Vec<Vec<i32>> = vec![vec![0; length],
                                                 vec![1; length]];
        let uniform = UniformCrossover::new(0.5);
        let children = uniform.cross(&parents);

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
