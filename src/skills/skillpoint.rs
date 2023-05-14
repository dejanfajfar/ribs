use std::ops::Add;

#[derive(Debug, Clone)]
struct SkillPoint {
    point_value: u8
}

impl SkillPoint{

    const MAX_VALUE: u8 = 10;
    const MIN_VALUE: u8 = 0;

    pub const MAX: SkillPoint = SkillPoint {
        point_value: SkillPoint::MAX_VALUE
    };

    pub const MIN: SkillPoint = SkillPoint {
        point_value: SkillPoint::MIN_VALUE
    };
}

impl From<u8> for SkillPoint {
    fn from(value: u8) -> Self {
        if SkillPoint::MAX_VALUE >= value && value >= SkillPoint::MIN_VALUE {
            return Self{
                point_value: value
            };
        }
        else {
            return Self {
                point_value: SkillPoint::MIN_VALUE
            };
        }
    }
}

impl Add for SkillPoint{
    type Output = SkillPoint;

    fn add(self, rhs: Self) -> Self::Output {
        match self.point_value {
            0 => Self::Output {
                point_value: rhs.point_value
            },
            1..=9 => {
                SkillPoint::MAX
            },
            10 => Self::Output {
                point_value: SkillPoint::MAX_VALUE
            },
            _ => SkillPoint::MAX
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_self_0_then_other_taken(){
        for other_points_value in 0..10 {
            let test_result: SkillPoint = SkillPoint::MIN + SkillPoint {
                point_value: other_points_value
            };

            assert_eq!(test_result.point_value, other_points_value)
        }
    }

    #[test]
    fn add_self_10_then_always_10(){
        for other_points_value in 0..10 {
            let test_result: SkillPoint = SkillPoint::MAX + SkillPoint {
                point_value: other_points_value
            };

            assert_eq!(test_result.point_value, SkillPoint::MAX_VALUE);
        }
    }
}