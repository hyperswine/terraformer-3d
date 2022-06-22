// -----------------------
// ENTITY COMPONENT SYSTEM
// -----------------------

// PRESETS
pub mod presets;

use glam::Vec3;

// -----------------------
// GENERIC ENTITY
// -----------------------

pub struct Entity {
    id: u64,
    systems: Vec<System>,
}

// -----------------------
// SYSTEMS
// -----------------------

pub enum System {
    /// Movable (translatable)
    Displaceable2D(DisplaceableDetails<(f32, f32)>),
}

// Maybe make it a method of the System::
pub struct DisplaceableDetails<Position> {
    current_position: Position,
}

// impl Displaceable for Entity {
//     /// Move to a new Coordinate
//     fn move_to(&mut self, coords: Vec3) -> Vec3 {
//         // Might not be possible to move to the new pos due to collision. So after collision calcs, return the actual pos of the entity

//         // TODO: calc collisions
//         self.position = coords;

//         coords
//     }
// }

impl Entity {
    pub fn new(id: u64, systems: Vec<System>) -> Self {
        Self { id, systems }
    }

    pub fn id(&mut self) -> u64 {
        self.id
    }
}

// -----------------------
// TEST
// -----------------------

#[test]
fn test_entity_creation() {
    let entity = Entity::new(0, vec![]);
}

#[test]
fn test_entity_methods() {
    let mut entity = Entity::new(0, vec![]);

    assert_eq!(entity.id, 0);
}
