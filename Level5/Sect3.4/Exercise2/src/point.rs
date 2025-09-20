use crate::shape::{Shape, ShapeData};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Point {
    // Composition: Point HAS-A ShapeData (equivalent to C++ inheritance)
    shape_data: ShapeData,
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Point {
    // Constructor with parameters (calls Shape constructor equivalent)
    pub fn new(x: f64, y: f64) -> Self {
        println!("  Point::new({}, {}) - Constructor called", x, y);
        Self {
            shape_data: ShapeData::new(),  // Equivalent to calling base class constructor
            x,
            y,
        }
    }

    // Constructor with coordinates and specific ID
    pub fn new_with_id(x: f64, y: f64, id: i32) -> Self {
        println!("  Point::new_with_id({}, {}, {}) - Constructor called", x, y, id);
        Self {
            shape_data: ShapeData::with_id(id),
            x,
            y,
        }
    }

    // Default constructor
    pub fn default() -> Self {
        println!("  Point::default() - Default constructor called");
        Self::new(0.0, 0.0)
    }

    // Getters
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    // Setters
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    // Distance to another point
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    // Distance to origin
    pub fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Point-specific string representation
    pub fn point_to_string(&self) -> String {
        format!("Point({:.1}, {:.1})", self.x, self.y)
    }
}

// Implement the Shape trait for Point (equivalent to C++ inheritance)
impl Shape for Point {
    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn shape_data_mut(&mut self) -> &mut ShapeData {
        &mut self.shape_data
    }

    // Override the to_string method (equivalent to C++ virtual function override)
    fn to_string(&self) -> String {
        format!("Point({:.1}, {:.1}) [{}]", self.x, self.y, self.shape_data.to_string())
    }

    fn shape_type(&self) -> &'static str {
        "Point"
    }
}

// Implement PartialEq for Point
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
        // Note: We don't compare IDs for geometric equality
    }
}

// Implement Drop trait (equivalent to C++ destructor)
impl Drop for Point {
    fn drop(&mut self) {
        println!("  Point::drop() - Destructor called for Point({}, {})", self.x, self.y);
    }
}

// Implement Display trait for easy printing
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = <Self as Shape>::to_string(self);
        f.write_str(&s)
    }
}

// Implement Default trait
impl Default for Point {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p1 = Point::new(1.0, 2.0);
        assert_eq!(p1.x(), 1.0);
        assert_eq!(p1.y(), 2.0);
        assert!(p1.id() >= 1000 && p1.id() <= 9999);
    }

    #[test]
    fn test_point_shape_trait() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.shape_type(), "Point");
        assert!(format!("{p}").contains("Point(3.0, 4.0)"));
        assert!(format!("{p}").contains("ID:"));
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_point_clone() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = p1.clone();
        assert_eq!(p1.x(), p2.x());
        assert_eq!(p1.y(), p2.y());
        assert_eq!(p1.id(), p2.id()); // ID should be copied in clone
    }

    #[test]
    fn test_point_as_shape() {
        let p = Point::new(5.0, 10.0);
        let shape: &dyn Shape = &p;
        assert_eq!(shape.shape_type(), "Point");
        assert_eq!(shape.id(), p.id());
    }
}
