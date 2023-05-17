pub mod skillpoint;

use std::{cmp::Ordering, hash, collections::HashMap};

use rand::{thread_rng, Rng};
use skillpoint::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Skills {
    strength: SkillPoint,
    dexterity: SkillPoint,
}

impl Skills {
    pub fn new(strength: u8, dexterity: u8) -> Self {
        return Skills {
            dexterity: SkillPoint::from(dexterity),
            strength: SkillPoint::from(strength),
        };
    }

    fn calculate_hit_probability(&self, min_requirements: Skills) -> f64 {
        // does not meet minimal requirements
        if self.dexterity() < min_requirements.dexterity() {
            let gap = min_requirements.dexterity - self.dexterity();

            return 0.5 - ((f64::from(gap.raw_value()) / 10.0) * 0.5);
        }

        // meets minimal requirements

        let gap: SkillPoint = self.dexterity() - min_requirements.dexterity();

        return (f64::from(gap.raw_value()) / 10.0) + 0.5;
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

    pub fn strength(&self) -> SkillPoint {
        return self.strength;
    }

    pub fn dexterity(&self) -> SkillPoint {
        return self.dexterity;
    }
}

impl PartialEq for Skills {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength && self.dexterity == other.dexterity
    }
}

impl PartialOrd for Skills {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.strength.partial_cmp(&other.strength) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        self.dexterity.partial_cmp(&other.dexterity)
    }
}

pub struct SkillsFactory;

impl SkillsFactory {
    pub fn random(&self) -> Skills {
        let mut rng = thread_rng();
        return Skills::new(rng.gen_range(1..=10), rng.gen_range(1..=10));
    }

    pub fn ninja(&self) -> Skills {
        return Skills::new(6, 9);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn eq(){
        let s1 = Skills::new(3, 3);
        let s2 = Skills::new(4, 4);

        assert_eq!(false, s1 == s2);
        assert_eq!(true, s1 != s2);
        assert_eq!(true, s1 == s1);
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

    #[test]
    fn foo(){
        for my_dex in 0..10 {
            for other_dex in 0..10 {
                println!("me: {0}, min: {1} => {2}", my_dex, other_dex, call_calculate_hit_probability(my_dex, other_dex));
            }
        }
    }

    fn call_calculate_hit_probability(player_dex: u8, min_dex: u8) -> f64 {
        let player_skills: Skills = Skills::new(0, player_dex);
        let min_skills: Skills = Skills::new(0, min_dex);

        return player_skills.calculate_hit_probability(min_skills);
    }

    #[test]
    fn ord_same_values() {
        let s1 = Skills::new(3, 3);
        let s2 = Skills::new(4, 4);

        assert_eq!(true, s1 < s2);
        assert_eq!(true, s1 <= s2);
        assert_eq!(true, s1 <= s1);
        assert_eq!(true, s1 >= s1);
    }

    #[test]
    fn ord_different_values() {
        let s1 = Skills::new(3, 4);
        let s2 = Skills::new(5, 4);

        assert_eq!(true, s1 < s2);
        assert_eq!(true, s1 <= s2);
        assert_eq!(false, s2 < s1);
        assert_eq!(false, s2 <= s1);
    }
}
