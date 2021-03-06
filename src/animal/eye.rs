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
        use super::{food, TestCase};
        use std::f32::consts::FRAC_PI_2;
        use test_case::test_case;

        /// During tests in this module, we're using a world that looks
        /// like this:
        ///
        /// ------------
        /// |          |
        /// |          |
        /// |    @>   %|
        /// |          |
        /// |          |
        /// ------------
        ///
        /// Each test gradually reduces our birdie's field of view and
        /// compares what the birdie sees:
        ///
        /// ------------
        /// |        /.|
        /// |      /...|
        /// |    @>...%|
        /// |      \...|
        /// |        \.|
        /// ------------
        ///
        /// ------------
        /// |          |
        /// |      /.| |
        /// |    @>..|%|
        /// |      \.| |
        /// |          |
        /// ------------
        ///
        /// ------------
        /// |          |
        /// |          |
        /// |    @>.| %|
        /// |          |
        /// |          |
        /// ------------
        ///
        /// Over time, what we see is the food gradually disappearing
        /// into an emptiness:
        ///
        /// (well, technically the food and bird remain stationary - it's
        /// only the birdie's own field of view that gets reduced.)
        #[test_case(1.0, "      +      ")] // Food is inside the FOV
        #[test_case(0.9, "      +      ")] // ditto
        #[test_case(0.8, "      +      ")] // ditto
        #[test_case(0.7, "      .      ")] // Food slowly disappears
        #[test_case(0.6, "      .      ")] // ditto
        #[test_case(0.5, "             ")] // Food disappeared!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn with_range(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.5)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_range,
                expected_vision,
            }
            .run()
        }
    }

    mod different_rotations {
        use super::{food, TestCase};
        use std::f32::consts::PI;
        use test_case::test_case;

        /// World:
        ///
        /// ------------
        /// |          |
        /// |          |
        /// |    @>    |
        /// |          |
        /// |         %|
        /// ------------
        ///
        /// Test cases:
        ///
        /// ------------
        /// |..........|
        /// |..........|
        /// |....@>....|
        /// |..........|
        /// |.........%|
        /// ------------
        ///
        /// ------------
        /// |..........|
        /// |..........|
        /// |....@.....|
        /// |....v.....|
        /// |.........%|
        /// ------------
        ///
        /// ------------
        /// |..........|
        /// |..........|
        /// |...<@.....|
        /// |..........|
        /// |.........%|
        /// ------------
        ///
        /// ... and so on, until we do a full circle, 360° rotation:
        #[test_case(0.00 * PI, "         +   ")] // Food is to our right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")]
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")] // Food is behind us
        #[test_case(1.25 * PI, " +           ")] // (we continue to see it
        #[test_case(1.50 * PI, "            +")] // due to 360° fov_angle.)
        #[test_case(1.75 * PI, "           + ")]
        #[test_case(2.00 * PI, "         +   ")] // Here we've done 360°
        #[test_case(2.25 * PI, "        +    ")] // (and a bit more, to
        #[test_case(2.50 * PI, "      +      ")] // prove the numbers wrap.)
        fn with_rotation(rot: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected_vision,
            }
            .run()
        }
    }

    mod different_positions {
        use super::{food, TestCase};
        use std::f32::consts::FRAC_PI_2;
        use test_case::test_case;

        /// World:
        ///
        /// ------------
        /// |          |
        /// |         %|
        /// |          |
        /// |         %|
        /// |          |
        /// ------------
        ///
        /// Test cases for the X axis:
        ///
        /// ------------
        /// |          |
        /// |        /%|
        /// |       @>.|
        /// |        \%|
        /// |          |
        /// ------------
        ///
        /// ------------
        /// |        /.|
        /// |      /..%|
        /// |     @>...|
        /// |      \..%|
        /// |        \.|
        /// ------------
        ///
        /// ... and so on, going further left
        ///     (or, from the bird's point of view - going _back_)
        ///
        /// Test cases for the Y axis:
        ///
        /// ------------
        /// |     @>...|
        /// |       \.%|
        /// |         \|
        /// |         %|
        /// |          |
        /// ------------
        ///
        /// ------------
        /// |      /...|
        /// |     @>..%|
        /// |      \...|
        /// |        \%|
        /// |          |
        /// ------------
        ///
        /// ... and so on, going further down
        ///     (or, from the bird's point of view - going _right_)

        // Checking the X axis:
        // (you can see the bird is "flying away" from the foods)
        #[test_case(0.9, 0.5, "#           #")]
        #[test_case(0.8, 0.5, "  #       #  ")]
        #[test_case(0.7, 0.5, "   +     +   ")]
        #[test_case(0.6, 0.5, "    +   +    ")]
        #[test_case(0.5, 0.5, "    +   +    ")]
        #[test_case(0.4, 0.5, "     + +     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]
        //
        // Checking the Y axis:
        // (you can see the bird is "flying alongside" the foods)
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn with_position(x: f32, y: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
                fov_range: 1.0,
                fov_angle: FRAC_PI_2,
                rot: 0.0,
                x,
                y,
                expected_vision,
            }
            .run()
        }
    }

    mod different_fov_angles {
        use super::{food, TestCase};
        use std::f32::consts::PI;
        use test_case::test_case;

        /// World:
        ///
        /// ------------
        /// |%  %  %  %|
        /// |          |
        /// |    @>    |
        /// |          |
        /// |%  %  %  %|
        /// ------------
        ///
        /// Test cases:
        ///
        /// ------------
        /// |%  %  %/.%|
        /// |      /...|
        /// |    @>....|
        /// |      \...|
        /// |%  %  %\.%|
        /// ------------
        ///
        /// ------------
        /// |%  %|.%..%|
        /// |    |.....|
        /// |    @>....|
        /// |    |.....|
        /// |%  %|.%..%|
        /// ------------
        ///
        /// ... and so on, until we reach the full, 360° FOV
        #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
        #[test_case(0.50 * PI, ".  +     +  .")]
        #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
        #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
        #[test_case(1.25 * PI, "   . + + .   ")]
        #[test_case(1.50 * PI, ".   .+ +.   .")]
        #[test_case(1.75 * PI, ".   .+ +.   .")]
        #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is wide = 8 foods
        fn with_angle(fov_angle: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_angle,
                expected_vision,
            }
            .run()
        }
    }
}
