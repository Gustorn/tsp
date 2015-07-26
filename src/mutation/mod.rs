mod centre_inverse;
mod partial_shuffle;
mod reverse_sequence;
mod thrors;
mod uniform;

pub use self::centre_inverse::CentreInverse;
pub use self::partial_shuffle::PartialShuffle;
pub use self::reverse_sequence::ReverseSequence;
pub use self::thrors::Thrors;
pub use self::uniform::Uniform;

pub trait Mutation<T> {
    fn mutate(&self, genes: Vec<T>, mutation_rate: f64) -> Vec<T>;
}
