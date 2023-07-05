#[derive(Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: u8,
    pub y: u8
}

impl Point {
    pub fn distance_to(&self, other : &Point) -> f64 {
        let x_dif = self.x - other.x;
        let y_dif = self.y - other.y;

        return (((x_dif as u32).pow(2) + (y_dif as u32).pow(2)) as f64).sqrt();
    }
}