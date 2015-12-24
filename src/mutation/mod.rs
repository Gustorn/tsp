mod centre_inverse;
mod flip_bit;
mod partial_shuffle;
mod reverse_sequence;
mod shuffle;
mod thrors;
mod twors;
mod uniform;

pub use self::centre_inverse::CentreInverse;
pub use self::flip_bit::FlipBit;
pub use self::partial_shuffle::PartialShuffle;
pub use self::reverse_sequence::ReverseSequence;
pub use self::shuffle::Shuffle;
pub use self::thrors::Thrors;
pub use self::twors::Twors;
pub use self::uniform::UniformMutation;

pub trait Mutation<T> {
    fn mutate(&self, genes: Vec<T>, mutation_rate: f64) -> Vec<T>;
}
