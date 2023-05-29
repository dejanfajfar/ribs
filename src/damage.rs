use std::{
    fmt::{Display, Formatter, Result},
    ops::Add,
};
use serde::{Deserialize, Serialize};

pub mod hit;

use crate::hit::*;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    #[default]
    Piercing,
    Slashing,
}

#[derive(Debug, Default, PartialEq)]
pub enum Damage {
    Hit(Hit),
    #[default]
    Miss,
}

impl Add for Damage {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Damage::Hit(h) => match rhs {
                Damage::Miss => Damage::Hit(Hit {
                    piercing: h.piercing,
                    slashing: h.slashing,
                }),
                Damage::Hit(h2) => Damage::Hit(Hit {
                    piercing: h.piercing + h2.piercing,
                    slashing: h.slashing + h2.slashing,
                }),
            },
            Damage::Miss => match rhs {
                Damage::Miss => Damage::Miss,
                Damage::Hit(h) => Damage::Hit(Hit {
                    piercing: h.piercing,
                    slashing: h.slashing,
                }),
            },
        }
    }
}

impl Display for Damage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Damage::Hit(h) => write!(f, "HIT, p:{}, s:{}", h.piercing, h.slashing),
            Damage::Miss => write!(f, "MISS"),
        }
    }
}

impl Display for DamageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DamageType::Piercing => write!(f, "Piercing"),
            DamageType::Slashing => write!(f, "Slashing"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn apply_damage_already_at_maximum() {
        let mut damage = Hit {
            slashing: f32::MAX,
            piercing: f32::MAX,
        };

        damage.apply_damage(DamageType::Slashing, 1.0);

        assert_eq!(f32::MAX, damage.slashing);
    }

    #[test]
    fn apply_damage_damage_added() {
        let mut damage = Hit {
            slashing: 2.0,
            piercing: 2.0,
        };

        damage.apply_damage(DamageType::Slashing, 1.0);

        assert_eq!(3.0, damage.slashing);

        damage.apply_damage(DamageType::Piercing, 1.0);

        assert_eq!(3.0, damage.slashing);
    }

    #[test]
    fn damage_add_miss_miss() {
        let result = Damage::Miss + Damage::Miss;

        assert_eq!(result, Damage::Miss);
    }

    #[test]
    fn damage_add_miss_hit() {
        let result = Damage::Miss
            + Damage::Hit(Hit {
                piercing: 1.0,
                slashing: 1.0,
            });

        assert_eq!(
            result,
            Damage::Hit(Hit {
                piercing: 1.0,
                slashing: 1.0
            })
        );
    }

    #[test]
    fn damage_add_hit_miss() {
        let result = Damage::Hit(Hit {
            piercing: 1.0,
            slashing: 1.0,
        }) + Damage::Miss;

        assert_eq!(
            result,
            Damage::Hit(Hit {
                piercing: 1.0,
                slashing: 1.0
            })
        );
    }

    #[test]
    fn damage_add_hit_hit() {
        let result = Damage::Hit(Hit {
            piercing: 1.0,
            slashing: 1.0,
        }) + Damage::Hit(Hit {
            piercing: 1.0,
            slashing: 1.0,
        });

        assert_eq!(
            result,
            Damage::Hit(Hit {
                piercing: 2.0,
                slashing: 2.0
            })
        );
    }
}
