pub mod character;
pub mod worldobject;

// Most things should be displaceable, except maybe intrinsic map decors
// user decors, buildings and characters all displaceable
pub trait Displaceable {
    fn move_to(&self, coords: (f32, f32));
}

// an entity has a position
pub struct Entity {}
