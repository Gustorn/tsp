use std::ops::Range;

use rand;

use crossover::Crossover;
use utility::RngExt;

#[derive(Copy, Clone)]
pub struct TwoPoint {
    preset_split: Option<(usize, usize)>,
}

impl TwoPoint {
    pub fn new() -> Self {
        TwoPoint { preset_split: None }
    }

    pub fn with_preset_split(range: Range<usize>) -> Self {
        if range.start >= range.end {
            panic!("Incorrect range for Crossover operator split point");
        }
        TwoPoint { preset_split: Some((range.start, range.end)) }
    }
}

impl<T> Crossover<T> for TwoPoint where T: Clone {
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
        let (mid_start, mid_end) = self.preset_split.unwrap_or(rng.range_indexes(parent1));

        vec![join!(parent1[0..mid_start], parent2[mid_start..mid_end], parent1[mid_end..length]),
             join!(parent2[0..mid_start], parent1[mid_start..mid_end], parent2[mid_end..length])]
    }
}

#[cfg(test)]
mod tests {
    test_crossover!(two_point_cross, i32, TwoPoint::with_preset_split(2..5),
                    parent(0, 1,  2,  3,  4,  5,  6,  7),
                    parent(8, 9, 10, 11, 12, 13, 14, 15),

                    child(0, 1, 10, 11, 12,  5,  6,  7),
                    child(8, 9,  2,  3,  4, 13, 14, 15));

    test_crossover_panic!(two_point_different_length, i32, TwoPoint::new(),
                          parent(8, 4, 7, 3, 6, 2, 5, 1),
                          parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    bench_crossover!(two_point_bench, i32, TwoPoint::new(),
                     parent(8, 4, 7, 3, 6, 2, 5, 1, 9, 0),
                     parent(0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
}
