use crate::types::FaceTriangle;
use glam::Vec3;

/// Convex shapes are faster and you can just specify a set of points
pub struct CollisionConvex {
    faces: Vec<Vec3>,
}

/// Concave shapes require explicit face specification
pub struct CollisionConcave {
    faces: Vec<FaceTriangle>,
}
