use crate::types::point::Point;

const MOVEMENT_RANGE : u8 = 3;

pub struct MovementEngine;

impl MovementEngine {
    pub fn do_move(movement_origin: Point, other_players: Vec<Point>) -> Point {
        // Determine movement target
        let  movement_target = movement_origin.closest(other_players);

        let path = MovementEngine::calculate_path(&movement_origin, &movement_target);

        return Point::default();
    }

    fn calculate_path(start: &Point, goal: &Point) -> Vec<Point> {
        if start == goal {
            return vec![];
        }

        return MovementEngine::find_route(start.clone(), &goal, vec![])[1..].to_vec();
    }

    fn find_route(start: Point, goal: &Point, path: Vec<Point>) -> Vec<Point>{
        let mut new_path = path.to_vec();
        new_path.push(start.clone());

        // When we and the goal have the same coordinates we have reached our goal
        if &start == goal {
            // return the path taken without the goals coordinates
            return path;
        }
        else {
            let next_step = goal.closest(start.neighbors(None));

            return MovementEngine::find_route(next_step, goal, new_path.to_vec());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_path(){
        let result: Vec<Point> = MovementEngine::calculate_path(&Point::new(1, 1), &Point::new(3, 3));

        assert_eq!(result.len(), 3);
    }
}