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

impl Openable for UseableObject {
    fn open(&mut self) {
        self.opened = true;
    }
}

impl Closeable for UseableObject {
    fn close(&mut self) {
        self.opened = false;
    }
}

impl Destroyable for UseableObject {
    fn destroy(&mut self) {
        self.destroyed = true;
    }
}

impl Recreateable for UseableObject {
    fn recreate(&mut self) {
        self.destroyed = false;
    }
}

// ------------
// USEABLE OBJECT
// ------------

// Allow hotkeys to be bound to useable objects so you can pick it up, assign it to a quick slot, open/close, destroy, etc.

// Reference implementation for a fully useable object according to the common ops of opening, closing, destroying. Moving is included in WorldObject and can be forwarded by world_info.move_to(new_pos)
pub struct UseableObject {
    opened: bool,
    // if destroyed, one could say disappear it or play a destruction animation and disappear it possible to make it reappear at the same or different position by recreate() but you have to implement it yourself. Just have to play the systems
    destroyed: bool,
}

