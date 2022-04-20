pub mod character;
pub mod worldobject;

// Most things should be displaceable, except maybe intrinsic map decors
// user decors, buildings and characters all displaceable
pub trait Displaceable {
    fn move_to(&self, coords: (f32, f32));
}

// assumed 3D world
type Pos3D = [f32; 3];

// an entity has a position
pub struct Entity {
    id: u64,
    position: Pos3D,
}

impl Entity {
    pub fn new(_id: u64) -> Self {
        Self {
            id: _id,
            position: [0.0; 3],
        }
    }

    pub fn id(&mut self) -> u64 {
        self.id
    }

    pub fn position(&mut self) -> &mut Pos3D {
        &mut self.position
    }


}

// HOW LIFETIME WORKS
// 'boot => references last at least as long as boot lasts
// struct Storage<'boot> {
//     f: &'boot str,
//     s: &'boot str,
// }
