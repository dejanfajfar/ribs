use std::collections::HashMap;

use crate::types::point::Point;

use super::err::Error;

#[derive(Debug, Clone)]
pub struct Map {
    width: u8,
    height: u8,
    pois: HashMap<Point, String>,
}

impl Map {
    pub fn new(width: u8, height: u8) -> Self {
        Map {
            width,
            height,
            pois: HashMap::new(),
        }
    }

    pub fn place_randomly(&mut self, id: String) -> Result<bool, Error> {
        let position = self.unoccupied_location();

        if self.position_for(&id).is_some() {
            return Err(Error::UserAlreadyOnMap);
        }

        self.pois.insert(position, id);
        Ok(true)
    }

    pub fn place(&mut self, id: String, location: Point) -> Result<bool, Error> {
        if !self.is_occupied(location) {
            return Err(Error::LocationOccupied(location));
        }

        self.pois.insert(location, id);

        Ok(true)
    }

    pub fn move_to(&mut self, origin: Point, goal: Point) -> Result<bool, Error> {
        let pois_clone = self.pois.clone();
        let origin_id: Option<&String> = pois_clone.get(&origin);

        match origin_id {
            Some(id) => {
                // if the origin and destination are the same point the the move is successful
                if origin == goal {
                    return Ok(true);
                }

                // check that the destination is inside the map bounds
                let map_bounds: Point = self.map_bounds();
                if goal > map_bounds {
                    return Err(Error::DestinationOutOfBounds(goal, map_bounds));
                }

                // if the destination is occupied then we can not move to the desired location
                if self.is_occupied(goal) {
                    return Err(Error::LocationOccupied(goal));
                }

                self.pois.remove(&origin); 
                self.pois.insert(goal, id.clone());

                Ok(true)
            }
            None => Err(Error::MapLocationEmpty(origin)),
        }
    }

    pub fn get_occupied_neighbors(&self, location: Point) -> Vec<String>{
        // if the provided location is outside of the map bounds then we return an empty array
        if location > self.map_bounds(){
            return vec![];
        }

        let mut occupied_neighbors = vec![];
        
        for neighbor in location.neighbors(None) {
            for poi in self.pois.clone() {
                if neighbor == poi.0 {
                    occupied_neighbors.push(poi.1.clone())
                }
            }
        }

        return occupied_neighbors;
    }

    fn unoccupied_location(&self) -> Point {
        let mut starting_position: Point = Point::random(Some(Point::new(self.height, self.width)));

        // ToDo: Potential endless loop id all positions are occupied
        while self.is_occupied(starting_position) {
            starting_position = Point::random(Some(Point::new(self.height, self.width)));
        }

        return starting_position;
    }

    pub fn is_occupied(&self, _location: Point) -> bool {
        self.pois.contains_key(&_location)
    }

    pub fn position_for(&self, id: &str) -> Option<Point> {
        for poi in self.pois.clone() {
            if poi.1 == id {
                return Some(poi.0.clone());
            }
        }
        None
    }

    pub fn map_bounds(&self) -> Point {
        Point {
            x: self.width,
            y: self.height,
        }
    }
}
