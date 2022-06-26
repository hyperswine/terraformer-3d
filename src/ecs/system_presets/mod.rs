// MOVEMENT SYSTEM. Depends on physics engine

use super::{System, component_presets::Movement};

/// Given AI and Player input accelerations and current positions + velocities, update new positions
/// If the entity also has a collision component, calculates that too
/// The accelerations should be changed directly (by InputSystem), but the positions and velocities should not
pub struct MovementSystem {
    movement_components: Vec<*mut Movement>
}

impl MovementSystem {
    pub fn update(&mut self) {
        // maybe calculate new positions?
        let movement_components = &self.movement_components;
        
        // use physics engine to calc new position
    }
}

// INPUT SYSTEM

pub struct InputSystem {}

