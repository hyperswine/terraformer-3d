// ----------
// Cameras
// ----------

// All you need is a current pos and the facing direction. Which is usually vec2d or vec3d
// Then you create a model matrix from that. 2d or 3d doesnt matter. Though I guess its nice to have presets

use glam::{Vec2, Vec4};

// A basic camera to pan over a texture/2D scene. No need for any complex transformations, just gimme the coords of each corner
pub struct Camera2D {
    curr_position: Vec2,
    // usually used for level start position
    level_start_position: Vec2,
    // prob not needed. Camera centered on curr_position with width x height
    corners: Vec4,
    size: Vec2,
}

impl Camera2D {
    pub fn new(
        curr_position: Vec2,
        level_start_position: Vec2,
        corners: Vec4,
        size: Vec2,
    ) -> Self {
        Self {
            curr_position,
            level_start_position,
            corners,
            size,
        }
    }

    pub fn move_to(&mut self, new_pos: Vec2) {
        self.curr_position = new_pos;
    }
}

// ----------
// 3D Camera
// ----------

pub trait Camera3D {
    fn new(position: (f32, f32, f32)) -> Self;
}

struct MainCamera {}

impl Camera3D for MainCamera {
    fn new(position: (f32, f32, f32)) -> Self {
        Self {}
    }
}
