// -------------------
// COLLISION DETECTION
// -------------------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BoundingBox3D {
    id: u64,
    min_axes: (f32, f32, f32),
    max_axes: (f32, f32, f32),
}

use itertools::Itertools;

// if a collision is detected between at least one pair of objects, calculate the total momentum inherited by each for the next timestep
pub fn check_collision(objects: &[BoundingBox3D]) {
    let mut pairs = objects.into_iter().combinations(2);

    for pair in pairs {
        let obj1 = pair[0];
        let obj2 = pair[1];
        // check collision between obj1 and obj2
        if obj1.min_axes.0 <= obj2.max_axes.0
            && obj1.max_axes.0 >= obj2.min_axes.0
            && obj1.min_axes.1 <= obj2.max_axes.1
            && obj1.max_axes.1 >= obj2.min_axes.1
            && obj1.min_axes.0 <= obj2.max_axes.2
            && obj1.max_axes.0 >= obj2.min_axes.2
        {
            println!("Collision Detected!");
        }
    }
}

// -------------------
// TRAJECTORY CALCS
// -------------------

/// Earth gravitational acceleration
const GRAVITATIONAL_ACCEL: f32 = 9.81;
const G_VEC: Vec3D = Vec3D::new(0.0, -GRAVITATIONAL_ACCEL, 0.0);

use nalgebra::Vector3;

type Vec3D = Vector3<f32>;

// given prev velocity and acceleration, calc next timestep
// without external forces
#[inline]
fn verlet_step(dt: f32, prev_pos: Vec3D, prev_v: Vec3D, prev_a: Vec3D, curr_a: Vec3D) -> Vec3D {
    // next velocity
    let next_v = prev_v + (prev_a + curr_a) / 2.0 * dt;
    // next position
    let next_pos = prev_pos + prev_v * dt + 0.5 * prev_a * (dt.powi(2));

    next_pos
}

// a typical projectile on earth with initial velocity, pos and no accel except G_VEC
fn bullet_step(dt: f32, prev_pos: Vec3D, prev_v: Vec3D, prev_a: Vec3D, mass: f32) -> Vec3D {
    // a = F/m = G/mass
    let curr_a = G_VEC / mass;

    // next velocity
    let next_v = prev_v + (prev_a + curr_a) / 2.0 * dt;
    // next position
    let next_pos = prev_pos + prev_v * dt + 0.5 * prev_a * (dt.powi(2));

    next_pos
}
