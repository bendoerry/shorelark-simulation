use lib_genetic_algorithm as ga;
use lib_neural_network as nn;

use super::eye::Eye;

#[derive(Debug)]
pub struct Brain {
    crate nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn rand::RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
