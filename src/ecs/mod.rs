// -----------------------
// ENTITY COMPONENT SYSTEM
// -----------------------

// BUILTIN PRESETS
pub mod component_presets;
pub mod entity_presets;
pub mod system_presets;

// ? Whatabout library presets??
// Best to use a stack based vector

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

pub struct Component {
    entities: Vec<*mut Entity>,
}

pub trait ComponentUpdate {
    fn bind_to_entities(&mut self, entity: &[Entity]);
}

// Maybe idk. We want to also reuse component instances maybe?
// Just have a list of components in the Scene which are bound to a list of entities
// Maybe an ADT of {component: [id]} pairs

// -----------------------
// SYSTEM
// -----------------------

// Systems should use new component values updated by other systems, which are updated by Scripted Systems or Dynamic Systems

// First class function objects with a set of 'Forces' that change the states of the components bound to it
// A force is some constant or script that is applied on the components in some "Manner"
// A manner is a set of imperative descriptions that state how the forces should be applied to the constants based on the current constant parameters

// A single system should not have any contact with another system. It should be mostly atomic
// Systems are applied in some sort of order to achieve complex game mechanics
// Order is the key and you should order like:
//   input systems -> AI systems -> physics systems -> graphics systems

pub struct System {
    components: Vec<*mut Component>,
}

impl System {
    pub fn new(components: Vec<*mut Component>) -> Self {
        Self { components }
    }

    pub fn get_components(&mut self) -> &[*mut Component] {
        &self.components
    }
}

pub trait SystemUpdate {
    fn update(&mut self);
    fn bind_component(&mut self, component: *mut Component);
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