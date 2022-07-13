// ----------
// Load Game State
// ----------

use super::game_controller::RealTimeController;
use crate::log_screen;
use std::collections::BTreeMap;

pub fn load_yml(path: &str) -> serde_yaml::Value {
    let f = std::fs::File::open(path).unwrap();

    // should tell us which models and assets in assets/ should be loaded here
    serde_yaml::from_reader(f).unwrap()
}

pub fn create_window() {
    // ? Maybe use wgpu windowing -> arcwm implementation
    // let res = crate::window::load_window();

    // load game controller
    let game_controller = RealTimeController::new();

    // pass game_controller to function calls like menu_state start_game and game_loop
    menu_state(game_controller);
}

// ----------
// MAIN MENU
// ----------

enum MenuOption {
    Settings,
    Start,
}

// A game should boot pretty much immediately
// Only [start_game] should load most of the models into RAM, though which ones
// can be dictated by the current map and level and in menu_load.yml
pub fn menu_state(game_controller: RealTimeController) {
    // make sure to do these lazily and only once!
    // TODO: encapsulate into a single function and call it lazily

    use std::lazy::Lazy;

    let mut menu_asset_loads: Lazy<_> = Lazy::new(|| load_yml("assets/menu_load.yml"));

    // should tell us which models and assets in assets/ should be loaded here
    let mut menu_asset_loads = load_yml("assets/menu_load.yml");

    // TODO: load assets based on whats specified in menu_asset_loads

    // change settings
    let pressed: MenuOption = MenuOption::Start;

    match pressed {
        MenuOption::Settings => {}
        MenuOption::Start => {
            // transition into game
            start_game(game_controller);

            // when in game press EXIT TO MENU, go back to menu_state
        }
    }
}

// NOTE: depending on the type of game config
// call the right function to load the specific yml states and configs. And models

// ----------
// Game Loop
// ----------

// Load game settings from assets/settings. Usually loads as much as possible, though for bigger games theres no need to load stuff outside of your current level/nearby levels. These can be specified in assets/settings/load_state.yml
// which should be updated according (automatically kinda) to the current level and maps and entities. Games built with terraformer should mostly be pick up and play kinda thing like SMT and BY Monsters so I dont really like loading on the fly
pub fn start_game(game_controller: RealTimeController) {
    // TODO: encapsulate loading into a single function and call it lazily

    let f = std::fs::File::open("assets/settings/global_settings.yml").unwrap();

    let mut global_settings: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();

    let f = std::fs::File::open("assets/settings/user.yml").unwrap();

    let mut user_game_progression: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();

    // load game assets (models)
    log_screen!("loading game assets...");

    let f = std::fs::File::open("assets/load_state.yml").unwrap();

    let mut game_assets: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();

    // TODO: load assets based on whats specified in load_state

    let f = std::fs::File::open("assets/load_state.yml").unwrap();

    let mut load_state: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();

    // read assets/objects

    // for asset in asset: game_controller.load_asset(asset)

    // read assets/scenes combine into scenes

    // read assets/levels to load levels
}
