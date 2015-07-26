mod epoch;

pub use self::epoch::Epoch;

pub trait Termination {
    fn reached(&mut self) -> bool;
}
