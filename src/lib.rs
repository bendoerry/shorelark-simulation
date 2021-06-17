use nalgebra as na;
// --------- ^^
// | This kind of import - one that uses `as` - is called an alias.
// | You'd say that we're aliasing `nalgebra` as `na`.
// ---

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
    position: na::Point2<f32>,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}
