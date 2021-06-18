use nalgebra as na;
use rand::Rng;

#[derive(Debug)]
pub struct Food {
    crate position: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}
