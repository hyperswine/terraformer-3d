use glam::Vec3;

pub type FaceTriangle = Vec<Vec3>;

pub struct Face {
    triangle: FaceTriangle,
    normal: Vec3,
}
