use crate::damage::*;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
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

    pub fn new(damage_type: DamageType, damage_value: f32) -> Self {
        let mut ret_val = Hit::default();
        ret_val.apply_damage(damage_type, damage_value);
        return ret_val;
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
}
