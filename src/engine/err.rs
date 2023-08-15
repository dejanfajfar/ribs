use crate::types::point::Point;

#[derive(Debug)]
pub enum Error {
    UserAlreadyOnMap,
    DestinationOccupied(Point, Point),
    DestinationOutOfBounds(Point, Point),
    MapLocationEmpty(Point),
    NoOpponentsPresent
}