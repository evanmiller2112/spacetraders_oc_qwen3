//! Distance tracking module for calculating distances between locations in SpaceTraders

use serde::{Deserialize, Serialize};

/// Structure to represent a 2D coordinate point
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new point with given coordinates
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Calculates the Euclidean distance to another point
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculates the Manhattan distance to another point
    pub fn manhattan_distance_to(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// Trait for objects that have a location
pub trait HasLocation {
    /// Returns the location of this object as a Point
    fn get_location(&self) -> Point;
}

/// Structure to represent a location with symbol and coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub symbol: String,
    pub point: Point,
}

impl Location {
    /// Creates a new location with symbol and coordinates
    pub fn new(symbol: String, x: i32, y: i32) -> Self {
        Location {
            symbol,
            point: Point::new(x, y),
        }
    }

    /// Calculates the distance to another location
    pub fn distance_to(&self, other: &Location) -> f64 {
        self.point.distance_to(&other.point)
    }
}

/// Structure to represent a system with its coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub symbol: String,
    pub point: Point,
}

impl System {
    /// Creates a new system with symbol and coordinates
    pub fn new(symbol: String, x: i32, y: i32) -> Self {
        System {
            symbol,
            point: Point::new(x, y),
        }
    }

    /// Calculates the distance to another system
    pub fn distance_to(&self, other: &System) -> f64 {
        self.point.distance_to(&other.point)
    }
}

/// Structure to represent a waypoint with its coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    pub symbol: String,
    pub point: Point,
}

impl Waypoint {
    /// Creates a new waypoint with symbol and coordinates
    pub fn new(symbol: String, x: i32, y: i32) -> Self {
        Waypoint {
            symbol,
            point: Point::new(x, y),
        }
    }

    /// Calculates the distance to another waypoint
    pub fn distance_to(&self, other: &Waypoint) -> f64 {
        self.point.distance_to(&other.point)
    }
}

/// Structure to represent a ship with its location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub symbol: String,
    pub point: Point,
}

impl Ship {
    /// Creates a new ship with symbol and coordinates
    pub fn new(symbol: String, x: i32, y: i32) -> Self {
        Ship {
            symbol,
            point: Point::new(x, y),
        }
    }

    /// Calculates the distance to another ship
    pub fn distance_to(&self, other: &Ship) -> f64 {
        self.point.distance_to(&other.point)
    }
}

/// Structure to represent an asteroid with its location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asteroid {
    pub symbol: String,
    pub point: Point,
}

impl Asteroid {
    /// Creates a new asteroid with symbol and coordinates
    pub fn new(symbol: String, x: i32, y: i32) -> Self {
        Asteroid {
            symbol,
            point: Point::new(x, y),
        }
    }

    /// Calculates the distance to another asteroid
    pub fn distance_to(&self, other: &Asteroid) -> f64 {
        self.point.distance_to(&other.point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 4);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_point_manhattan_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 4);
        assert_eq!(p1.manhattan_distance_to(&p2), 7);
    }

    #[test]
    fn test_location_distance() {
        let loc1 = Location::new("LOC1".to_string(), 0, 0);
        let loc2 = Location::new("LOC2".to_string(), 3, 4);
        assert_eq!(loc1.distance_to(&loc2), 5.0);
    }

    #[test]
    fn test_system_distance() {
        let sys1 = System::new("SYS1".to_string(), 0, 0);
        let sys2 = System::new("SYS2".to_string(), 3, 4);
        assert_eq!(sys1.distance_to(&sys2), 5.0);
    }

    #[test]
    fn test_waypoint_distance() {
        let wp1 = Waypoint::new("WP1".to_string(), 0, 0);
        let wp2 = Waypoint::new("WP2".to_string(), 3, 4);
        assert_eq!(wp1.distance_to(&wp2), 5.0);
    }

    #[test]
    fn test_ship_distance() {
        let ship1 = Ship::new("SHIP1".to_string(), 0, 0);
        let ship2 = Ship::new("SHIP2".to_string(), 3, 4);
        assert_eq!(ship1.distance_to(&ship2), 5.0);
    }

    #[test]
    fn test_asteroid_distance() {
        let ast1 = Asteroid::new("AST1".to_string(), 0, 0);
        let ast2 = Asteroid::new("AST2".to_string(), 3, 4);
        assert_eq!(ast1.distance_to(&ast2), 5.0);
    }
}