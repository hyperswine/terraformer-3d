pub mod building;
pub mod decoration;

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
