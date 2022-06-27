use crate::ecs::component_presets::movement::Movement;

// ----------------
// MOVEMENT SYSTEM
// ----------------

// Depends on physics engine

/// Given AI and Player input accelerations and current positions + velocities, update new positions
/// If the entity also has a collision component, calculates that too
/// The accelerations should be changed directly (by InputSystem), but the positions and velocities should not
pub struct MovementSystem {
    movement_components: Vec<*mut Movement>,
}

impl MovementSystem {
    pub fn update(&mut self) {
        // maybe calculate new positions?
        let movement_components = &self.movement_components;

        // use physics engine to calc new position
    }
}

// ----------------
// COLLISION SYSTEM
// ----------------

// Maybe a uniform interface that returns something that can be directly compared
// Like uh.. maybe just return a set of faces transformed into world coords?

pub trait Collidable {
    type Rhs;
    fn check_collision(&self, other: &Self::Rhs);
}

pub fn check_collision<C: Collidable>(collidable1: &C, collidable2: &C) -> bool {
    true
}
