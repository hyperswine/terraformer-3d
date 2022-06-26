// -----------------------
// ENTITY COMPONENT SYSTEM
// -----------------------

// PRESETS
pub mod presets;

// -----------------------
// GENERIC ENTITY
// -----------------------

pub struct Entity {
    id: u64,
    children: Vec<Entity>,
}

impl Entity {
    pub fn new(id: u64, children: Vec<Entity>) -> Self {
        Self { id, children }
    }
}

// -----------------------
// COMPONENT
// -----------------------

// Custom structs with information that implements the Component Trait
// For systems bind to and modify

pub trait Component {
    type Data;
    fn update(&mut self, new_data: Self::Data);
}

// -----------------------
// SYSTEM
// -----------------------

// First class function objects with a set of 'Forces' that change the states of the components bound to it
// A force is some constant or script that is applied on the components in some "Manner"
// A manner is a set of imperative descriptions that state how the forces should be applied to the constants based on the current constant parameters

// A single system should not have any contact with another system. It should be mostly atomic
// Systems are applied in some sort of order to achieve complex game mechanics
// Order is the key and you should order like:
//   input systems -> AI systems -> physics systems -> graphics systems

pub trait System {
    // fn update(components: &[Component]);
}

// "Systems" dont really exist. All you do is register a function with SceneSystems or GlobalSystems
// Maybe just scene systems

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
