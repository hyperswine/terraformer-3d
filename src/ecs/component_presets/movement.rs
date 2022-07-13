use bevy_ecs::prelude::Component;

/// NEWTONIAN MOVEMENT
#[derive(Debug, Component)]
pub struct Movement {
    acceleration: glam::Vec3,
    position: glam::Vec3,
}
