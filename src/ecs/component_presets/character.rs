// ------------
// CHARACTER
// ------------

// A Character is a more generic version of a combatant
// It lives to be an "intelligent" creature whereas worldobjects are "non intelligent" and have more straightforward mechanics and interactions with other entities
struct Character;

// ------------
// TURN BASED COMBAT
// ------------

// Most things in a game like Eletian Journey should be a combatant
pub trait Combatant {
    fn attack(&self, combatant: &dyn Combatant);
    fn defend(&self, combatant: &dyn Combatant);
}

// ------------
// REAL TIME COMBAT
// ------------

// biggest difference vs turn based = tick based
// each tick, the gamecontroller tells each character to apply effects on them

// useful data structures for real time combat
// movespeed esp
// maybe a buffer time too

pub struct RealTimeCombatantInfo {
    hp: f32,
    movement_speed: f32,
    att_dmg: f32,
    buffer_secs_after_taking_dmg: f32,
}

impl RealTimeCombatantInfo {
    pub fn new(
        hp: f32,
        movement_speed: f32,
        att_dmg: f32,
        buffer_secs_after_taking_dmg: f32,
    ) -> Self {
        Self {
            hp,
            movement_speed,
            att_dmg,
            buffer_secs_after_taking_dmg,
        }
    }

    // all entities should only have a single attack. No multi attacks in backyard monsters, just basic attack
    // though there can also be abilities, which are more generic
    // for RTC though not really
    pub fn basic_attack(&self, enemy: &mut RealTimeCombatantInfo) {
        enemy.take_dmg(self.att_dmg);
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

// ------------
// Ailment
// ------------

// Usual ailments. Make the combatant choose how they will take the ailment
// each tick or turn
enum Ailment {
    Burn,
    Paralysis,
    Sleep,
    Freeze,
    Fear,
    Rage,
}

// ------------
// REAL TIME ABILITIES
// ------------

// one can define an ability
// as either a Supportive or Destructive ability

enum Target {
    SelfUnit,
    AllyUnit,
    EnemyUnit,
}

// Supportive: boost stats like RealTimeCombatInfo, whether hp, etc
// or recover from ailments in Ailments
struct RealTimeCombatAbility {
    target: Target,
    value: f32,
    number_of_targets: usize,
}

// ------------
// TEST
// ------------

#[test]
fn test_combat() {
    let mut combatinf = RealTimeCombatantInfo::new(100.0, 2.0, 10.0, 1.0);
    combatinf.take_dmg(500.0);
    assert_eq!(combatinf.get_hp(), 0.0);
}
