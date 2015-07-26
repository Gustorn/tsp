use rand::{self, Rng};
use crossover::Crossover;

#[derive(Clone, Copy)]
pub struct PrecedencePreservative;

impl PrecedencePreservative {
    pub fn new() -> Self {
        PrecedencePreservative
    }
}

impl<T> Crossover<T> for PrecedencePreservative where T: Clone + PartialEq {
    fn parents(&self) -> usize {
        2
    }

    fn cross(&self, parents: &[Vec<T>], crossover_rate: f64) -> Vec<Vec<T>> {
        assert!(parents.len() == Crossover::<T>::parents(self));
        assert!(parents[0].len() == parents[1].len());

        let mut rng = rand::thread_rng();
        pre_crossover!(rng, crossover_rate, parents[0], parents[1]);

        let (parent1, parent2) = (&parents[0], &parents[1]);
        let length = parent2.len();
        let (mut genes1, mut genes2) = (parent1.clone(), parent2.clone());
        let mut child = Vec::with_capacity(length);

        for _ in 0..length {
            if rng.gen_range(0, 2) == 0 {
                precedence_preservative(&mut genes1, &mut genes2, &mut child);
            } else {
                precedence_preservative(&mut genes2, &mut genes1, &mut child);
            }
        }
        vec![child]
    }
}



fn precedence_preservative<T>(p0: &mut Vec<T>, p1: &mut Vec<T>, c: &mut Vec<T>)
    where T: Clone + PartialEq {

    let gene = p0[0].clone();
    p0.retain(|g| *g != gene);
    p1.retain(|g| *g != gene);
    c.push(gene);
}

#[cfg(test)]
mod tests {
    test_crossover_panic!(precedence_preservative_different_length, i32,
                          PrecedencePreservative::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    test_crossover_passthrough!(precedence_preservative_passthrough, i32,
                                PrecedencePreservative::new(),
                                parent(0, 1,  2,  3,  4,  5,  6,  7),
                                parent(8, 9, 10, 11, 12, 13, 14, 15));

    bench_crossover!(precedence_preservative_bench, i32, PrecedencePreservative::new(),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
