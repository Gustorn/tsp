pub struct Chromosome<T> {
    pub genes: Vec<T>,
    pub fitness: f64,
}

impl<T> Chromosome<T> {
    pub fn new(genes: Vec<T>, fitness: f64) -> Self {
        Chromosome { genes: genes, fitness: fitness }
    }
}

forward_as!(Chromosome, T, genes);
forward_index!(Chromosome, T, genes);

