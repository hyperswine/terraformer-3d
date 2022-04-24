use super::entity::Entity;

// Created when the user clicks on the game executable
// Should be one of the first things started
pub struct GameController {
    entities: Vec<Entity>,
    // time elapsed since starting the game
    time_since_game_start: f32,
}

// useful macro

// fn cmp( a1: &A, a2: &A ) -> bool { a1 as *const _ == a2 as *const _ }
macro_rules! cmp_ref {
    ($a:expr,$b:expr) => {
        $a as *const _ == $b as *const _
    };
}

impl GameController {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            time_since_game_start: 0.0,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.time_since_game_start += dt;

        // TODO: update all the entity states, collisions, new positions and mvp transforms, frontend
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

    // frontend update based on new backend info
    pub fn update_view(&self) {}
}

#[test]
fn test_tick() {
    let mut game_controller = GameController::new();
    // test that tick works properly
    game_controller.tick(100.0);
}
