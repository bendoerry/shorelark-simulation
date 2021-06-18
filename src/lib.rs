#![feature(crate_visibility_modifier)]

use nalgebra as na;
use rand::Rng;

pub use crate::animal::Animal;
pub use crate::food::Food;
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
    pub fn step(&mut self, rng: &mut dyn rand::RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }

    fn process_collisions(&mut self, rng: &mut dyn rand::RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    food.position = rng.gen()
                }
            }
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
}
