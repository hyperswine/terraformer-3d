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

/// Resolve systems at runtime per tick, collisions, additions and removals
pub struct EntityManager {
    entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new(entities: Vec<Entity>) -> Self {
        Self { entities }
    }
    // attach game controller as a listener to an entity's state
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    // remove an entity, e.g. it died or went out of bounds for game logic to care about
    pub fn remove_entity(&mut self, entity: &mut Entity) {
        // retain anything that matches the preposition
        self.entities.retain_mut(|e| e.id() != entity.id());
    }

    // Each tick, each component could have an Updatable System, like an animation
    // So you need to go through each entity and find updatable systems, to then .update(). Or maybe just have update() for System in general and let the type overload it

    pub fn update(&mut self) {
        // UPDATE entity states, collisions, new positions and mvp transforms, frontend and stuff should be updated right afterwards in the FrontendManager/SceneManager
    }
}

// -----------------------
// SYSTEMS
// -----------------------

// Actually these should be components
// The systems are the methods that act on the components of an entity. But should maybe be separate from the components?

pub enum System {
    /// Movable (translatable)
    Displaceable2D(DisplaceableDetails<(f32, f32)>),
    Openable(OpenableDetails),
    Destroyable(DestroyableDetails),
}

/// A useful detail for many things like doors, cabinets, etc
pub struct OpenableDetails {
    opened: bool,
}

/// Maybe there is a state of being 'in tact' or 'destroyed' which may make it unusable even if it hasnt been used yet
pub struct DestroyableDetails {
    intact: bool,
}

// Maybe make it a method of the System::
pub struct DisplaceableDetails<Position: Clone> {
    current_position: Position,
}

impl<Position: Clone> DisplaceableDetails<Position> {
    pub fn new(current_position: Position) -> Self {
        Self { current_position }
    }
    fn move_to(&mut self, new_position: Position) -> Position {
        // Might not be possible to move to the new pos due to collision. So after collision calcs, return the actual pos of the entity

        // Calc collisions if there is a collision system?
        // no do it in the Entity Manager
        self.current_position = new_position.clone();

        new_position
    }
}

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
