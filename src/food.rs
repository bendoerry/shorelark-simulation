use nalgebra as na;
use rand::Rng;

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }
}
