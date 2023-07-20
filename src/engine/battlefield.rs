use std::collections::HashMap;

use crate::{
    storage::{
        battlefields::BattleFieldRecord,
        combatants::CombatantRecord,
        Record,
    },
    types::point::Point,
};

use super::err::Error;

#[derive(Debug)]
pub struct BattlefieldData {
    map: BattleFieldMap,
    combatants: Vec<CombatantRecord>,
}

#[derive(Debug, Clone)]
pub struct BattleFieldMap {
    width: u8,
    height: u8,
    combatants_positions: HashMap<String, Point>,
}

impl From<BattleFieldRecord> for BattleFieldMap {
    fn from(value: BattleFieldRecord) -> Self {
        BattleFieldMap {
            width: value.width,
            height: value.height,
            combatants_positions: HashMap::new(),
        }
    }
}

impl Clone for BattlefieldData {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            combatants: self.combatants.to_vec(),
        }
    }
}

impl BattlefieldData {
    pub fn add_combatant(&mut self, combatant: CombatantRecord) -> Result<bool, Error> {
        self.map.place_randomly(combatant.get_id())?;
        Ok(true)
    }
}

impl BattleFieldMap {
    pub fn place_randomly(&mut self, combatant_id: String) -> Result<bool, Error> {
        let combatant_position = self.unoccupied_location();

        if self.combatants_positions.contains_key(&combatant_id) {
            return Err(Error::UserAlreadyOnMap);
        }

        self.combatants_positions
            .insert(combatant_id, combatant_position);
        Ok(true)
    }

    fn unoccupied_location(&self) -> Point {
        let mut starting_position: Point = Point::random(Some(Point::new(self.height, self.width)));

        while self.is_occupied(starting_position) {
            starting_position = Point::random(Some(Point::new(self.height, self.width)));
        }

        return starting_position;
    }

    pub fn is_occupied(&self, location: Point) -> bool {
        return false;
    }

    pub fn combatant_positions(&self, active_combatant: String) -> (Point, Vec<Point>) {
        todo!()
    }

    pub fn map_bounds(&self) -> Point {
        Point { x: self.width, y: self.height }
    }
}
