// ------------
// COMBATANT
// ------------

pub struct Combatant {
    hp: f32,
    att_dmg: f32,
}

impl Combatant {
    pub fn new(
        hp: f32,
        att_dmg: f32,
    ) -> Self {
        Self {
            hp,
            att_dmg,
        }
    }

    pub fn basic_attack(&self, enemy: &mut Combatant) {
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

// Usual ailments. Attach to an ailment system that applies statuses each tick or turn
enum Ailment {
    Burn,
    Paralysis,
    Sleep,
    Freeze,
    Fear,
    Rage,
}

// ------------
// ABILITIES
// ------------

/// You can define an entity as being a Self, Ally or Enemy in Battle
/// Then apply the combat system to them
enum TargetType {
    SelfUnit,
    AllyUnit,
    EnemyUnit,
}

/// An ability is something with a set of values and a target
/// The ability type and impl defines how those values affect the target
struct Ability<Value, TargetHandle> {
    targets: Vec<TargetHandle>,
    value: Value,
}

// ------------
// TEST
// ------------

#[test]
fn test_combat() {
    let mut combatant = Combatant::new(100.0, 10.0);
    combatant.take_dmg(500.0);
    assert_eq!(combatant.get_hp(), 0.0);
}
