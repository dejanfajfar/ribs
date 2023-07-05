#[derive(Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: u8,
    pub y: u8
}

impl Point {
    pub fn distance_to(&self, other : &Point) -> f64 {
        let x_dif: i32 = self.x as i32 - other.x as i32;
        let y_dif: i32 = self.y as i32 - other.y as i32;

        return ((x_dif.pow(2) + y_dif.pow(2)) as f64).sqrt();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn distance(){
        let p1 : Point = Point { x: 1, y: 1 };
        let p2 : Point = Point { x: 2, y: 2 };

        assert_relative_eq!(1.4, p1.distance_to(&p2), max_relative = 0.02);
    }
}