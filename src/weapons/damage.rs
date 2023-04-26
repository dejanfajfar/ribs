use std::{
    default,
    fmt::{Display, Formatter, Result},
    ops::Add,
};

#[derive(Default, Debug, Copy, Clone)]
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

#[derive(Debug, PartialEq)]
pub struct Hit {
    pub piercing: f32,
    pub slashing: f32,
}

impl Hit {
    pub fn apply_damage(&mut self, damage_type: DamageType, damage_value: f32) {
        match damage_type {
            DamageType::Piercing => self.piercing += damage_value,
            DamageType::Slashing => self.slashing += damage_value,
        }
    }
}

impl Default for Hit {
    fn default() -> Self {
        Self {
            piercing: 0.0,
            slashing: 0.0,
        }
    }
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
