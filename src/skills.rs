use rand::{thread_rng, Rng};

#[derive(Debug, Default, Clone, Copy)]
pub struct Skills {
    strength: u8,
    dexterity: u8,
}

impl Skills {
    pub fn new(strength: u8, dexterity: u8) -> Self {
        let mut new_skill = Skills::default();
        *new_skill.strength_mut() = strength;
        *new_skill.dexterity_mut() = dexterity;
        return new_skill;
    }

    pub fn skill_check(&self, min: Option<Skills>) -> bool {
        match min {
            None => return true,
            Some(min_skill) => {
                return self.strength() >= min_skill.strength()
                    && self.dexterity() >= min_skill.dexterity()
            }
        }
    }

    fn calculate_hit_probability(&self, min_requirements: Skills) -> f64 {
        match self.dexterity() {
            0..=2 => 0.2,
            10 => 1.0,
            _ => {
                let gap: i16 =
                    i16::from(self.dexterity()) - i16::from(min_requirements.dexterity());

                match gap {
                    0 => 0.5,
                    ..=-1 => 0.5 - f64::from(gap.abs()) * 0.05,
                    1..=2 => 0.5 + f64::from(gap) * 0.1,
                    3.. => 0.6 + f64::from(gap) * 0.05,
                }
            }
        }
    }

    pub fn is_hit(&self, min_requirements: Option<Skills>) -> bool {
        match min_requirements {
            None => return true,
            Some(skills) => {
                let mut rng = thread_rng();
                let p: f64 = self.calculate_hit_probability(skills);

                return rng.gen_bool(p);
            }
        }
    }

    pub fn normalize_skill(input: u8) -> u8 {
        match input {
            0..=10 => input,
            _ => 10,
        }
    }

    pub fn normalize_skill_mut(input: &mut u8) -> &mut u8 {
        match input {
            0..=10 => input,
            _ => {
                *input = 10;
                return input;
            }
        }
    }

    pub fn strength(&self) -> u8 {
        return Self::normalize_skill(self.strength);
    }

    pub fn strength_mut(&mut self) -> &mut u8 {
        return Self::normalize_skill_mut(&mut self.strength);
    }

    pub fn dexterity(&self) -> u8 {
        return Self::normalize_skill(self.dexterity);
    }

    pub fn dexterity_mut(&mut self) -> &mut u8 {
        return Self::normalize_skill_mut(&mut self.dexterity);
    }
}

pub struct SkillsFactory;

impl SkillsFactory{
    pub fn random(&self) -> Skills {
        let mut rng = thread_rng();
        return Skills::new(rng.gen_range(1..=10), rng.gen_range(1..=10));
    }

    pub fn ninja(&self) -> Skills {
        return Skills::new(6, 9);
    }
}

#[cfg(test)]
mod skills_test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn skill_check_none_given() {
        let t = Skills::default();

        assert_eq!(true, t.skill_check(None));
    }

    #[test]
    fn strength_mut_as_setter() {
        let mut s = Skills::default();
        *s.strength_mut() = 15;
        assert_eq!(10, s.strength());
    }

    #[test]
    fn calculate_hit_probability_tests() {
        // if required and player stat same then the probability is 0.5
        assert_eq!(0.5, call_calculate_hit_probability(3, 3));
        assert_eq!(0.5, call_calculate_hit_probability(4, 4));
        assert_eq!(0.5, call_calculate_hit_probability(5, 5));
        assert_eq!(0.5, call_calculate_hit_probability(6, 6));
        assert_eq!(0.5, call_calculate_hit_probability(7, 7));
        assert_eq!(0.5, call_calculate_hit_probability(8, 8));
        assert_eq!(0.5, call_calculate_hit_probability(9, 9));

        // required is larger than the player
        assert_relative_eq!(
            0.45,
            call_calculate_hit_probability(3, 4),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.40,
            call_calculate_hit_probability(3, 5),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.35,
            call_calculate_hit_probability(3, 6),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.30,
            call_calculate_hit_probability(3, 7),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.25,
            call_calculate_hit_probability(3, 8),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.20,
            call_calculate_hit_probability(3, 9),
            max_relative = 0.01
        );

        // required is smaller that the player

        assert_relative_eq!(
            0.60,
            call_calculate_hit_probability(4, 3),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.70,
            call_calculate_hit_probability(5, 3),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.75,
            call_calculate_hit_probability(6, 3),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.80,
            call_calculate_hit_probability(7, 3),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.85,
            call_calculate_hit_probability(8, 3),
            max_relative = 0.01
        );
        assert_relative_eq!(
            0.90,
            call_calculate_hit_probability(9, 3),
            max_relative = 0.01
        );
    }

    fn call_calculate_hit_probability(player_dex: u8, min_dex: u8) -> f64 {
        let player_skills: Skills = Skills::new(0, player_dex);
        let min_skills: Skills = Skills::new(0, min_dex);

        return player_skills.calculate_hit_probability(min_skills);
    }
}
