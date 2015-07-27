use std::ops;

use itertools::RepeatCall;
use rand::Rng;
use rand::distributions::IndependentSample;
use rand::distributions::range::{Range, SampleRange};

pub trait Problem<T> : Clone where T: Clone {
    fn generate_population(&self, size: usize) -> Vec<Vec<T>>;
}

pub trait UniformProblem<T> : Problem<T> where T: Clone {
    fn generate_gene(&self, index: usize) -> T;
}

#[derive(Clone, Copy)]
pub struct Numeric<T> where T: Clone + Copy + PartialOrd + SampleRange {
    pub length: usize,
    pub range: Range<T>,
}

#[derive(Clone)]
pub struct Permutation<T> where T: Clone {
    pub values: Vec<T>,
}

impl<T> Numeric<T> where T: Clone + Copy + PartialOrd + SampleRange {
    pub fn new(length: usize, (low, high): (T, T)) -> Self {
        Numeric {
            length: length,
            range: Range::new(low, high),
        }
    }
}

impl<T> Problem<T> for Numeric<T> where T: Clone + Copy + PartialOrd + SampleRange {
    fn generate_population(&self, size: usize) -> Vec<Vec<T>> {
        let mut rng = ::rand::thread_rng();
        let chromosomes = RepeatCall::new(|| {
            RepeatCall::new(|| self.range.ind_sample(&mut rng)).take(self.length).collect()
        });
        chromosomes.take(size).collect()
    }
}

impl<T> UniformProblem<T> for Numeric<T> where T: Clone + Copy + PartialOrd + SampleRange {
    fn generate_gene(&self, _: usize) -> T {
        let mut rng = ::rand::thread_rng();
        self.range.ind_sample(&mut rng)
    }
}

impl<T> From<Vec<T>> for Permutation<T> where T: Clone {
    fn from(values: Vec<T>) -> Self {
        Permutation {
            values: values,
        }
    }
}

impl<'a, T> From<&'a [T]> for Permutation<T> where T: Clone {
    fn from(values: &[T]) -> Self {
        Permutation {
            values: Vec::from(values.as_ref()),
        }
    }
}

macro_rules! from_integer_range {
    ($Int: ty) => {
        impl From<ops::Range<$Int>> for Permutation<$Int> {
            fn from(range: ops::Range<$Int>) -> Self {
                Permutation {
                    values: range.collect()
                }
            }
        }
    };
}

from_integer_range!(i8);
from_integer_range!(i16);
from_integer_range!(i32);
from_integer_range!(i64);
from_integer_range!(u8);
from_integer_range!(u16);
from_integer_range!(u32);
from_integer_range!(u64);
from_integer_range!(isize);
from_integer_range!(usize);


impl<T> Problem<T> for Permutation<T> where T: Clone {
    fn generate_population(&self, size: usize) -> Vec<Vec<T>> {
        let chromosomes = RepeatCall::new(|| {
            let mut genes = self.values.clone();
            ::rand::thread_rng().shuffle(&mut genes);
            genes
        });
        chromosomes.take(size).collect()
    }
}


