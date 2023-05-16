use std::{
    fmt::Display,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, Eq)]
pub struct SkillPoint {
    point_value: u8,
}

impl SkillPoint {
    const MAX_VALUE: u8 = 10;
    const MIN_VALUE: u8 = 0;

    pub const MAX: SkillPoint = SkillPoint {
        point_value: SkillPoint::MAX_VALUE,
    };

    pub const MIN: SkillPoint = SkillPoint {
        point_value: SkillPoint::MIN_VALUE,
    };

    pub fn raw_value(&self) -> u8 {
        return self.point_value;
    }
}

impl Default for SkillPoint {
    fn default() -> Self {
        SkillPoint::MIN
    }
}

impl From<u8> for SkillPoint {
    fn from(value: u8) -> Self {
        if SkillPoint::MAX_VALUE >= value && value >= SkillPoint::MIN_VALUE {
            return Self { point_value: value };
        } else {
            return SkillPoint::MAX;
        }
    }
}

impl Add for SkillPoint {
    type Output = SkillPoint;

    fn add(self, rhs: Self) -> Self::Output {
        match self.point_value {
            SkillPoint::MIN_VALUE => Self::Output {
                point_value: rhs.point_value,
            },
            1..=9 => {
                let combined: u8 = self.point_value + rhs.point_value;
                if combined >= SkillPoint::MAX_VALUE {
                    return SkillPoint::MAX;
                } else {
                    return SkillPoint {
                        point_value: combined,
                    };
                }
            }
            _ => SkillPoint::MAX,
        }
    }
}

impl PartialOrd for SkillPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.point_value.partial_cmp(&other.point_value)
    }
}

impl PartialEq for SkillPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point_value == other.point_value
    }
}

impl PartialEq<u8> for SkillPoint {
    fn eq(&self, other: &u8) -> bool {
        &self.point_value == other
    }
}

impl Display for SkillPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.point_value)
    }
}

impl Sub for SkillPoint {
    type Output = SkillPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        match rhs.point_value {
            SkillPoint::MAX_VALUE => SkillPoint::MIN,
            SkillPoint::MIN_VALUE => self.clone(),
            _ => match self >= rhs {
                true => SkillPoint {
                    point_value: self.point_value - rhs.point_value,
                },
                false => SkillPoint::MIN,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(SkillPoint::MAX, SkillPoint::MAX + SkillPoint::MAX);
        assert_eq!(SkillPoint::MIN, SkillPoint::MIN + SkillPoint::MIN);
        assert_eq!(SkillPoint::MAX, SkillPoint::MAX + SkillPoint::MIN);
        assert_eq!(SkillPoint::MAX, SkillPoint::MIN + SkillPoint::MAX); // reversible
        assert_eq!(SkillPoint::MAX, SkillPoint::from(6) + SkillPoint::from(6));
        assert_eq!(
            SkillPoint::from(6),
            SkillPoint::from(3) + SkillPoint::from(3)
        );
        assert_eq!(SkillPoint::from(3), SkillPoint::from(3) + SkillPoint::MIN);
        assert_eq!(SkillPoint::from(3), SkillPoint::MIN + SkillPoint::from(3));
    }

    #[test]
    fn sub() {
        assert_eq!(SkillPoint::MIN, SkillPoint::MAX - SkillPoint::MAX);
        assert_eq!(SkillPoint::MAX, SkillPoint::MAX - SkillPoint::MIN);
        assert_eq!(
            SkillPoint::from(3),
            SkillPoint::from(6) - SkillPoint::from(3)
        );
        assert_eq!(SkillPoint::MIN, SkillPoint::from(3) - SkillPoint::from(6));
    }

    #[test]
    fn from_min_value() {
        assert_eq!(SkillPoint::MIN, SkillPoint::from(u8::MIN));
    }

    #[test]
    fn from_max_value() {
        assert_eq!(SkillPoint::MAX, SkillPoint::from(u8::MAX));
    }

    #[test]
    fn from_upper_bound() {
        assert_eq!(SkillPoint::MAX, SkillPoint::from(SkillPoint::MAX_VALUE));
    }

    #[test]
    fn eq_pass() {
        let s1: SkillPoint = SkillPoint::from(3);
        let s2: SkillPoint = SkillPoint::from(3);

        assert_eq!(true, s1 == s2);
        assert_eq!(false, s1 != s2);
    }

    #[test]
    fn eq_with_u8() {
        let s1: SkillPoint = SkillPoint::from(3);

        assert_eq!(true, s1 == 3);
        assert_eq!(false, s1 == u8::MAX);
    }

    #[test]
    fn eq_fail() {
        let s1: SkillPoint = SkillPoint::from(3);
        let s2: SkillPoint = SkillPoint::from(4);

        assert_eq!(false, s1 == s2);
        assert_eq!(true, s1 != s2);
    }

    #[test]
    fn eq_max() {
        assert_eq!(true, SkillPoint::MAX == SkillPoint::MAX);
        assert_eq!(true, SkillPoint::MAX != SkillPoint::MIN);
        assert_eq!(false, SkillPoint::MAX == SkillPoint::MIN);
    }

    #[test]
    fn eq_min() {
        assert_eq!(true, SkillPoint::MIN == SkillPoint::MIN);
        assert_eq!(true, SkillPoint::MAX != SkillPoint::MIN);
    }

    #[test]
    fn ord() {
        let s1: SkillPoint = SkillPoint::from(3);
        let s2: SkillPoint = SkillPoint::from(4);

        assert_eq!(true, s1 < s2);
        assert_eq!(true, s1 <= s1);
    }
}
