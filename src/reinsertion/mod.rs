use generation::Generation;

pub trait ReinsertIterator<T> : Iterator<Item = Vec<T>> {
    fn reinsert<R>(self, reinsertion: &R, parents: &Generation<T>) -> Generation<T>
        where Self: Sized, R: Reinsertion<T> {
        reinsertion.reinsert(parents, Vec::new()) //self.collect())
    }
}

impl<T, I> ReinsertIterator<T> for I where I: Iterator<Item = Vec<T>> {}

pub trait Reinsertion<T> {
    fn reinsert(&self, parents: &Generation<T>, offspring: Vec<T>) -> Generation<T>;
}
