use lib_genetic_algorithm as ga;

pub struct AnimalIndividual;

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        todo!();
    }

    fn chromosome(&self) -> &ga::Chromosome {
        todo!()
    }

    fn fitness(&self) -> f32 {
        todo!();
    }
}
