use crate::types::point::Point;

pub struct MovementEngine {
    current_position: Point,
    enemies: Vec<Point>,
    step_limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct MovementResult {
    pub start: Point,
    pub goal: Point,
    pub last_position: Point,
    pub steps: Vec<Point>,
}

impl MovementResult {
    pub fn already_at_goal(start: Point, goal: Point) -> Self {
        MovementResult {
            start: start,
            goal: goal,
            last_position: start,
            steps: vec![],
        }
    }

    pub fn has_moved(&self) -> bool {
        self.steps.len() != 0
    }
}

impl MovementEngine {
    pub fn new(current_position: Point, enemies: Vec<Point>, step_limit: Option<usize>) -> Self {
        MovementEngine {
            current_position,
            enemies,
            step_limit,
        }
    }

    pub fn do_move(&self) -> MovementResult {
        // Determine movement target
        let movement_goal = self.current_position.closest(self.enemies.to_vec());

        if self
            .current_position
            .neighbors(None)
            .contains(&movement_goal)
            || self.current_position == movement_goal
        {
            return MovementResult::already_at_goal(self.current_position, movement_goal);
        }

        let mut path: Vec<Point> =
            MovementEngine::find_route(self.current_position.clone(), &movement_goal, vec![])[1..]
                .to_vec();
        path.truncate(self.normalized_step_limit());

        let destination_reached: Option<Point> = path.to_vec().pop();

        match destination_reached {
            Some(d) => MovementResult {
                goal: movement_goal,
                start: self.current_position,
                steps: path,
                last_position: d,
            },
            None => MovementResult {
                goal: movement_goal,
                start: self.current_position,
                steps: path,
                last_position: self.current_position,
            },
        }
    }

    fn normalized_step_limit(&self) -> usize {
        match self.step_limit {
            Some(ml) => ml,
            None => usize::MAX,
        }
    }

    fn find_route(start: Point, goal: &Point, path: Vec<Point>) -> Vec<Point> {
        let mut new_path = path.to_vec();
        new_path.push(start.clone());

        // When we and the goal have the same coordinates we have reached our goal
        if &start == goal {
            // return the path taken without the goals coordinates
            return path;
        } else {
            let next_step = goal.closest(start.neighbors(None));

            return MovementEngine::find_route(next_step, goal, new_path.to_vec());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_path() {
        let player = Point::new(3, 3);
        let enemy1 = Point::new(10, 20);
        let enemy2 = Point::default();

        let result: MovementResult = MovementEngine {
            current_position: player,
            enemies: vec![enemy1, enemy2],
            step_limit: None,
        }
        .do_move();

        assert_eq!(player, result.start);
        assert_eq!(enemy2, result.goal);
    }

    #[test]
    fn do_move_target_beyond_reach() {
        let player = Point::new(3, 3);
        let enemy1 = Point::new(10, 20);

        let result: MovementResult = MovementEngine {
            current_position: player,
            enemies: vec![enemy1],
            step_limit: Some(3),
        }
        .do_move();

        assert_eq!(player, result.start);
        assert_eq!(enemy1, result.goal);
        assert_eq!(Point::new(3, 6), result.last_position);
        assert_eq!(3, result.steps.len());
    }

    #[test]
    fn do_move_target_closer_than_range() {
        let player = Point::new(3, 3);
        let enemy1 = Point::new(10, 20);

        let result: MovementResult = MovementEngine {
            current_position: player,
            enemies: vec![enemy1],
            step_limit: Some(30),
        }
        .do_move();

        assert_eq!(player, result.start);
        assert_eq!(enemy1, result.goal);
        assert_eq!(Point::new(10, 19), result.last_position);
        assert_eq!(23, result.steps.len());
    }
}
