use lib_neural_network as nn;
use nalgebra as na;
use rand::Rng;

use self::eye::Eye;

pub use self::individual::AnimalIndividual;

mod eye;
mod individual;

#[derive(Debug)]
pub struct Animal {
    crate position: na::Point2<f32>,
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: nn::Network,
}

impl Animal {
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        let eye = Eye::default();

        let brain = nn::Network::random(
            rng,
            &[
                nn::LayerTopology {
                    neurons: eye.cells(),
                },
                nn::LayerTopology {
                    neurons: 2 * eye.cells(),
                },
                nn::LayerTopology { neurons: 2 },
            ],
        );

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
        }
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
