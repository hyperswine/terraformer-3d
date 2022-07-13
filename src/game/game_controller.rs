// ------------------
// REAL TIME GAME CONTROLLER
// ------------------

// use crate::ecs::{Entity};

/// Most games should use a RT game controller as the outer controller and overworld events
pub struct RealTimeController {
    time_since_game_start: f32,
}

impl RealTimeController {
    pub fn new() -> Self {
        Self {
            time_since_game_start: 0.0,
        }
    }

    // REAL TIME GAME LOOP
    pub fn tick(&mut self, dt: f32) {
        self.time_since_game_start += dt;
    }

    // frontend update based on new backend info
    pub fn update_view(&self) {}
}

// ------------------
// TURN BASED CONTROLLER
// ------------------

// Could be good to embed in a turn based combat situation. Where you're mostly just waiting. And the base controller is still real time

// ? embeddable within a real time game controller, e.g. an overworld real time vs in battle turn based

/// For scenes where the main logic is mostly turn based
struct TurnBasedController;

// ------------------
// TEST
// ------------------

#[test]
fn test_tick() {
    let mut game_controller = RealTimeController::new();
    // test that tick works properly
    game_controller.tick(100.0);
}
