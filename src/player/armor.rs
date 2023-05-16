use crate::damage::hit::*;
use crate::damage::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Armor {
    piercing: f32,
    slashing: f32,
}

impl Armor {
    pub fn new(piercing: f32, slashing: f32) -> Self {
        return Armor {
            piercing: piercing,
            slashing: slashing,
        };
    }

    pub fn calculate_reduction(&self, incoming_damage: &Damage) -> Damage {
        match incoming_damage {
            Damage::Miss => return Damage::Miss,
            Damage::Hit(h) => {
                let piercing_damage: f32 = h.piercing - self.piercing;
                let slashing_damage: f32 = h.slashing - self.slashing;

                if piercing_damage <= 0.0 && slashing_damage <= 0.0 {
                    return Damage::Miss;
                }

                return Damage::Hit(Hit {
                    slashing: slashing_damage.max(0.0),
                    piercing: piercing_damage.max(0.0),
                });
            }
        }
    }

    pub fn piercing_reduction(&self) -> f32 {
        self.piercing
    }

    pub fn slashing_reduction(&self) -> f32 {
        self.slashing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incoming_damage_reduced() {
        let incoming_damage: Damage = Damage::Hit(Hit {
            piercing: 10.0,
            slashing: 10.0,
        });

        let armor: Armor = Armor::new(5.0, 5.0);

        let reduced_damage: Damage = armor.calculate_reduction(&incoming_damage);

        match reduced_damage {
            Damage::Miss => assert!(false),
            Damage::Hit(h) => {
                assert_eq!(5.0, h.piercing);
                assert_eq!(5.0, h.slashing);
            }
        }
    }

    #[test]
    fn reduce_damage_past_zero_piercing() {
        let incoming_damage: Damage = Damage::Hit(Hit {
            piercing: 5.0,
            slashing: 5.0,
        });

        let armor: Armor = Armor::new(10.0, 0.0);

        let reduced_damage: Damage = armor.calculate_reduction(&incoming_damage);

        match reduced_damage {
            Damage::Miss => assert!(false),
            Damage::Hit(h) => {
                assert_eq!(0.0, h.piercing);
                assert_eq!(5.0, h.slashing);
            }
        }
    }

    #[test]
    fn reduce_damage_past_zero_slashing() {
        let incoming_damage: Damage = Damage::Hit(Hit {
            piercing: 5.0,
            slashing: 5.0,
        });

        let armor: Armor = Armor::new(0.0, 10.0);

        let reduced_damage: Damage = armor.calculate_reduction(&incoming_damage);

        match reduced_damage {
            Damage::Miss => assert!(false),
            Damage::Hit(h) => {
                assert_eq!(5.0, h.piercing);
                assert_eq!(0.0, h.slashing);
            }
        }
    }

    #[test]
    fn reduce_all_damage_then_miss() {
        let incoming_damage: Damage = Damage::Hit(Hit {
            piercing: 5.0,
            slashing: 5.0,
        });

        let armor: Armor = Armor::new(5.0, 5.0);

        let reduced_damage: Damage = armor.calculate_reduction(&incoming_damage);

        match reduced_damage {
            Damage::Miss => assert!(true),
            Damage::Hit(h) => {
                assert!(false)
            }
        }
    }
}
