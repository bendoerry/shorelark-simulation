use lib_genetic_algorithm as ga;
use nalgebra as na;
use rand::Rng;

use self::{brain::Brain, eye::Eye};

pub use self::individual::AnimalIndividual;

mod brain;
mod eye;
mod individual;

#[derive(Debug)]
pub struct Animal {
    crate position: na::Point2<f32>,
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: Brain,
    /// Number of foods eaten by this animal
    crate satiation: usize,
}

impl Animal {
    fn new(eye: Eye, brain: Brain, rng: &mut dyn rand::RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    /// "Restores" bird from a chromosome.
    ///
    /// We have to have access to the PRNG in here, because our
    /// chromosomes encode only the brains - and while we restore the
    /// bird, we have to also randomize its position, direction, etc.
    /// (so it's stuff that wouldn't make sense to keep in the genome.)
    crate fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn rand::RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        // We evolve only our birds' brains, but technically there's no
        // reason not to simulate e.g. physical properties such as size.
        //
        // If that was to happen, this function could be adjusted to
        // return a longer chromosome that encodes not only the brain,
        // but also, say, birdie's color.

        self.brain.as_chromosome()
    }

    pub fn position(&self) -> na::Point2<f32> {
        // ------------------ ^
        // | No need to return a reference, because na::Point2 is Copy.
        // |
        // | (meaning: it's so small that cloning it is cheaper than
        // | messing with references.)
        // |
        // | Of course you don't have to memorize which types are Copy
        // | and which aren't - if you accidentally return a reference
        // | to a type that's Copy, rust-clippy will point it out and
        // | suggest a change :-)
        // ---

        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
