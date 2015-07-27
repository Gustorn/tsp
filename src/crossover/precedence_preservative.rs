use rand::{self, Rng};
use crossover::Crossover;

#[derive(Copy, Clone)]
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

    fn children(&self) -> usize {
        1
    }

    fn cross<U>(&self, parents: &[U]) -> Vec<Vec<T>> where U: AsRef<[T]> {
        let mut rng = rand::thread_rng();
        let (parent1, parent2) = (parents[0].as_ref(), parents[1].as_ref());
        let length = parent2.len();
        let (mut genes1, mut genes2) = (parent1.into(), parent2.into());
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

    bench_crossover!(precedence_preservative_bench, i32, PrecedencePreservative::new(),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
