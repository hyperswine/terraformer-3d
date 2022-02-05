use crate::scene::Scene;

// vulkano wrappers
pub mod shaders;
pub mod vulkan;

fn create_window() {}

#[test]
fn test_window() {}

pub trait Drawable {
    fn draw(&self, shader: shaders::Shader);
    // place a drawable object (mesh, if a 2d image then simply a square with a texture or model)
    fn initialize(&self, scene: Scene);
}
