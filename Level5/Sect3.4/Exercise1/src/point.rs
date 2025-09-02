use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Point {
    // Constructor with parameters (equivalent to C++ parameterized constructor)
    pub fn new(x: f64, y: f64) -> Self {
        println!("    Point::new({}, {}) - Constructor called", x, y);
        Self { x, y }
    }

    // Default constructor
    pub fn default() -> Self {
        println!("    Point::default() - Default constructor called");
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

    // ToString function
    pub fn to_string(&self) -> String {
        format!("Point({:.1}, {:.1})", self.x, self.y)
    }
}

// Implement Clone trait (equivalent to C++ copy constructor)
impl Clone for Point {
    fn clone(&self) -> Self {
        println!("    Point::clone() - Copy constructor called for Point({}, {})", self.x, self.y);
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

// Implement Drop trait (equivalent to C++ destructor)
impl Drop for Point {
    fn drop(&mut self) {
        println!("    Point::drop() - Destructor called for Point({}, {})", self.x, self.y);
    }
}

// Implement Display trait for easy printing
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({:.1}, {:.1})", self.x, self.y)
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
        assert_eq!(p1, p2);
    }
}
