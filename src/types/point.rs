use std::fmt::Display;

use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Hash)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        let x_dif: i32 = self.x as i32 - other.x as i32;
        let y_dif: i32 = self.y as i32 - other.y as i32;

        return ((x_dif.pow(2) + y_dif.pow(2)) as f64).sqrt();
    }

    pub fn closest(&self, others: Vec<Point>) -> Point {
        let distances: Vec<(f64, Point)> = others.iter().map(|p| (p.distance_to(self), p.clone())).collect::<Vec<(f64, Point)>>();

        let mut minimal_distance: (f64, Point) = (f64::MAX, Point::default());

        for distance in distances {
            if distance.0 <= minimal_distance.0 {
                minimal_distance = distance;
            }
        }

        return minimal_distance.1;
    }

    pub fn random(bounds: Option<Point>) -> Point {
        match bounds {
            Some(b) => {
                let mut rng = thread_rng();
                let gen_x: u8 = rng.gen_range(0..b.x);
                let gen_y: u8 = rng.gen_range(0..b.y);

                Point::new(gen_x, gen_y)
            }
            None => {
                let mut rng = thread_rng();
                let gen_x: u8 = rng.gen();
                let gen_y: u8 = rng.gen();

                Point::new(gen_x, gen_y)
            }
        }
    }

    pub fn neighbors(&self, bounds: Option<Point>) -> Vec<Point>{
        match bounds {
            Some(boundary) => {
                let mut neighbors: Vec<Point> = vec![];

                if self > &boundary {
                    return vec![];
                }

                let move_north = self.y.overflowing_add(1);
                let north_neighbor = Point::new(self.x, move_north.0);
                if !move_north.1 && north_neighbor <= boundary {
                    neighbors.push(north_neighbor);
                }

                let move_east = self.x.overflowing_add(1);
                let east_neighbor = Point::new(move_east.0, self.y);
                if !move_east.1 && east_neighbor <= boundary {
                    neighbors.push(east_neighbor);
                }

                let move_south = self.y.overflowing_sub(1);
                let south_neighbor = Point::new(self.x, move_south.0);
                if !move_south.1 && south_neighbor <= boundary {
                    neighbors.push(south_neighbor);
                }

                let move_west = self.x.overflowing_sub(1);
                let west_neighbor = Point::new(move_west.0, self.y);
                if !move_west.1 && west_neighbor <= boundary {
                    neighbors.push(west_neighbor);
                }
                
                return neighbors;
            },
            None => {
                self.neighbors(Some(Point::MAX))
            },
        }
    }

    pub fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }

    pub const MAX : Point = Point { x: u8::MAX, y: u8::MAX };
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn distance() {
        let p1: Point = Point { x: 1, y: 1 };
        let p2: Point = Point { x: 2, y: 2 };

        assert_relative_eq!(1.4, p1.distance_to(&p2), max_relative = 0.02);
    }


    #[test]
    fn neighbors_center(){
        let center = Point::new(1, 1);
        let neighbors = center.neighbors(None);

        assert!(neighbors.contains(&Point::new(1, 0)));
        assert!(neighbors.contains(&Point::new(1, 2)));
        assert!(neighbors.contains(&Point::new(0, 1)));
        assert!(neighbors.contains(&Point::new(2, 1)));
    }

    #[test]
    fn neighbors_0_0(){
        let point = Point::new(0, 0);
        let neighbors: Vec<Point> = point.neighbors(None);

        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Point::new(1, 0)));
        assert!(neighbors.contains(&Point::new(0, 1)));
    }

    #[test]
    fn neighbors_max_max(){
        let point = Point::MAX;
        let neighbors: Vec<Point> = point.neighbors(None);

        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Point::new(u8::MAX, 254)));
        assert!(neighbors.contains(&Point::new(254, u8::MAX)));
    }

    #[test]
    fn neighbors_out_of_bounds(){
        let point = Point::MAX;
        let neighbors: Vec<Point> = point.neighbors(Some(Point::new(10, 10)));

        assert_eq!(neighbors.len(), 0);
    }

    #[test]
    fn neighbors_on_boundary_line(){
        let point = Point::new(10, 10);
        let boundary = Point::new(10, 12);
        let neighbors: Vec<Point> = point.neighbors(Some(boundary));

        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&Point::new(10, 9)));
        assert!(neighbors.contains(&Point::new(10, 11)));
        assert!(neighbors.contains(&Point::new(9, 10)));
    }

    #[test]
    fn neighbors_on_boundary_corner(){
        let point = Point::new(10, 10);
        let neighbors: Vec<Point> = point.neighbors(Some(point));

        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Point::new(10, 9)));
        assert!(neighbors.contains(&Point::new(9, 10)));
    }

    #[test]
    fn closest(){
        let p1 = Point::new(0, 0);
        let p2 = Point::new(4, 4);
        let p3 = Point::new(6, 5);
        let p4 = Point::new(5, 5);

        let closest = p1.closest(vec![p2, p3, p4]);

        assert_eq!(closest, p2);
    }
}
