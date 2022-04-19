pub mod decoration;

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

#[test]
fn test_worldobj_interaction() {
    let worldobj = WorldObject::new();
    worldobj.interact();
}
