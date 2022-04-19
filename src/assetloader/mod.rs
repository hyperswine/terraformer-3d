// ----------
// ASSETS
// ----------

use super::scene::{Mesh, Model, Texture, Vert};

// Mostly blend, gltf and obj files. Also mtl files and png files

// An Asset can be: .txt, .obj, .fbi, .wav, .mp3, .mp4, .gif, .png, .mtl, .blend

// Should be called from GameController::ObjectManager
pub fn load_obj(id: u64, path: &str) -> Mesh {
    use obj::*;
    let loaded_obj = Obj::load(path).unwrap();

    Mesh::new(id)
}

// terraformer has first class support for blender modules
fn load_blend(id: u64, path: &str) -> Mesh {
    Mesh::new(id)
}

// ----------
// TEST
// ----------

#[test]
fn test_load_blend() {
    let suzanne = load_blend(0, "assets/demo/suzanne.blend");
}
