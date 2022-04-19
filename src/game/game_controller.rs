use super::DELTA_TIME;
use super::entity::Entity;

pub struct GameController {
    observed_entities: Vec<Entity>,
}

impl GameController {
    pub fn new() -> Self {
        Self {
            observed_entities: vec![Entity {}],
        }
    }

    pub fn get_delta_time() {
        DELTA_TIME.try_lock();
        // DELTA_TIME.get_mut().unwrap()
    }
}

// attach game controller as a listener to an entity's state
fn listen_to(entity: &Entity) {}

// should be updated on every tick. No it doesnt take that much cpu/gpu
// just update it, even if nothing changes. If nothing changes then GPU wont need to completely re-render/load new assets and render a new scene
fn update_view() {}
