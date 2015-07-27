#![feature(plugin)]
#![feature(test)]
#![plugin(clippy)]


extern crate approx;
extern crate itertools;
extern crate linear_map;
extern crate rand;
extern crate test;

#[macro_use]
mod utility;

mod chromosome;
mod generation;

#[macro_use]
pub mod algorithm;

pub mod crossover;
pub mod mutation;
pub mod problem;
pub mod reinsertion;
pub mod selection;
pub mod termination;

pub use algorithm::Algorithm;
pub use crossover::Crossover;
pub use mutation::Mutation;
pub use problem::{Problem, UniformProblem, Numeric, Permutation};
pub use reinsertion::Reinsertion;
pub use selection::Selection;
pub use termination::Termination;
