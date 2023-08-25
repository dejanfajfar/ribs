/// Represents a combatant in the battle.
/// No alliances are represented and not positional data on the map is kept by this structure
#[derive(Clone, Debug, PartialOrd, Eq, Ord)]
pub struct Combatant{
    pub name: String,
    pub hp: u16,
    pub dmg: u16,
    pub id: Option<String>,
    pub avatar: Option<String>,
}

impl Combatant {
    /// Determines if the combatant can still partake in the battle
    pub fn is_alive(&self) -> bool {
        self.hp != u16::MIN
    }

    /// Applies the given damage to the current combatant
    pub fn apply_damage(&mut self, damage: u16) {
        let sub_result = self.hp.overflowing_sub(damage);

        if sub_result.1 {
            self.hp = u16::MIN;
        }
        else {
            self.hp = sub_result.0;
        }
    }   
}

impl PartialEq for Combatant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_alive_hp_0(){
        let test_object = Combatant{
            name: "test".to_owned(),
            dmg: 2,
            hp: 0,
            avatar: None,
            id: None,
        };

        assert!(!test_object.is_alive());
    }

    #[test]
    fn is_alive_hp_min(){
        let test_object = Combatant{
            name: "test".to_owned(),
            dmg: 2,
            hp: u16::MIN,
            avatar: None,
            id: None,
        };

        assert!(!test_object.is_alive());
    }

    #[test]
    fn apply_damage_hp_left(){
        let mut test_object = Combatant{
            name: "test".to_owned(),
            dmg: 2,
            hp: 15,
            avatar: None,
            id: None,
        };

        test_object.apply_damage(5);

        assert_eq!(10, test_object.hp);
    }

    #[test]
    fn apply_damage_hp_gone(){
        let mut test_object = Combatant{
            name: "test".to_owned(),
            dmg: 2,
            hp: 15,
            avatar: None,
            id: None,
        };

        test_object.apply_damage(15);

        assert_eq!(0, test_object.hp);
        assert_eq!(u16::MIN, test_object.hp);

        assert!(!test_object.is_alive());
    }

    #[test]
    fn apply_damage_dmg_higher_than_hp(){
        let mut test_object = Combatant{
            name: "test".to_owned(),
            dmg: 2,
            hp: 15,
            avatar: None,
            id: None,
        };

        test_object.apply_damage(u16::MAX);

        assert_eq!(0, test_object.hp);
        assert_eq!(u16::MIN, test_object.hp);

        assert!(!test_object.is_alive());
    }

    #[test]
    fn partial_eq(){
        let c1 = Combatant{
            name: "test".to_owned(),
            dmg: 2,
            hp: 15,
            avatar: None,
            id: None,
        };

        assert_eq!(c1, c1);
    }

    #[test]
    fn partial_eq_only_name_checked(){
        let c1 = Combatant{
            name: "c1".to_owned(),
            dmg: 5,
            hp: 10,
            avatar: None,
            id: None,
        };

        let c2 = Combatant{
            name: "c1".to_owned(),
            dmg: 2,
            hp: 20,
            avatar: None,
            id: None,
        };

        assert_eq!(c1, c2);
    }


}