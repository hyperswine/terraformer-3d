// ---------------
// USES
// ---------------

use glam::{Vec2, Vec3};

use crate::types::VectorOps;

// ---------------
// Generic Camera
// ---------------

/// A basic camera to pan over a texture or a 3D scene. Needs at least 2 axis (usually direction/bitangent and tangent)
pub struct Camera<T: VectorOps + Copy> {
    curr_position: T,
    direction: T,
    right: T,
    // cached
    up: T,
    level_start_position: T,
    size: T,
}

impl<T: VectorOps + Copy> Camera<T> {
    pub fn new(
        curr_position: T,
        direction: T,
        right: T,
        up: T,
        level_start_position: T,
        size: T,
    ) -> Self {
        Self {
            curr_position,
            direction,
            right,
            up,
            level_start_position,
            size,
        }
    }

    pub fn get_dimensions(&self) -> &T {
        &self.size
    }

    pub fn get_starting_position(&self) -> &T {
        &self.level_start_position
    }

    /// Every time you move, recalc the up vector using your new dir and right vec
    pub fn update(&mut self, direction: T, right: T) {
        self.direction = direction;
        self.right = right;

        // cross multiply
        self.up = direction.cross(&right);
    }
}

pub type Camera2D = Camera<Vec2>;
pub type Camera3D = Camera<Vec3>;

// ---------------
// TESTS
// ---------------

#[test]
fn test_camera() {
    let v = Vec2::new(0.0, 0.0);
    let camera2d = Camera2D::new(v, v, v, v, v, v);

    assert_eq!(camera2d.get_dimensions().clone(), v);
    assert_eq!(camera2d.get_starting_position().clone(), v);
}
