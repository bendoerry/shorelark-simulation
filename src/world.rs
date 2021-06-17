use crate::animal::Animal;
use crate::food::Food;

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}
