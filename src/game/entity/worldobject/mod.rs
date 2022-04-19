// ------------
// WORLD OBJECT
// ------------

// an interactable worldobject
pub struct WorldObject;

impl WorldObject {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interact(&self) -> bool {
        true
    }
}

pub mod decoration;

// ------------
// TEST
// ------------

#[test]
fn test_worldobj_interaction() {
    let worldobj = WorldObject::new();
    worldobj.interact();
}
