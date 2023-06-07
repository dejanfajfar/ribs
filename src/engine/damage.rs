use std::ops::{Add, AddAssign, Mul, Sub};

/**
Defines the actual damage in hit points

# Note

Opted for a simple damage model without damage types in order to
concentrate efforts on features and to not complicate things to much

# Range

A hit can have a range of -9999 to 9999

> Negative numbers can be used to add hit points to the player
*/
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Damage {
    value: i16,
}

impl Damage {
    const MAX_VALUE: i16 = 9999;
    const MIN_VALUE: i16 = -9999;

    pub const MAX: Damage = Damage {
        value: Damage::MAX_VALUE,
    };
    pub const MIN: Damage = Damage {
        value: Damage::MIN_VALUE,
    };

    pub fn damage(&self) -> i16 {
        self.value
    }
}

impl AddAssign for Damage {
    fn add_assign(&mut self, rhs: Self) {
        let sum: (i16, bool) = self.value.overflowing_add(rhs.value);

        if sum.1 || sum.0 > Damage::MAX_VALUE {
            self.value = Damage::MAX_VALUE;
        } else if sum.0 < Damage::MIN_VALUE {
            self.value = Damage::MIN_VALUE
        } else {
            self.value = sum.0;
        }
    }
}

impl Add for Damage {
    type Output = Damage;

    fn add(self, rhs: Self) -> Self::Output {
        let sum: (i16, bool) = self.value.overflowing_add(rhs.value);

        if sum.1 || sum.0 > Damage::MAX_VALUE {
            return Damage::MAX;
        } else if sum.0 < Damage::MIN_VALUE {
            return Damage::MIN;
        } else {
            return Damage::from(sum.0);
        }
    }
}

impl Mul<i32> for Damage {
    type Output = Damage;

    fn mul(self, rhs: i32) -> Self::Output {
        let mul_dmg = i32::from(self.value) * rhs;

        return Damage::from(mul_dmg);
    }
}

impl Sub for Damage {
    type Output = Damage;

    fn sub(self, rhs: Self) -> Self::Output {
        let sub: (i16, bool) = self.value.overflowing_sub(rhs.value);

        if sub.1 || sub.0 < Damage::MIN_VALUE {
            return Damage::MIN;
        } else if sub.0 > Damage::MAX_VALUE {
            return Damage::MAX;
        } else {
            return Damage::from(sub.0);
        }
    }
}

impl From<i16> for Damage {
    fn from(value: i16) -> Self {
        if value > Damage::MAX_VALUE {
            Damage::MAX
        } else if value < Damage::MIN_VALUE {
            Damage::MIN
        } else {
            Damage { value }
        }
    }
}

impl From<f32> for Damage {
    fn from(value: f32) -> Self {
        let small_val: i32 = value as i32;

        return Damage::from(small_val);
    }
}

impl From<i32> for Damage {
    fn from(value: i32) -> Self {
        if value > i32::from(i16::MAX) {
            return Damage::from(i16::MAX);
        }
        if value < i32::from(i16::MIN) {
            return Damage::from(i16::MIN);
        }

        return Damage::from(value as i16);
    }
}

impl Default for Damage {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_in_range() {
        assert_eq!(15, Damage::from(15).value);
    }

    #[test]
    fn from_upper_bound() {
        assert_eq!(Damage::MAX_VALUE, Damage::from(Damage::MAX).value);
    }

    #[test]
    fn from_above_upper_bound() {
        assert_eq!(Damage::MAX_VALUE, Damage::from(i16::MAX).value);
    }

    #[test]
    fn from_lower_bound() {
        assert_eq!(Damage::MIN_VALUE, Damage::from(Damage::MIN).value);
    }

    #[test]
    fn from_below_lower_bound() {
        assert_eq!(Damage::MIN_VALUE, Damage::from(i16::MIN).value);
    }

    #[test]
    fn add_assign_in_bound() {
        let mut hd1 = Damage::default();
        let hd2 = Damage::from(12);
        hd1 += hd2;
        assert_eq!(12, hd1.value);
    }

    #[test]
    fn add_assign_min_min() {
        let mut hd1 = Damage::MIN;
        let hd2 = Damage::MIN;
        hd1 += hd2;
        assert_eq!(Damage::MIN_VALUE, hd1.value);
    }

    #[test]
    fn add_min_min() {
        let hd1 = Damage::MIN;
        let hd2 = Damage::MIN;
        let sum = hd1 + hd2;
        assert_eq!(Damage::MIN_VALUE, sum.value);
    }
}
