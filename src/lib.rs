#![feature(once_cell)]

pub mod assetloader;
pub mod game;
pub mod graphics;
pub mod math;
pub mod scene;
pub mod settings;
pub mod window;

// ----------
// Terraformer3D Global States
// ----------

// stores common things across game engine states
// global game engine states like settings and preferences
pub struct Terraformer3D {
    user_save_state: String,
    graphics_settings: String,
    interface_settings: String,
    // should have defaults for all
    keybindings: String,
}

impl Terraformer3D {
    pub fn new(
        user_save_state: &str,
        graphics_settings: &str,
        interface_settings: &str,
        keybindings: &str,
    ) -> Self {
        Self {
            user_save_state: user_save_state.to_string(),
            graphics_settings: graphics_settings.to_string(),
            interface_settings: interface_settings.to_string(),
            keybindings: keybindings.to_string(),
        }
    }
    
    // TODO: wrappers around game::loader
}

// ----------
// LOGGING
// ----------

pub fn get_current_logger_context() -> u32 {
    0
}

// log to the current logger context
#[macro_export]
macro_rules! log_screen {
    ($a:expr) => {
        let log_context = crate::get_current_logger_context();
    };
}

// ----------
// TEST
// ----------

#[test]
fn test_terraformer() {
    let terraformer = Terraformer3D::new("default", "default", "default", "default");
}
