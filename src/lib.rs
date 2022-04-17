pub mod filesystem;
pub mod graphics;
pub mod math;
pub mod scene;
pub mod game;

// ----------
// Terraformer3D Main
// ----------

// stores common things across game engine states
// global game engine states like settings and preferences
pub struct Terraformer3D {}

impl Terraformer3D {
    pub fn new() -> Self {
        Terraformer3D {}
    }
}

// ----------
// Game Loop
// ----------

// TODO: arcwm api
fn create_window() {}

enum MenuOption {
    Settings,
    Start,
}

fn menu_state() {
    // change settings
    let pressed: MenuOption = MenuOption::Start;

    match pressed {
        Settings => {}
        Start => {
            // transition into game
            start_game();

            // when in game press EXIT TO MENU, go back to menu_state
        }
    }
}

fn get_current_logger_context() -> u32 {
    0
}

// log to the current logger context
macro_rules! log_screen {
    ($a:expr) => {
        let log_context = get_current_logger_context();
    };
}

// load game settings from assets/settings
fn start_game() {
    let f = std::fs::read_to_string("assets/settings/user.cfg")
        .unwrap_or("Could not read user.cfg".to_string());

    // load game assets
    log_screen!("loading game assets...");

    use blend::Blend;
    let obj = Blend::from_path("assets/Character1.blend");

    for _obj in obj.get_by_code(*b"OB") {
        let loc = _obj.get_f32_vec("loc");
        let name = _obj.get("id").get_string("name");

        println!("\"{}\" at {:?}", name, loc);
    }
}

// ----------
// Cameras
// ----------

pub trait Camera3D {
    fn new(position: (f32, f32, f32)) -> Self;
}

struct MainCamera {}

impl Camera3D for MainCamera {
    fn new(position: (f32, f32, f32)) -> Self {
        Self {}
    }
}

// ----------
// TEST
// ----------

#[test]
fn test_terraformer() {
    let terraformer = Terraformer3D {};
    let terraformer = Terraformer3D::new();
}
