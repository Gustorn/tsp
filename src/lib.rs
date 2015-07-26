#![feature(plugin)]
#![feature(test)]
#![plugin(clippy)]


#[cfg(test)] extern crate approx;
extern crate itertools;
extern crate linear_map;
extern crate rand;
extern crate test;

#[macro_use] mod utility;
mod chromosome;
mod generation;
mod static_algorithm;

pub mod crossover;
pub mod mutation;
pub mod problem;
pub mod reinsertion;
pub mod selection;
pub mod termination;

pub use crossover::Crossover;
pub use mutation::Mutation;
pub use problem::{Problem, UniformProblem};
pub use reinsertion::Reinsertion;
pub use selection::Selection;
pub use static_algorithm::StaticAlgorithm;
pub use termination::Termination;
