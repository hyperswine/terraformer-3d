// --------------------
// SUB MODULES
// --------------------

pub mod animator;
pub mod camera;

// --------------------
// SCENE
// --------------------

// Basically a tree of SceneNode that represents Entities like Players, NPCs, Enemies, World Objects, etc

/// A scene should contain a single model. Aka be 'bound' to a model. And any sub models are sub scenes
pub struct Scene {
    model: Model,
    children: Vec<Scene>,
}

impl Scene {
    pub fn new(model: Model, children: Vec<Scene>) -> Self {
        Self { model, children }
    }
}

/// A way to propagate transformation and animation and mechanism logic to subscenes/nodes. I.e. solar system architecture
pub struct SceneGraph {
    root: Scene,
}

impl SceneGraph {
    pub fn new(root: Scene) -> Self {
        Self { root }
    }
}

// --------------------
// VERTICES, MESHES, TEXTURES, MODELS
// --------------------

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
    // if loading twice, should destroy an existing version. But IDK, it should be up to the dev to not be stupid
    id: u64,
    meshes: Vec<Mesh>,
}

impl Model {
    pub fn new(id: u64, meshes: Vec<Mesh>) -> Self {
        Self { id, meshes }
    }

    pub fn new_dummy() -> Self {
        Self {
            id: 0,
            meshes: vec![],
        }
    }
}

/// Can either be applied to a model/mesh aka Object3D or painted directly
pub struct Texture {
    // given by vulkan
    t_id: u64,
}

// --------------------
// TEST
// --------------------

#[test]
fn test_scene_creation() {
    let scene = Scene::new(Model::new_dummy(), vec![]);
}
