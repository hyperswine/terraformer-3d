// ------------
// USEABLE OBJECT
// ------------

// Allow hotkeys to be bound to useable objects so you can pick it up, assign it to a quick slot, open/close, destroy, etc.

use bevy_ecs::prelude::Component;

// Reference implementation for a fully useable object according to the common ops of opening, closing, destroying. Moving is included in WorldObject and can be forwarded by world_info.move_to(new_pos)
#[derive(Debug, Component)]
pub struct UseableObject {
    opened: bool,
    destroyed: bool,
}
