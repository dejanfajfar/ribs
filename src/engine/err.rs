use crate::types::point::Point;

#[derive(Debug)]
pub enum Error {
    UserAlreadyOnMap,
    LocationOccupied(Point),
    DestinationOutOfBounds(Point, Point),
    MapIdUnknown(String),
    MapLocationEmpty(Point),
    NoOpponentsPresent
}