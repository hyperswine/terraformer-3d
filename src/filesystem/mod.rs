// an asset is anything like a .txt file, .obj file, .fbi, .wav, .mp3, .mp4, .gif, .png
trait Asset {
    fn use_resource(&self);
    fn load(&self);
}

struct Obj;

pub struct Blend;

impl Asset for Obj {
    fn use_resource(&self) {}
    // load verts and stuff
    fn load(&self) {}
}

impl Asset for Blend {
    fn use_resource(&self) {}
    // load the vertices, scene graph, etc
    fn load(&self) {}
}

impl Blend {
    pub fn draw(&self) {

    }
}

fn load_obj(path: &str) -> Obj {
    Obj {}
}

// terraformer has first class support for blender modules
fn load_blend(path: &str) -> Blend {
    Blend {}
}

#[test]
fn test_load_blend() {
    let suzanne = load_blend("assets/demo/suzanne.blend");
    suzanne.load();
}
