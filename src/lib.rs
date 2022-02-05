pub mod math;
pub mod physics;
pub mod scene;

// stores common things across game engine states
// global game engine states like settings and preferences
struct Terraformer3D {}

impl Terraformer3D {
    fn new() -> Self {
        Terraformer3D {}
    }
}

#[test]
fn test_terraformer() {
    let terraformer = Terraformer3D {};
    let terraformer = Terraformer3D::new();
}
