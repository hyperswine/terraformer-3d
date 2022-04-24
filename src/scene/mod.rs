// ----------
// SCENE
// ----------

pub struct Scene;

// a way to propagate transformation and animation and mechanism logic to subscenes/nodes
// i.e. solar system architecture
pub struct SceneGraph {
    root: Scene,
}

// ----------
// VERTICES, MESHES, TEXTURES, MODELS
// ----------

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

impl Mesh {
    pub fn new(_id: u64) -> Self {
        Self {
            id: _id,
            verts: vec![],
            textures: vec![],
        }
    }
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

// ----------
// SUB MODULES
// ----------

pub mod camera;
