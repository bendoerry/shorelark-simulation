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
            let vec = food.position - position;
            let dist = vec.norm();

            if dist >= self.fov_range {
                continue;
            }

            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            // Makes angle *relative* to our birdie's field of view - that is:
            // transforms it from <-FOV_ANGLE/2,+FOV_ANGLE/2> to <0,FOV_ANGLE>.
            //
            // After this operation:
            // - an angle of 0° means "the beginning of the FOV",
            // - an angle of self.fov_angle means "the ending of the FOV".
            let angle = angle + self.fov_angle / 2.0;

            // Since this angle is now in range <0,FOV_ANGLE>, by dividing it by
            // FOV_ANGLE, we transform it to range <0,1>.
            //
            // The value we get can be treated as a percentage, that is:
            //
            // - 0.2 = the food is seen by the "20%-th" eye cell
            //         (practically: it's a bit to the left)
            //
            // - 0.5 = the food is seen by the "50%-th" eye cell
            //         (practically: it's in front of our birdie)
            //
            // - 0.8 = the food is seen by the "80%-th" eye cell
            //         (practically: it's a bit to the right)
            let cell = angle / self.fov_angle;

            // With cell in range <0,1>, by multiplying it by the number of
            // cells we get range <0,CELLS> - this corresponds to the actual
            // cell index inside our `cells` array.
            //
            // Say, we've got 8 eye cells:
            // - 0.2 * 8 = 20% * 8 = 1.6 ~= 1 = second cell (indexing from 0!)
            // - 0.5 * 8 = 50% * 8 = 4.0 ~= 4 = fifth cell
            // - 0.8 * 8 = 80% * 8 = 6.4 ~= 6 = seventh cell
            let cell = cell * (self.cells as f32);

            // Our `cell` is of type `f32` - before we're able to use it to
            // index an array, we have to convert it to `usize`.
            //
            // We're also doing `.min()` to cover an extreme edge case: for
            // cell=1.0 (which corresponds to a food being maximally to the
            // right side of our birdie), we'd get `cell` of `cells.len()`,
            // which is one element *beyond* what the `cells` array contains
            // (its range is <0, cells.len()-1>).
            let cell = (cell as usize).min(cells.len() - 1);
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}
