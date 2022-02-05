pub struct Scene;

pub struct Vert {
    coordinates: [i32; 3],
    normals: [i32; 3],
    tc: [i32; 2],
    tangents: [i32; 3],
    bitangents: [i32; 3],
}

pub struct Mesh {
    id: u64,
    verts: Vec<Vert>,
    textures: Vec<Texture>,
}

pub struct Model {
    // if loading twice, destroy an existing version
    id: u64,
    meshes: Vec<Mesh>,
}

// can either be applied to a model/mesh aka Object3D or painted directly
pub struct Texture {
    // given by vulkan
    t_id: u64,

}

use crate::graphics::Drawable;
use crate::graphics::shaders::Shader;

impl Drawable for Model {
    fn draw(&self, shader: Shader) {
        shader.apply();
    }

    fn initialize(&self, scene: Scene) {
        
    }
}
