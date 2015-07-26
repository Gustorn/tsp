#[macro_use] pub mod macros;
pub mod adaptors;

use self::adaptors::{Pairs, Triples};

pub trait Gather: Iterator {
    fn pairs(self) -> Pairs<Self> where Self: Sized {
        Pairs::new(self)
    }

    fn triples(self) -> Triples<Self> where Self: Sized {
        Triples::new(self)
    }
}

impl<I> Gather for I where I: Iterator {}

