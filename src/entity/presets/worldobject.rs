// ------------
// USEABLE OBJECT
// ------------

// Allow hotkeys to be bound to useable objects so you can pick it up, assign it to a quick slot, open/close, destroy, etc.

// Reference implementation for a fully useable object according to the common ops of opening, closing, destroying. Moving is included in WorldObject and can be forwarded by world_info.move_to(new_pos)
pub struct UseableObject {
    opened: bool,
    destroyed: bool,
}

