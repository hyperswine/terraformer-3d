// GROUPS OF CHARACTERS AND ENTITIES
// for a coilgun team, need 4 coilgun character entities placed generally next to each other
// in a certain radius. This is radius TeamR.
// for a base, can place worldobjects anywhere. Worldobjects can be interacted with in groups
// by grouping them up. They will then be useful for base views
// ! we dont care too much about this rn though

use super::Combatant;
use super::CombatantInfo;
use super::Displaceable;

pub struct TeamInfo;

// Actions a team can take, e.g. attack an entity (team or worldobj), defend a position (current pos), move to a position (mouse click x,y)
// When attacking/defending, doing so to an attackable/defendable

pub struct CoilgunTeam {
    combat_info: CombatantInfo,
    meta_info: TeamInfo,
}

impl Combatant for CoilgunTeam {
    fn attack(&self, combatant: &dyn Combatant) {
        combatant.defend(self);
    }

    // this is where the real logic is
    fn defend(&self, combatant: &dyn Combatant) {
        // take dmg
        // combat_info.take_dmg(combatant.get_att());
    }
}

pub struct RailgunTeam {
    info: TeamInfo,
}
pub struct MechaTeam {
    info: TeamInfo,
}
pub struct TankerTeam {
    info: TeamInfo,
}
pub struct SupplyConvoyTeam {
    info: TeamInfo,
}
