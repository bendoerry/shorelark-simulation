pub struct Simulation {
    world: World,
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

#[derive(Debug)]
pub struct Animal {
    position: Point2,
}

#[derive(Debug)]
pub struct Food {
    position: Point2,
}

#[derive(Debug)]
pub struct Point2 {
    x: f32,
    y: f32,
}
