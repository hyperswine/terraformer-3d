pub mod animate;
pub mod camera;
pub mod combat;
pub mod useable;
pub mod resource;

// BUFFERING AFTER DAMAGE (usually for real time)
pub struct Buffer {
    time: f32,
}

// NEWTONIAN MOVEMENT
pub struct Movement {
    acceleration: glam::Vec3,
    position: glam::Vec3
}
