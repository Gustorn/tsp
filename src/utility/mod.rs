#[macro_use]
pub mod macros;

pub mod adaptors;

use rand::Rng;
use self::adaptors::{Pairs, Triples};

pub trait Gather: Iterator where Self: Sized {
    fn pairs(self) -> Pairs<Self> where Self: Sized {
        Pairs::new(self)
    }

    fn triples(self) -> Triples<Self> where Self: Sized {
        Triples::new(self)
    }
}

impl<I> Gather for I where I: Iterator {}

pub trait RngExt: Rng where Self: Sized {
    fn choose1<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        self.choose(slice)
            .expect("Empty slices are unsupported for this operation")
    }

    fn independent_indexes<T>(&mut self, first: &[T], second: &[T]) -> (usize, usize) {
        (self.gen_range(0, first.len()), self.gen_range(0, second.len()))
    }

    fn index<T>(&mut self, slice: &[T]) -> usize {
        self.gen_range(0, slice.len())
    }

    fn range_indexes<T>(&mut self, slice: &[T]) -> (usize, usize) {
        let start = self.gen_range(0, slice.len() - 1);
        (start, self.gen_range(start + 1, slice.len()))
    }

    fn happens(&mut self, probability: f64) -> bool {
        self.next_f64() < probability
    }
}

impl<R> RngExt for R where R: RngExt {}
