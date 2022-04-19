pub mod game_controller;
pub mod entity;

use lazy_static::lazy_static;
use std::sync::Mutex;

// SOLUTION 1: GLOBAL MUTABLE VAR, require taking lock before writing
// To be completely safe, take lock before reading as well
lazy_static! {
    pub static ref DELTA_TIME: Mutex<f64> = Mutex::new(0.0);
}

// _dt: milliseconds
pub fn tick(_dt: f64) {
    // increment by dt
    match DELTA_TIME.lock().as_deref_mut() {
        Ok(d) => *d += _dt,
        Err(err) => panic!("couldn't dereference pointer for updating global dt"),
    }
    // update all the entity states for the game controller
}

#[test]
fn test_tick() {
    // test that tick works properly
    tick(100.0);
}

// test that characters can be made and attack turrets and stuff and state changes frontend
