// ------------
// WORLD OBJECT
// ------------

// a 3D interactable worldobject
pub struct WorldObject {
    position: Vec3,
}

impl WorldObject {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }

    pub fn interact(&self) -> bool {
        true
    }
}

impl Displaceable for WorldObject {
    fn move_to(&mut self, coords: Vec3) -> Vec3 {
        self.position = coords;
        coords
    }
}

pub mod decoration;

// ------------
// Common Interactions
// ------------

// should change the state of your custom object
// if implementing open(), should also bind that global user action / hotkey to F or something. And give your struct an open var to tell the game controller to start opening it on the frontend model binded / scene next tick

pub trait Openable {
    fn open(&mut self);
}

pub trait Closeable {
    fn close(&mut self);
}

pub trait Destroyable {
    fn destroy(&mut self);
}

pub trait Recreateable {
    fn recreate(&mut self);
}

// ------------
// USEABLE OBJECT
// ------------

// allow hotkeys to be bound to useable objects so you can
// pick it up, open/close, destroy, etc.

// Reference implementation for a fully useable object according to the common ops of opening, closing, destroying. Moving is included in WorldObject and can be forwarded by world_info.move_to(new_pos)
pub struct FullUseableObject {
    world_info: WorldObject,
    opened: bool,
    // if destroyed, one could say disappear it or play a destruction animation and disappear it
    // possible to make it reappear at the same or different position by recreate() but you have to implement it yourself
    destroyed: bool,
}

impl Openable for FullUseableObject {
    fn open(&mut self) {
        self.opened = true;
    }
}

impl Closeable for FullUseableObject {
    fn close(&mut self) {
        self.opened = false;
    }
}

impl Destroyable for FullUseableObject {
    fn destroy(&mut self) {
        self.destroyed = true;
    }
}

impl Recreateable for FullUseableObject {
    fn recreate(&mut self) {
        self.destroyed = false;
    }
}

impl FullUseableObject {
    pub fn new(world_info: WorldObject, opened: bool, destroyed: bool) -> Self {
        Self {
            world_info,
            opened,
            destroyed,
        }
    }
}

// ------------
// TEST
// ------------

use super::Displaceable;
use glam::Vec3;

#[test]
fn test_worldobj_interaction() {
    let worldobj = WorldObject::new(Vec3::new(0.0, 0.0, 0.0));
    worldobj.interact();
}
