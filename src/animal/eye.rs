use nalgebra as na;
use std::f32::consts::{FRAC_PI_4, PI};

use crate::Food;

/// How far our eye can see:
///
/// -----------------
/// |               |
/// |               |
/// |               |
/// |@      %      %|
/// |               |
/// |               |
/// |               |
/// -----------------
///
/// If @ marks our birdie and % marks food, then a FOV_RANGE of:
///
/// - 0.1 = 10% of the map = bird sees no foods (at least in this case)
/// - 0.5 = 50% of the map = bird sees one of the foods
/// - 1.0 = 100% of the map = bird sees both foods
const FOV_RANGE: f32 = 0.25;

/// How wide our eye can see.
///
/// If @> marks our birdie (rotated to the right) and . marks the area
/// our birdie sees, then a FOV_ANGLE of:
///
/// - PI/2 = 90° =
///   -----------------
///   |             /.|
///   |           /...|
///   |         /.....|
///   |       @>......|
///   |         \.....|
///   |           \...|
///   |             \.|
///   -----------------
///
/// - PI = 180° =
///   -----------------
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   |       @>......|
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   -----------------
///
/// - 2 * PI = 360° =
///   -----------------
///   |...............|
///   |...............|
///   |...............|
///   |.......@>......|
///   |...............|
///   |...............|
///   |...............|
///   -----------------
///
/// Field of view depends on both FOV_RANGE and FOV_ANGLE:
///
/// - FOV_RANGE=0.4, FOV_ANGLE=PI/2:
///   -----------------
///   |       @       |
///   |     /.v.\     |
///   |   /.......\   |
///   |   ---------   |
///   |               |
///   |               |
///   |               |
///   -----------------
///
/// - FOV_RANGE=0.5, FOV_ANGLE=2*PI:
///   -----------------
///   |               |
///   |      ---      |
///   |     /...\     |
///   |    |..@..|    |
///   |     \.../     |
///   |      ---      |
///   |               |
///   -----------------
const FOV_ANGLE: f32 = PI + FRAC_PI_4;

/// How much photoreceptors there are in a single eye.
///
/// More cells means our birds will have more "crisp" vision, allowing
/// them to locate the food more precisely - but the trade-off is that
/// the evolution process will then take longer, or even fail, unable
/// to find any solution.
///
/// I've found values between 3~11 sufficient, with eyes having more
/// than ~20 photoreceptors yielding progressively worse results.
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    // FOV_RANGE, FOV_ANGLE & CELLS are the values we'll use during
    // simulation - but being able to create an arbitrary eye will
    // come handy during the testing:
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = self.food_vec(position, food);
            let dist = vec.norm();
            let angle = self.food_angle(rotation, vec);

            if !self.visible_food(dist, angle) {
                continue;
            }

            let cell = angle / self.fov_angle * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;

            cells[cell] += energy;
        }

        cells
    }

    fn food_vec(&self, position: na::Point2<f32>, food: &Food) -> na::Vector2<f32> {
        food.position - position
    }

    fn food_angle(&self, rotation: na::Rotation2<f32>, vec: na::Vector2<f32>) -> f32 {
        let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
        let angle = angle - rotation.angle();
        let angle = na::wrap(angle, -PI, PI);
        let angle = angle + self.fov_angle / 2.0;

        angle
    }

    fn visible_food(&self, dist: f32, angle: f32) -> bool {
        dist < self.fov_range && angle >= 0.0 && angle <= self.fov_angle
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use crate::food::Food;
    use nalgebra as na;

    use super::Eye;

    /// All our tests will use eyes hard-coded to thirteen eye cells.
    ///
    /// As for the "why":
    ///
    /// While we certainly *could* implement tests for different number of
    /// eye cells, after a while I've decided it's just not worth the
    /// hassle - as you'll see in a moment, we'll already get a good coverage
    /// via the other parameters, so creating a separate set of tests for
    /// different values of eye cells seemed like a waste of time.
    ///
    /// As for the "why this number in particular":
    ///
    /// I've checked a few numbers by hand, and generally found 13 to yield
    /// pretty good results. As always, nothing special about 13 in
    /// particular, your (eye) mileage may vary.
    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );

            // The finish line!
            assert_eq!(convert_vision(actual_vision), self.expected_vision);
        }
    }

    fn convert_vision(vision: Vec<f32>) -> String {
        let vision: Vec<_> = vision
            .into_iter()
            .map(|cell| {
                // As a reminder, the higher cell's value, the closer
                // the food is:

                if cell >= 0.7 {
                    // <0.7, 1.0>
                    // food is right in front of us
                    "#"
                } else if cell >= 0.3 {
                    // <0.3, 0.7)
                    // food is somewhat further
                    "+"
                } else if cell > 0.0 {
                    // <0.0, 0.3)
                    // food is pretty far away
                    "."
                } else {
                    // 0.0
                    // no food in sight, this cell sees empty space
                    " "
                }
            })
            .collect();

        // As before, there's nothing special about the cell values
        // (`0.7`, `0.3`, `0.0`) or the characters (`#`, `+`, `.`).
        //
        // I've chosen hash because to my eye it seems to occupy the
        // most "visual space" out of all the ASCII characters (thus
        // it represents a food being close), and then plus and dot
        // are just smaller (representing food being further away).

        // `.join()` converts `Vec<String>` into `String` using a
        // separator - e.g. `vec!["a", "b", "c"].join("|")` would
        // return `a|b|c`.
        vision.join("")
    }

    /// A helper-function that allows to create food easily
    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    mod different_fov_ranges {
        use test_case::test_case;

        use super::TestCase;

        #[test_case(1.0, "not sure yet")]
        #[test_case(0.5, "not sure yet")]
        #[test_case(0.1, "not sure yet")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: todo!(),
                fov_angle: todo!(),
                x: todo!(),
                y: todo!(),
                rot: todo!(),
                fov_range,
                expected_vision,
            }
            .run()
        }
    }
}
