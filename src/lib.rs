#![feature(crate_visibility_modifier)]

pub use crate::animal::Animal;
pub use crate::world::World;

mod animal;
mod food;
mod world;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}
