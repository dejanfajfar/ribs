use std::{clone, ops::AddAssign};

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
struct HitDamage{
    value: i16
}

impl HitDamage {
    pub const MAX : i16 = 9999;
    pub const MIN : i16 = -9999;
}

impl AddAssign for HitDamage {
    fn add_assign(&mut self, rhs: Self) {
        let sum: (i16, bool) = self.value.overflowing_add(rhs.value);

        if sum.1 || sum.0 > HitDamage::MAX {
            self.value = HitDamage::MAX;
        }
        else {
            self.value = sum.0;
        }

    }
}

impl From<i16> for HitDamage{
    fn from(value: i16) -> Self {
        if value > HitDamage::MAX {
            HitDamage { value: HitDamage::MAX}
        }
        else if value < HitDamage::MIN {
            HitDamage {value: HitDamage::MIN}
        }
        else {
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
mod test {
    use super::*;

    #[test]
    fn from_in_range(){
        let test_object: HitDamage = HitDamage::from(15);

        assert_eq!(15, test_object.value);
    }

    
}