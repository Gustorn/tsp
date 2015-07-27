use crossover::Crossover;

#[derive(Copy, Clone)]
pub struct ThreeParent;

impl ThreeParent {
    pub fn new() -> Self {
        ThreeParent
    }
}

impl<T> Crossover<T> for ThreeParent where T: Clone + PartialEq {
    fn parents(&self) -> usize {
        3
    }

    fn children(&self) -> usize {
        1
    }

    fn cross<U>(&self, parents: &[U]) -> Vec<Vec<T>> where U: AsRef<[T]> {
        let (parent1, parent2, parent3) = (parents[0].as_ref(),
                                           parents[1].as_ref(),
                                           parents[2].as_ref());
        let genes = parent1.iter()
            .zip(parent2.iter())
            .zip(parent3.iter())
            .map(|((p1, p2), p3)| {
                if *p1 == *p2 {
                    p1.clone()
                } else {
                    p3.clone()
                }
            });
        vec![genes.collect()]
    }
}

#[cfg(test)]
mod tests {
    test_crossover!(three_parent_cross, i32, ThreeParent::new(),
                    parent(0, 0, 1, 1, 0, 0, 1, 1),
                    parent(0, 1, 0, 1, 0, 1, 0, 1),
                    parent(0, 0, 0, 0, 1, 1, 1, 1),

                    child(0, 0, 0, 1, 0, 1, 1, 1));

    test_crossover_panic!(three_parent_different_length, i32, ThreeParent::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
                          parent(1, 2, 3));

    bench_crossover!(three_parent_bench, i32, ThreeParent::new(),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(8, 6, 2, 2, 9, 5, 3, 1, 8, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
