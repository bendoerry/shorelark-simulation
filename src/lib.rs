#![feature(crate_visibility_modifier)]

use nalgebra as na;

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

    /// Performs a single step - a single second, so to say - of our
    /// simulation.
    pub fn step(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);
        }
    }
}
