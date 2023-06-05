use std::{
    clone,
    ops::{Add, AddAssign},
};

/**
Defines the actual damage in hit points

# Note

Opted for a simple damage model without damage types in order to
concentrate efforts on features and to not complicate things to much

# Range

A hit can have a range of -9999 to 9999

> Negative numbers can be used to add hit points to the player
*/
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct HitDamage {
    value: i16,
}

impl HitDamage {
    const MAX_VALUE: i16 = 9999;
    const MIN_VALUE: i16 = -9999;

    pub const MAX: HitDamage = HitDamage {
        value: HitDamage::MAX_VALUE,
    };
    pub const MIN: HitDamage = HitDamage {
        value: HitDamage::MIN_VALUE,
    };
}

impl AddAssign for HitDamage {
    fn add_assign(&mut self, rhs: Self) {
        let sum: (i16, bool) = self.value.overflowing_add(rhs.value);

        if sum.1 || sum.0 > HitDamage::MAX_VALUE {
            self.value = HitDamage::MAX_VALUE;
        } else {
            self.value = sum.0;
        }
    }
}

impl Add for HitDamage {
    type Output = HitDamage;

    fn add(self, rhs: Self) -> Self::Output {
        let sum: (i16, bool) = self.value.overflowing_add(rhs.value);

        if sum.1 || sum.0 > HitDamage::MAX_VALUE {
            return HitDamage::MAX;
        } else {
            return HitDamage { value: sum.0 };
        }
    }
}

impl From<i16> for HitDamage {
    fn from(value: i16) -> Self {
        if value > HitDamage::MAX_VALUE {
            HitDamage::MAX
        } else if value < HitDamage::MIN_VALUE {
            HitDamage::MIN
        } else {
            HitDamage { value: value }
        }
    }
}

impl Default for HitDamage {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_in_range() {
        assert_eq!(15, HitDamage::from(15).value);
    }

    #[test]
    fn from_upper_bound() {
        assert_eq!(HitDamage::MAX_VALUE, HitDamage::from(HitDamage::MAX).value);
    }

    #[test]
    fn from_above_upper_bound() {
        assert_eq!(HitDamage::MAX_VALUE, HitDamage::from(i16::MAX).value);
    }

    #[test]
    fn from_lower_bound() {
        assert_eq!(HitDamage::MIN_VALUE, HitDamage::from(HitDamage::MIN).value);
    }

    #[test]
    fn from_below_lower_bound() {
        assert_eq!(HitDamage::MIN_VALUE, HitDamage::from(i16::MIN).value);
    }

    #[test]
    fn add_assign_in_bound() {
        let mut hd1 = HitDamage::default();
        let hd2 = HitDamage::from(12);
        hd1 += hd2;
        assert_eq!(12, hd1.value);
    }

    #[ignore]
    #[test]
    fn add_assign_min_min() {
        let mut hd1 = HitDamage::MIN;
        let hd2 = HitDamage::MIN;
        hd1 += hd2;
        assert_eq!(HitDamage::MIN_VALUE, hd1.value);
    }

    #[ignore]
    #[test]
    fn add_min_min() {
        let hd1 = HitDamage::MIN;
        let hd2 = HitDamage::MIN;
        let sum = hd1 + hd2;
        assert_eq!(HitDamage::MIN_VALUE, sum.value);
    }
}
