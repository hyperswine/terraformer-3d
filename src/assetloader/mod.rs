// ----------
// ASSETS
// ----------

use gltf::Gltf;

use crate::graphics::Mesh;

// Mostly blend, gltf and obj files. Also mtl files and png files
// And result in either a Scenegraph, Model or Texture or Animation2D
// Animation3D should be taken from the models within the scene graph

// An Asset can be: .txt, .obj, .fbi, .wav, .mp3, .mp4, .gif, .png, .mtl, .blend

// Should be called from GameController::ObjectManager
pub fn load_obj(id: u64, path: &str) -> Mesh {
    use obj::*;
    let loaded_obj = Obj::load(path).unwrap();

    let objs = loaded_obj.data.objects;
    for __obj in objs {
        println!("object name = {}, ", __obj.name);
    }

    Mesh::new(id)
}

// terraformer has first class support for blender modules
pub fn load_blend(id: u64, path: &str) -> Mesh {
    use blend::Blend;

    let obj = Blend::from_path(path);

    for _obj in obj.get_by_code(*b"OB") {
        let loc = _obj.get_f32_vec("loc");
        let name = _obj.get("id").get_string("name");

        println!("\"{}\" at {:?}", name, loc);
    }

    Mesh::new(id)
}

pub fn load_gltf(id: u64, path: &str) -> Mesh {
    let gltf = Gltf::open(path).unwrap();

    // TODO: create a scene graph and return that
    for scene in gltf.scenes() {
        for node in scene.nodes() {
            println!(
                "Node #{} has {} children",
                node.index(),
                node.children().count()
            );
        }
    }

    Mesh::new(id)
}

// ----------
// TEST
// ----------

#[test]
fn test_load_blend() {
    let suzanne = load_blend(0, "assets/models/suzanne.blend");
}

#[test]
fn test_load_obj() {
    let suzanne = load_obj(0, "assets/models/suzanne.obj");
}

#[test]
fn test_load_gltf() {
    let char1 = load_gltf(0, "assets/models/Character1.gltf");
}
