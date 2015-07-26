#[macro_use]
mod macros;

mod cut_splice;
mod cycle;
mod edge_recombination;
mod half_uniform;
mod one_point;
mod order1;
mod partially_mapped;
mod precedence_preservative;
mod three_parent;
mod two_point;
mod uniform;

pub use self::cut_splice::CutAndSplice;
pub use self::cycle::Cycle;
pub use self::edge_recombination::EdgeRecombination;
pub use self::half_uniform::HalfUniform;
pub use self::one_point::OnePoint;
pub use self::order1::Order1;
pub use self::partially_mapped::PartiallyMapped;
pub use self::precedence_preservative::PrecedencePreservative;
pub use self::three_parent::ThreeParent;
pub use self::two_point::TwoPoint;
pub use self::uniform::Uniform;

pub trait Crossover<T> {
    fn parents(&self) -> usize;
    fn cross(&self, parents: &[Vec<T>], crossover_rate: f64) -> Vec<Vec<T>>;
}

