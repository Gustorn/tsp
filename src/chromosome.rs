use std::cmp::Ordering;

pub struct Chromosome<T> {
    genes: Vec<T>,
    pub fitness: f64,
}

impl<T> Chromosome<T> {
    pub fn new(genes: Vec<T>, fitness: f64) -> Self {
        Chromosome { genes: genes, fitness: fitness }
    }

    pub fn genes(&self) -> &[T] {
        &self.genes
    }
}

forward_as!(Chromosome, T, genes);
forward_index!(Chromosome, T, genes);
forward_into_iter!(Chromosome, T, genes);

impl<T> Clone for Chromosome<T> where T: Clone {
    fn clone(&self) -> Self {
        Chromosome {
            genes: self.genes.clone(),
            fitness: self.fitness,
        }
    }
}

impl<T> From<Vec<T>> for Chromosome<T> {
    fn from(genes: Vec<T>) -> Self {
        Chromosome {
            genes: genes,
            fitness: 0.0,
        }
    }
}

impl<T> Eq for Chromosome<T> {}

// It is assumed that a chromosome's fitness may never be NaN, so Ord is implemented to allow the use of
// useful standard library functions (sort, max, min, etc.).
impl<T> Ord for Chromosome<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering
        } else {
            panic!("A Chromosome's fitness value may never be NaN")
        }
    }
}

impl<T> PartialEq for Chromosome<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> PartialOrd for Chromosome<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

