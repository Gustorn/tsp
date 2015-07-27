pub struct Pairs<I> where I: Iterator {
    iter: I,
}

impl<I> Pairs<I> where I: Iterator {
    pub fn new(iter: I) -> Self {
        Pairs { iter: iter }
    }
}

impl<I> Iterator for Pairs<I> where I: Iterator {
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let a = match self.iter.next() {
            None => return None,
            Some(a) => a,
        };

        match self.iter.next() {
            None => None,
            Some(b) => Some((a, b)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min, max) = self.iter.size_hint();

        if let Some(max) = max {
            (min / 2 + 1, Some(max / 2 + 1))
        } else {
            (min / 2 + 1, None)
        }
    }
}

pub struct Triples<I> where I: Iterator {
    iter: I,
}

impl<I> Triples<I> where I: Iterator {
    pub fn new(iter: I) -> Self {
        Triples { iter: iter }
    }
}

impl<I> Iterator for Triples<I> where I: Iterator {
    type Item = (I::Item, I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let a = match self.iter.next() {
            None => return None,
            Some(a) => a,
        };

        let b = match self.iter.next() {
            None => return None,
            Some(b) => b,
        };

        match self.iter.next() {
            None => None,
            Some(c) => Some((a, b, c)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min, max) = self.iter.size_hint();

        if let Some(max) = max {
            (min / 3 + 1, Some(max / 3 + 1))
        } else {
            (min / 3 + 1, None)
        }
    }
}

