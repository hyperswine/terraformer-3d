pub mod character;
pub mod worldobject;

use glam::Vec3;

use crate::scene::Model;

// Most things should be displaceable, except maybe intrinsic map decors
// user decors, buildings and characters all displaceable
pub trait Displaceable {
    fn move_to(&mut self, coords: Vec3) -> Vec3;
}

// assumed 3D world. For 2D chars, use entity2d instead of entity
// an entity has a position
// An entity should have a bound model, whether it be 3D or 2D
// if 2D, can use Entity2D or just a plane
pub struct Entity {
    id: u64,
    position: Vec3,
    bound_model: Model,
}

impl Displaceable for Entity {
    // might not be possible to move to the new pos due to collision
    // so after collision calcs, return the actual pos of the entity
    fn move_to(&mut self, coords: Vec3) -> Vec3 {
        // TODO: calc collisions
        self.position = coords;
        // return new position
        coords
    }
}

impl Entity {
    pub fn dnew(id: u64, position: Vec3, bound_model: Model) -> Self {
        Self {
            id,
            position,
            bound_model,
        }
    }

    pub fn id(&mut self) -> u64 {
        self.id
    }

    pub fn position(&mut self) -> &mut Vec3 {
        &mut self.position
    }
}

// HOW LIFETIME WORKS
// 'boot => references last at least as long as boot lasts
// struct Storage<'boot> {
//     f: &'boot str,
//     s: &'boot str,
// }
// MOSTLY when:
// - returning references from functions, esp if youre taking in one
// - creating structs with references
