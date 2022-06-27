// ------------
// CHARACTER
// ------------

use super::component_presets::combat::Combatant;

/// A Character is a more specific version of a combatant. You attach a controller to it, either Player or AI
struct Character;

impl Character {
    pub fn new() -> Self {
        // create
        let char = Self{};

        // attach components
        let combatant = Combatant::new(100.0, 0.0);
        

        char
    }
}

// A character may have combat capabilities
