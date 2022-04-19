pub mod character;
pub mod worldobject;

// TODO: remove the stuff in character and use Engagabable/Combatant instead
pub trait Combatant {
    // all entities should only have a single attack. No multi attacks in backyard monsters, just basic attack
    fn attack(&self, combatant: &dyn Combatant);
    fn defend(&self, combatant: &dyn Combatant);
}

pub struct CombatantInfo {
    hp: f32,
    movement_speed: f32,
    att_dmg: f32,
}

impl CombatantInfo {
    pub fn new() -> Self {
        Self {
            hp: 100.0,
            movement_speed: 5.0,
            att_dmg: 50.0,
        }
    }
    pub fn take_dmg(&mut self, att_dmg: f32) {
        if self.hp - att_dmg < 0.0 {
            self.hp = 0.0;
        } else {
            self.hp -= att_dmg;
        }
    }
    pub fn get_hp(&self) -> f32 {
        self.hp
    }
}

#[test]
fn test_combat() {
    let mut combatinf = CombatantInfo::new();
    combatinf.take_dmg(500.0);
    assert_eq!(combatinf.get_hp(), 0.0);
}

// Most things should be displaceable, except maybe intrinsic map decors
// user decors, buildings and characters all displaceable
pub trait Displaceable {
    fn move_to(&self, coords: (f32, f32));
}

// an entity has a position
pub struct Entity {}
